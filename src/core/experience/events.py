from dataclasses import dataclass, field
from typing import List, Optional, Any, Dict, Tuple
import time


@dataclass
class EventRecord:
    event_id: str
    event_type: str
    timestamp: float
    source_component: str
    data: Dict[str, Any] = field(default_factory=dict)
    state: Optional[List[float]] = None
    action: Optional[List[float]] = None
    reward: Optional[float] = None
    next_state: Optional[List[float]] = None
    done: bool = False
    priority: float = 0.0
    caused_by: Optional[str] = None
    leads_to: List[str] = field(default_factory=list)


@dataclass
class ExperienceTrajectory:
    trajectory_id: str
    events: List[EventRecord] = field(default_factory=list)
    total_reward: float = 0.0
    trajectory_length: int = 0
    start_timestamp: float = 0.0
    end_timestamp: float = 0.0
    outcome: str = 'unknown'
    quality_score: float = 0.0
    initial_state: Optional[List[float]] = None
    final_state: Optional[List[float]] = None
    adna_generation: Optional[int] = None

    def add_event(self, event: EventRecord) -> None:
        if not self.events:
            self.start_timestamp = event.timestamp
            if event.state is not None:
                self.initial_state = event.state.copy()
        self.events.append(event)
        self.trajectory_length += 1
        self.end_timestamp = event.timestamp
        if event.reward is not None:
            self.total_reward += event.reward
        if event.next_state is not None:
            self.final_state = event.next_state.copy()


@dataclass
class ExperienceBatch:
    batch_id: str
    events: List[EventRecord] = field(default_factory=list)
    trajectories: List[ExperienceTrajectory] = field(default_factory=list)
    batch_size: int = 0
    avg_reward: float = 0.0
    success_rate: float = 0.0
    diversity_score: float = 0.0
    time_window: Optional[Tuple[float, float]] = None
    sampling_strategy: str = 'uniform'
    weights: Optional[List[float]] = None

    def compute_stats(self) -> None:
        self.batch_size = len(self.events)
        rewards = [e.reward for e in self.events if e.reward is not None]
        if rewards:
            self.avg_reward = sum(rewards) / len(rewards)
        else:
            self.avg_reward = 0.0
