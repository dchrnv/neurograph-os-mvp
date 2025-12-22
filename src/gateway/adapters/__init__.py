"""
Gateway Adapters - input source integrations

Adapters provide convenient wrappers for integrating various
input sources with the Gateway:

- TextAdapter: Text-based inputs (Telegram, Slack, Discord, etc.)
- SystemAdapter: System monitoring and metrics
- TimerAdapter: Periodic/scheduled events
"""

from .text import TextAdapter
from .system import SystemAdapter
from .timer import TimerAdapter

__all__ = [
    "TextAdapter",
    "SystemAdapter",
    "TimerAdapter",
]
