"""
Базовый класс для компонентов, интегрированных с DNAGuardian.
"""

from typing import Optional

from ..guardian.subscriptions import DNASubscription


class DNAIntegratedComponent:
    """Компонент, подключённый к DNAGuardian."""

    def __init__(self, name: str, guardian):
        self.name = name
        self.dna_guardian = guardian
        self._subscription: Optional[DNASubscription] = None
        if guardian:
            # create subscription object and register
            sub = DNASubscription(subscriber_id=f"{name}_{id(self)}", component_name=name)
            sub.callback = self._handle_dna_event
            guardian.subscribe(sub)
            self._subscription = sub

    def _handle_dna_event(self, event):
        """Распределение событий по методам on_cdna_updated / on_adna_updated."""
        if event["type"] == "cdna_update" and hasattr(self, "on_cdna_updated"):
            self.on_cdna_updated(event)
        elif event["type"] == "adna_update" and hasattr(self, "on_adna_updated"):
            self.on_adna_updated(event)

    def get_cdna_slice(self, section: str):
        if not self.dna_guardian:
            return b""
        return self.dna_guardian.get_cdna_slice(section)

    def get_cdna_data(self):
        if not self.dna_guardian:
            return b""
        return self.dna_guardian.get_cdna_data()

    def close(self):
        """Unregister subscription from guardian (if any)."""
        try:
            if self.dna_guardian and self._subscription:
                self.dna_guardian._subscriptions.pop(self._subscription.subscriber_id, None)
                self._subscription = None
        except Exception:
            pass

    def __del__(self):
        try:
            self.close()
        except Exception:
            pass
