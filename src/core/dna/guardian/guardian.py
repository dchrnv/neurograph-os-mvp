"""
DNAGuardian — центральный посредник между CDNA, ADNA и компонентами.
"""

import threading
from typing import Dict, List, Callable, Set, Any, Tuple, Optional
import time
import hashlib
from ..binary.cdna_format import CDNAStructure
from .events import DNAEvent
from .subscriptions import DNASubscription


class DNAGuardian:
    """
    Управляет связями между CDNA и интегрированными компонентами.
    Позволяет подписывать модули и уведомлять их об изменениях.
    """

    def __init__(self, cdna: Optional[CDNAStructure] = None):
        self._cdna: CDNAStructure = cdna or CDNAStructure()
        self._adna: Dict[str, Any] = {}
        self._subscriptions: Dict[str, DNASubscription] = {}
        self._event_history: List[DNAEvent] = []
        self._locks = {
            'cdna': threading.RLock(),
            'adna': threading.RLock(),
            'subscriptions': threading.RLock()
        }

        # Hot-slice cache: component -> (bytes, timestamp)
        self._hot_cache: Dict[str, Tuple[bytes, float]] = {}
        self._cache_ttl = 300.0  # seconds

        # Statistics
        self._stats = {
            'cdna_reads': 0,
            'adna_reads': 0,
            'cdna_writes': 0,
            'adna_writes': 0,
            'events_published': 0,
            'cache_hits': 0,
            'cache_misses': 0
        }

    # === Подписки ===
    def subscribe(self, subscription: DNASubscription) -> None:
        """Register a DNASubscription instance."""
        with self._locks['subscriptions']:
            self._subscriptions[subscription.subscriber_id] = subscription

    def unsubscribe(self, subscriber_id: str) -> None:
        """Unregister subscription by id."""
        with self._locks['subscriptions']:
            if subscriber_id in self._subscriptions:
                try:
                    del self._subscriptions[subscriber_id]
                except KeyError:
                    pass

    def _publish_event(self, event: 'DNAEvent') -> None:
        """Publish DNAEvent to interested subscriptions (thread-safe)."""
        self._stats['events_published'] += 1
        self._event_history.append(event)

        # trim history
        if len(self._event_history) > 1000:
            self._event_history = self._event_history[-500:]

        interested: List[DNASubscription] = []
        with self._locks['subscriptions']:
            for sub in self._subscriptions.values():
                try:
                    if sub.is_interested_in_event(event):
                        interested.append(sub)
                except Exception:
                    continue

        # notify outside of lock
        for sub in interested:
            try:
                if sub.callback:
                    sub.callback(event)
                sub.last_notification = event.timestamp
            except Exception as e:
                print(f"[DNA] Error notifying subscriber {sub.subscriber_id}: {e}")

    # === Доступ к данным CDNA ===
    def get_cdna_slice(self, component: str, subscriber_id: Optional[str] = None) -> bytes:
        """Return a hot slice for a named component, with caching.

        component: e.g. 'coordinate_system', 'token', 'graph', 'evolution'
        """
        self._stats['cdna_reads'] += 1

        # cache check
        if component in self._hot_cache:
            data, ts = self._hot_cache[component]
            if time.time() - ts < self._cache_ttl:
                self._stats['cache_hits'] += 1
                return data

        # miss
        self._stats['cache_misses'] += 1
        # delegate to CDNAStructure.get_hot_slice
        with self._locks['cdna']:
            try:
                data = self._cdna.get_hot_slice(component)
            except Exception:
                data = self._cdna.pack()

            # store cache
            self._hot_cache[component] = (data, time.time())
            return data

    def get_cdna_data(self) -> bytes:
        self._stats['cdna_reads'] += 1
        # return raw packed data
        with self._locks['cdna']:
            return self._cdna.pack()

    # === ADNA параметры (активные настройки) ===
    def update_adna(self, key: str, value: Any, updater_id: str, affected_components: Optional[Set[str]] = None) -> bool:
        """Atomically update ADNA parameter and publish event if changed."""
        try:
            with self._locks['adna']:
                old_value = self._adna.get(key)
                self._adna[key] = value
                self._stats['adna_writes'] += 1

            if old_value != value:
                if affected_components is None:
                    affected_components = self._guess_affected_components(key)

                event = DNAEvent(
                    event_type="ADNA_UPDATED",
                    affected_components=affected_components,
                    changed_data=str(value).encode(),
                    metadata={"key": key, "old_value": old_value, "new_value": value, "updater": updater_id}
                )
                self._publish_event(event)

            return True
        except Exception as e:
            print(f"ADNA update failed for key {key}: {e}")
            return False

    def get_adna_value(self, key: str, default: Any = None) -> Any:
        self._stats['adna_reads'] += 1
        with self._locks['adna']:
            return self._adna.get(key, default)

    def update_cdna(self, updater_id: str, new_cdna: CDNAStructure, affected_components: Optional[Set[str]] = None) -> bool:
        """Atomically replace CDNA after validation and publish event."""
        try:
            with self._locks['cdna']:
                if not self._validate_cdna(new_cdna):
                    return False

                old_data = self._cdna.pack()
                self._cdna = new_cdna
                new_data = new_cdna.pack()
                self._stats['cdna_writes'] += 1
                # clear cache
                self._hot_cache.clear()

                if affected_components is None:
                    affected_components = {"graph", "coordinate_system", "token", "evolution"}

                event = DNAEvent(
                    event_type="CDNA_UPDATED",
                    affected_components=affected_components,
                    changed_data=new_data,
                    metadata={"updater": updater_id, "old_checksum": int(hashlib.sha256(old_data).hexdigest()[:8], 16)}
                )

                self._publish_event(event)
                return True
        except Exception as e:
            print(f"CDNA update failed: {e}")
            return False

    def _validate_cdna(self, cdna: CDNAStructure) -> bool:
        try:
            packed = cdna.pack()
            if len(packed) != CDNAStructure.TOTAL_SIZE:
                return False
            # some basic checks
            tp = cdna.token_properties
            if tp.weight_min >= tp.weight_max:
                return False
            ec = cdna.evolution_constraints
            if ec.mutation_rate_base > ec.mutation_rate_max:
                return False
            return True
        except Exception:
            return False

    def _guess_affected_components(self, adna_key: str) -> Set[str]:
        key_lower = adna_key.lower()
        components = set()
        if any(word in key_lower for word in ['graph', 'connection', 'edge', 'node']):
            components.add('graph')
        if any(word in key_lower for word in ['coordinate', 'spatial', 'grid', 'position']):
            components.add('coordinate_system')
        if any(word in key_lower for word in ['token', 'weight', 'flag']):
            components.add('token')
        if any(word in key_lower for word in ['evolution', 'mutation', 'fitness']):
            components.add('evolution')
        return components if components else {'unknown'}

    def _publish_event_direct(self, event: DNAEvent) -> None:
        # backward compatible wrapper
        self._publish_event(event)

    def get_statistics(self) -> Dict[str, Any]:
        return {
            **self._stats,
            'active_subscriptions': len(self._subscriptions),
            'cached_components': len(self._hot_cache),
            'recent_events': len([e for e in self._event_history if time.time() - e.timestamp < 3600])
        }
