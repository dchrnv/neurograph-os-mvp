import time
from typing import Set, Dict, Any, Optional


class DNAEvent:
	"""Represents a CDNA/ADNA change event."""

	def __init__(self, event_type: str, affected_components: Set[str], changed_data: bytes, metadata: Optional[Dict[str, Any]] = None):
		self.event_id = f"dna_{int(time.time() * 1000)}"
		self.event_type = event_type
		self.affected_components = set(affected_components or [])
		self.changed_data = changed_data
		self.metadata = metadata or {}
		self.timestamp = time.time()
