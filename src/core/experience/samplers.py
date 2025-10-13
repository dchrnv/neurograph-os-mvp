import random
from typing import List, Tuple
from enum import Enum

from .event import ExperienceEvent


class SamplingStrategy(Enum):
	UNIFORM = 'uniform'
	PRIORITIZED = 'prioritized'
	RECENT = 'recent'
	DIVERSE = 'diverse'


class ExperienceSampler:
	"""Стратегии сэмплирования опыта"""

	def __init__(self, alpha: float = 0.6, beta: float = 0.4):
		self.alpha = float(alpha)
		self.beta = float(beta)

	def sample_uniform(self, events: List[ExperienceEvent], batch_size: int) -> List[ExperienceEvent]:
		if len(events) <= batch_size:
			return events.copy()
		return random.sample(events, batch_size)

	def sample_prioritized(self, events: List[ExperienceEvent], batch_size: int) -> Tuple[List[ExperienceEvent], List[float]]:
		# compute priorities
		priorities = []
		for e in events:
			p = 0.0
			try:
				p = float(getattr(e, 'priority', 0.0) or 0.0)
			except Exception:
				p = 0.0
			if p == 0.0:
				# try metadata priority
				try:
					meta = getattr(e, 'metadata', None)
					if isinstance(meta, dict) and 'priority' in meta:
						p = float(meta['priority'])
				except Exception:
					pass
				# fallback to reward magnitude
				if p == 0.0:
					try:
						p = abs(float(e.reward)) if e.reward is not None else 0.1
					except Exception:
						p = 0.1
			priorities.append(max(p, 1e-6) ** self.alpha)

		total = sum(priorities)
		if total == 0:
			# fallback uniform
			sampled = self.sample_uniform(events, batch_size)
			return sampled, [1.0] * len(sampled)

		probs = [p / total for p in priorities]
		indices = random.choices(range(len(events)), weights=probs, k=batch_size)
		sampled = [events[i] for i in indices]

		# importance sampling weights
		weights = []
		N = len(events)
		for idx in indices:
			w = (N * probs[idx]) ** (-self.beta)
			weights.append(w)

		# normalize weights
		maxw = max(weights) if weights else 1.0
		weights = [w / maxw for w in weights]

		return sampled, weights

	def sample_recent(self, events: List[ExperienceEvent], batch_size: int, window_size: int = 10000) -> List[ExperienceEvent]:
		recent = events[-window_size:] if len(events) > window_size else events
		return self.sample_uniform(recent, batch_size)

	def sample_diverse(self, events: List[ExperienceEvent], batch_size: int) -> List[ExperienceEvent]:
		if len(events) <= batch_size:
			return events.copy()
		sorted_events = sorted(events, key=lambda e: getattr(e, 'timestamp', 0))
		step = max(1, len(sorted_events) // batch_size)
		result = []
		for i in range(batch_size):
			idx = min(i * step, len(sorted_events) - 1)
			result.append(sorted_events[idx])
		return result

