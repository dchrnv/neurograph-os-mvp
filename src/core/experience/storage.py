import time
import threading
from typing import List, Optional
from collections import deque

from .event import ExperienceEvent


class CircularBuffer:
	"""Кольцевой буфер для горячего опыта"""

	def __init__(self, capacity: int = 100000):
		self.capacity = int(capacity)
		self._buffer = deque(maxlen=self.capacity)
		self._lock = threading.RLock()

	def append(self, event: ExperienceEvent) -> None:
		with self._lock:
			self._buffer.append(event)

	def get_latest(self, count: int) -> List[ExperienceEvent]:
		with self._lock:
			if count <= 0:
				return []
			return list(self._buffer)[-count:]

	def get_all(self) -> List[ExperienceEvent]:
		with self._lock:
			return list(self._buffer)

	def get_range(self, start_idx: int, end_idx: int) -> List[ExperienceEvent]:
		with self._lock:
			return list(self._buffer)[start_idx:end_idx]

	def __len__(self) -> int:
		return len(self._buffer)


class SlidingWindow:
	"""Скользящее окно для среднесрочного опыта"""

	def __init__(self, max_events: int = 1000000, duration_seconds: float = 86400.0):
		self.max_events = int(max_events)
		self.duration_seconds = float(duration_seconds)
		self._events: List[ExperienceEvent] = []
		self._lock = threading.RLock()

	def append(self, event: ExperienceEvent) -> None:
		with self._lock:
			self._events.append(event)
			self._cleanup()

	def _cleanup(self) -> None:
		current_time = time.time()
		cutoff_time = current_time - self.duration_seconds

		# remove old by time
		self._events = [e for e in self._events if e.timestamp >= cutoff_time]

		# truncate by count
		if len(self._events) > self.max_events:
			self._events = self._events[-self.max_events:]

	def get_events_in_window(self, start_time: float, end_time: float) -> List[ExperienceEvent]:
		with self._lock:
			return [e for e in self._events if start_time <= e.timestamp <= end_time]

	def get_all(self) -> List[ExperienceEvent]:
		with self._lock:
			return list(self._events)

	def __len__(self) -> int:
		return len(self._events)

