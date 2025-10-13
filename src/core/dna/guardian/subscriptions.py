from typing import Set, Callable, Optional, Dict, Any


class DNASubscription:
	"""Represents a subscription to DNA updates for a component."""

	def __init__(self, subscriber_id: str, component_name: str, cdna_blocks: Optional[Set[str]] = None, adna_sections: Optional[Set[str]] = None):
		self.subscriber_id = subscriber_id
		self.component_name = component_name
		self.cdna_blocks = set(cdna_blocks or [])
		self.adna_sections = set(adna_sections or [])
		self.callback: Optional[Callable[..., None]] = None
		self.last_notification: float = 0.0

	def is_interested_in_event(self, event) -> bool:
		"""Return True if the subscription is interested in the given DNAEvent."""
		try:
			# If event affects components and this subscription's component matches
			if self.component_name in event.affected_components:
				return True

			# If subscriber specified cdna_blocks, check overlap
			if self.cdna_blocks and hasattr(event, 'changed_data'):
				# can't inspect binary changed_data easily here; assume interest
				return True

			# ADNA: check metadata key
			key = event.metadata.get('key') if hasattr(event, 'metadata') else None
			if key and self.adna_sections:
				return any(sec in key for sec in self.adna_sections)

			return False
		except Exception:
			return False
