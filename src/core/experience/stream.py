from typing import Any, List, Optional, Callable, Dict, Tuple
import asyncio
import json
import os
import dataclasses
import time
import random
import threading
from dataclasses import dataclass, field, asdict

from .event import ExperienceEvent as PydanticExperienceEvent
from .events import EventRecord, ExperienceTrajectory, ExperienceBatch
from .storage import CircularBuffer, SlidingWindow
from .samplers import ExperienceSampler, SamplingStrategy
import httpx
from .config import load_experience_config


@dataclass
class ExperienceStreamConfig:
    batch_size: int = 16
    flush_interval: float = 1.0  # seconds
    backend: str = "console"  # options: console, file, http
    file_path: Optional[str] = None
    http_endpoint: Optional[str] = None
    default_strategy: Optional[str] = None


class ExperienceStream:
    """Гибкий поток опыта.

    Поддерживает:
    - синхронную запись в локальный буфер (`write_event`)
    - опциональную асинхронную отправку батчей (`flush` / background task)
    - бэкенды: console (print), file (append JSONL), http (POST JSON batch)
    """

    def __init__(self, config: Optional[ExperienceStreamConfig] = None):
        # config can be either ExperienceStreamConfig or path/dict loaded from YAML/JSON
        if isinstance(config, ExperienceStreamConfig) or config is None:
            self.config = config or ExperienceStreamConfig()
        elif isinstance(config, str):
            cfg = load_experience_config(config)
            self.config = ExperienceStreamConfig()
            # map known top-level keys
            stream_cfg = cfg.get('experience', {}).get('stream', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('stream', {})
            if stream_cfg:
                self.config.batch_size = stream_cfg.get('batch_size', self.config.batch_size)
                self.config.flush_interval = stream_cfg.get('flush_interval', self.config.flush_interval)
                self.config.backend = stream_cfg.get('backend', self.config.backend)
                self.config.file_path = stream_cfg.get('file_path', self.config.file_path)
                self.config.http_endpoint = stream_cfg.get('http_endpoint', self.config.http_endpoint)
            # storage & sampling defaults
            storage_cfg = cfg.get('experience', {}).get('storage', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('storage', {})
            sampling_cfg = cfg.get('experience', {}).get('sampling', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('sampling', {})
            self._storage_cfg: Dict[str, Any] = storage_cfg or {}
            self._sampling_cfg: Dict[str, Any] = sampling_cfg or {}
        elif isinstance(config, dict):
            cfg = config
            self.config = ExperienceStreamConfig()
            stream_cfg = cfg.get('experience', {}).get('stream', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('stream', {})
            if stream_cfg:
                self.config.batch_size = stream_cfg.get('batch_size', self.config.batch_size)
                self.config.flush_interval = stream_cfg.get('flush_interval', self.config.flush_interval)
                self.config.backend = stream_cfg.get('backend', self.config.backend)
                self.config.file_path = stream_cfg.get('file_path', self.config.file_path)
                self.config.http_endpoint = stream_cfg.get('http_endpoint', self.config.http_endpoint)
            # storage & sampling defaults
            storage_cfg = cfg.get('experience', {}).get('storage', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('storage', {})
            sampling_cfg = cfg.get('experience', {}).get('sampling', {}) if isinstance(cfg.get('experience'), dict) else cfg.get('sampling', {})
            self._storage_cfg: Dict[str, Any] = storage_cfg or {}
            self._sampling_cfg: Dict[str, Any] = sampling_cfg or {}
        else:
            # unknown type -> use defaults
            self.config = ExperienceStreamConfig()
            self._storage_cfg: Dict[str, Any] = {}
            self._sampling_cfg: Dict[str, Any] = {}
        # in-memory staging buffer for immediate flush
        self._buffer: List[Any] = []
        self._lock = asyncio.Lock()
        self._bg_task: Optional[asyncio.Task] = None
        self._stopping = False

        # Storage backends: prioritize explicit storage_cfg provided via YAML/dict
        circ_size = int(self._storage_cfg.get('circular_buffer_size', self.config.batch_size * 100))
        self._circular_buffer = CircularBuffer(circ_size)
        # sliding window size and duration can be configured via config if provided
        sliding_size = int(self._storage_cfg.get('sliding_window_size', 1000000))
        sliding_hours = float(self._storage_cfg.get('sliding_window_duration_hours', 24))
        self._sliding_window = SlidingWindow(sliding_size, int(sliding_hours * 3600))

        # sampler: apply alpha/beta if provided
        alpha = float(self._sampling_cfg.get('prioritized_alpha', 0.6))
        beta = float(self._sampling_cfg.get('prioritized_beta', 0.4))
        self._sampler = ExperienceSampler(alpha=alpha, beta=beta)
        # default strategy name available on stream config for sample_batch fallback
        try:
            self.config.default_strategy = self._sampling_cfg.get('default_strategy', None)
        except Exception:
            # best-effort: ignore if config dataclass doesn't accept extra attr
            pass

        # trajectories
        self._active_trajectories: Dict[str, ExperienceTrajectory] = {}
        self._completed_trajectories: List[ExperienceTrajectory] = []
        self._trajectory_lock = threading.RLock()

        # causality graph
        self._causality_graph: Dict[str, List[str]] = {}

        # If file backend requested but no path provided — use default
        if self.config.backend == 'file' and not self.config.file_path:
            self.config.file_path = os.path.join(os.getcwd(), 'experience_events.jsonl')

        # Start background flusher only if running inside an event loop
        try:
            loop = asyncio.get_running_loop()
            # schedule periodic flush
            self._bg_task = loop.create_task(self._periodic_flush())
        except RuntimeError:
            self._bg_task = None

    def write_event(self, event: Any) -> None:
        """Записать событие в буфер (sync-safe)."""
        try:
            # normalize/convert event to internal EventRecord where possible
            rec = self._to_event_record(event)
            # store in hot storage
            self._circular_buffer.append(rec)
            self._sliding_window.append(rec)
            # keep staging buffer for flush/backends (store original for payload)
            self._buffer.append(event)
            if len(self._buffer) >= self.config.batch_size:
                # Best-effort scheduling of flush: if an event loop is running, schedule an async task;
                # otherwise run flush synchronously. Avoid calling asyncio.run when already inside an event loop.
                try:
                    loop = asyncio.get_running_loop()
                except RuntimeError:
                    # No running loop -> safe to run
                    try:
                        asyncio.run(self.flush())
                    except Exception:
                        # don't raise from library
                        pass
                else:
                    # schedule background flush
                    try:
                        loop.create_task(self.flush())
                    except Exception:
                        pass
        except Exception:
            # Never raise from the stream
            pass

    async def _periodic_flush(self) -> None:
        while not self._stopping:
            await asyncio.sleep(self.config.flush_interval)
            try:
                await self.flush()
            except Exception:
                pass

    async def flush(self) -> None:
        """Отправить накопленные события согласно конфигурации бэкенда."""
        async with self._lock:
            if not self._buffer:
                return
            batch = list(self._buffer)
            self._buffer.clear()

        # normalize events to primitive dicts
        payload = []
        for ev in batch:
            try:
                payload.append(self._event_to_payload(ev))
            except Exception:
                try:
                    payload.append({'raw': str(ev)})
                except Exception:
                    payload.append({'raw': '<unserializable>'})

        # Dispatch according to backend
        if self.config.backend == 'console':
            for item in payload:
                print(json.dumps(item))
        elif self.config.backend == 'file':
            try:
                path = self.config.file_path
                if path:
                    with open(path, 'a', encoding='utf-8') as f:
                        for item in payload:
                            f.write(json.dumps(item, ensure_ascii=False) + '\n')
            except Exception:
                pass
        elif self.config.backend == 'http':
            try:
                if not self.config.http_endpoint:
                    return
                async with httpx.AsyncClient(timeout=5.0) as client:
                    await client.post(self.config.http_endpoint, json=payload)
            except Exception:
                pass

    def get_buffer(self) -> List[Any]:
        return list(self._buffer)

    # === Converters and helpers ===
    def _to_event_record(self, ev: Any) -> EventRecord:
        """Convert various event representations to internal EventRecord dataclass."""
        # Already an EventRecord
        if isinstance(ev, EventRecord):
            return ev

        # Pydantic model (ExperienceEvent)
        if isinstance(ev, PydanticExperienceEvent):
            data = ev.model_dump() if hasattr(ev, 'model_dump') else ev.dict()
            return EventRecord(
                event_id=data.get('event_id'),
                event_type=data.get('event_type'),
                timestamp=data.get('timestamp', 0.0),
                source_component=data.get('source_component', ''),
                data=data.get('data', {}),
                state=data.get('state'),
                action=data.get('action'),
                reward=data.get('reward'),
                next_state=data.get('next_state'),
                done=data.get('done', False),
                priority=data.get('priority', 0.0),
                caused_by=data.get('caused_by'),
                leads_to=data.get('leads_to', [])
            )

        # dict-like
        if isinstance(ev, dict):
            return EventRecord(
                event_id=ev.get('event_id', str(hash(json.dumps(ev, default=str)))),
                event_type=ev.get('event_type', 'unknown'),
                timestamp=ev.get('timestamp', time.time()),
                source_component=ev.get('source_component', ''),
                data=ev.get('data', {}),
                state=ev.get('state'),
                action=ev.get('action'),
                reward=ev.get('reward'),
                next_state=ev.get('next_state'),
                done=ev.get('done', False),
                priority=ev.get('priority', 0.0),
                caused_by=ev.get('caused_by'),
                leads_to=ev.get('leads_to', [])
            )

        # Fallback: wrap generic object
        try:
            return EventRecord(event_id=str(id(ev)), event_type=type(ev).__name__, timestamp=time.time(), source_component='unknown', data={'repr': str(ev)})
        except Exception:
            return EventRecord(event_id=str(time.time()), event_type='unknown', timestamp=time.time(), source_component='unknown')

    def _event_to_payload(self, ev: Any) -> Dict[str, Any]:
        """Create JSON-serializable payload for backend flush from various event types."""
        # If pydantic -> model_dump/dict
        if isinstance(ev, PydanticExperienceEvent):
            return ev.model_dump() if hasattr(ev, 'model_dump') else ev.dict()
        # dataclass
        if dataclasses.is_dataclass(ev):
            return asdict(ev)
        # dict
        if isinstance(ev, dict):
            return ev
        # try generic
        try:
            return json.loads(json.dumps(ev, default=lambda o: getattr(o, '__dict__', str(o))))
        except Exception:
            return {'raw': str(ev)}

    # === Reading / sampling ===
    def get_latest_events(self, count: int) -> List[EventRecord]:
        return self._circular_buffer.get_latest(count)

    def sample_batch(self, batch_size: int, strategy: Optional[str] = None) -> ExperienceBatch:
        if strategy is None:
            strategy = self.config.backend if getattr(self.config, 'default_strategy', None) is None else getattr(self.config, 'default_strategy')

        all_events = self._sliding_window.get_all()
        # RL events: have state and action
        rl_events = [e for e in all_events if getattr(e, 'state', None) is not None and getattr(e, 'action', None) is not None]

        batch = ExperienceBatch(batch_id=f"batch_{int(time.time() * 1000)}", sampling_strategy=strategy)
        if not rl_events:
            return batch

        weights = None
        if strategy == SamplingStrategy.UNIFORM.value:
            sampled = self._sampler.sample_uniform(rl_events, batch_size)
        elif strategy == SamplingStrategy.PRIORITIZED.value:
            sampled, weights = self._sampler.sample_prioritized(rl_events, batch_size)
        elif strategy == SamplingStrategy.RECENT.value:
            sampled = self._sampler.sample_recent(rl_events, batch_size)
        elif strategy == SamplingStrategy.DIVERSE.value:
            sampled = self._sampler.sample_diverse(rl_events, batch_size)
        else:
            sampled = self._sampler.sample_uniform(rl_events, batch_size)

        batch.events = sampled
        batch.batch_size = len(sampled)
        batch.weights = weights
        batch.compute_stats()
        return batch

    def calculate_statistics(self, time_window: Optional[Tuple[float, float]] = None) -> Dict[str, Any]:
        if time_window:
            events = self.get_events_in_window(time_window[0], time_window[1])
        else:
            events = self._sliding_window.get_all()

        if not events:
            return {'total_events': 0, 'events_with_reward': 0, 'avg_reward': 0.0, 'event_types': {}, 'components': {}}

        stats = {
            'total_events': len(events),
            'time_span_seconds': events[-1].timestamp - events[0].timestamp if len(events) > 1 else 0,
            'events_with_reward': sum(1 for e in events if e.reward is not None),
            'events_with_state': sum(1 for e in events if e.state is not None)
        }

        rewards = [e.reward for e in events if e.reward is not None]
        if rewards:
            stats['avg_reward'] = sum(rewards) / len(rewards)
            stats['max_reward'] = max(rewards)
            stats['min_reward'] = min(rewards)
            stats['total_reward'] = sum(rewards)
        else:
            stats['avg_reward'] = 0.0
            stats['max_reward'] = 0.0
            stats['min_reward'] = 0.0
            stats['total_reward'] = 0.0

        event_types = {}
        components = {}
        for event in events:
            event_types[event.event_type] = event_types.get(event.event_type, 0) + 1
            components[event.source_component] = components.get(event.source_component, 0) + 1

        stats['event_types'] = event_types
        stats['components'] = components
        with self._trajectory_lock:
            stats['active_trajectories'] = len(self._active_trajectories)
            stats['completed_trajectories'] = len(self._completed_trajectories)

        return stats

    def get_events_in_window(self, start_time: float, end_time: float) -> List[EventRecord]:
        return self._sliding_window.get_events_in_window(start_time, end_time)

    # === Trajectory management ===
    def begin_trajectory(self, trajectory_id: Optional[str] = None) -> str:
        if trajectory_id is None:
            trajectory_id = f"traj_{int(time.time() * 1000)}_{random.randint(1000,9999)}"
        traj = ExperienceTrajectory(trajectory_id=trajectory_id)
        # thread-safe insertion
        with self._trajectory_lock:
            self._active_trajectories[trajectory_id] = traj
        return trajectory_id

    def add_to_trajectory(self, trajectory_id: str, event: EventRecord) -> bool:
        if trajectory_id not in self._active_trajectories:
            return False
        self._active_trajectories[trajectory_id].add_event(event)
        return True

    def end_trajectory(self, trajectory_id: str, outcome: str = 'unknown') -> bool:
        if trajectory_id not in self._active_trajectories:
            return False
        traj = self._active_trajectories.pop(trajectory_id)
        traj.outcome = outcome
        if traj.total_reward != 0:
            traj.quality_score = traj.total_reward / max(traj.trajectory_length, 1)
        self._completed_trajectories.append(traj)
        return True


    async def close(self) -> None:
        self._stopping = True
        if self._bg_task:
            try:
                await self._bg_task
            except Exception:
                pass
        await self.flush()
