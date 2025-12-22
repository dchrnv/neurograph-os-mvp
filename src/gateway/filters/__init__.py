"""
Gateway Filters - event filtering and subscription system

Provides MongoDB-style query operators for filtering SignalEvents
in subscription handlers.
"""

from .subscription_filter import SubscriptionFilter

__all__ = [
    "SubscriptionFilter",
]
