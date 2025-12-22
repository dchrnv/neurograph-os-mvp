"""
TimerAdapter - for periodic/scheduled events

Provides convenient methods for creating timer-based events,
scheduled tasks, and periodic signals.
"""

from typing import Optional, Dict, Any, Callable
import time
import threading
from datetime import datetime, timedelta
from ..gateway import SignalGateway
from ..models import SignalEvent


class TimerAdapter:
    """
    Adapter for timer and scheduled events.

    Handles:
    - Periodic events (every N seconds)
    - Scheduled events (at specific time)
    - Countdown timers
    - Cron-like scheduling
    - One-shot timers

    Usage:
        gateway = SignalGateway()
        gateway.initialize()

        adapter = TimerAdapter(gateway)

        # Periodic event every 5 seconds
        timer_id = adapter.start_periodic(
            interval_seconds=5.0,
            callback=lambda event: print(f"Tick: {event.temporal.neuro_tick}")
        )

        # Stop timer
        adapter.stop_timer(timer_id)
    """

    def __init__(
        self,
        gateway: SignalGateway,
        sensor_id: str = "builtin.timer",
    ):
        """
        Initialize TimerAdapter.

        Args:
            gateway: SignalGateway instance
            sensor_id: Sensor ID for timer events
        """
        self.gateway = gateway
        self.sensor_id = sensor_id

        # Active timers
        self._timers: Dict[str, threading.Thread] = {}
        self._timer_states: Dict[str, bool] = {}
        self._next_timer_id = 0

    def send_timer_event(
        self,
        timer_name: str = "tick",
        tick_count: int = 0,
        priority: int = 50,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Send single timer event.

        Args:
            timer_name: Timer identifier
            tick_count: Tick counter value
            priority: Event priority
            metadata: Additional metadata

        Returns:
            Created SignalEvent
        """
        timer_metadata = metadata or {}
        timer_metadata["timer_name"] = timer_name
        timer_metadata["tick_count"] = tick_count

        # Create 8D vector: [tick_normalized, 0, 0, 0, 0, 0, 0, 0]
        # Normalize tick to [0, 1] range (assuming max 1000 ticks)
        tick_normalized = min(tick_count / 1000.0, 1.0)

        # Use push_system with dict data for PASSTHROUGH encoder
        return self.gateway.push_system(
            metric_name=f"timer.{timer_name}",
            metric_value=float(tick_count),
            sensor_id=self.sensor_id,
            priority=priority,
            metadata=timer_metadata,
        )

    def start_periodic(
        self,
        interval_seconds: float,
        callback: Optional[Callable[[SignalEvent], None]] = None,
        timer_name: str = "periodic",
        max_ticks: Optional[int] = None,
        priority: int = 50,
    ) -> str:
        """
        Start periodic timer.

        Args:
            interval_seconds: Interval between ticks
            callback: Optional callback function(event)
            timer_name: Timer identifier
            max_ticks: Maximum tick count (None = infinite)
            priority: Event priority

        Returns:
            Timer ID (for stopping)
        """
        timer_id = self._generate_timer_id()
        self._timer_states[timer_id] = True

        def timer_loop():
            tick_count = 0

            while self._timer_states.get(timer_id, False):
                # Send event
                event = self.send_timer_event(
                    timer_name=timer_name,
                    tick_count=tick_count,
                    priority=priority,
                )

                # Call callback if provided
                if callback:
                    try:
                        callback(event)
                    except Exception as e:
                        print(f"Timer callback error: {e}")

                # Check max ticks
                tick_count += 1
                if max_ticks is not None and tick_count >= max_ticks:
                    self._timer_states[timer_id] = False
                    break

                # Sleep
                time.sleep(interval_seconds)

            # Cleanup
            if timer_id in self._timers:
                del self._timers[timer_id]
            if timer_id in self._timer_states:
                del self._timer_states[timer_id]

        thread = threading.Thread(target=timer_loop, daemon=True)
        thread.start()
        self._timers[timer_id] = thread

        return timer_id

    def start_countdown(
        self,
        duration_seconds: float,
        callback: Optional[Callable[[SignalEvent], None]] = None,
        timer_name: str = "countdown",
        tick_interval: float = 1.0,
        priority: int = 100,
    ) -> str:
        """
        Start countdown timer.

        Args:
            duration_seconds: Total countdown duration
            callback: Callback for each tick
            timer_name: Timer identifier
            tick_interval: Interval between ticks
            priority: Event priority

        Returns:
            Timer ID
        """
        max_ticks = int(duration_seconds / tick_interval)

        return self.start_periodic(
            interval_seconds=tick_interval,
            callback=callback,
            timer_name=timer_name,
            max_ticks=max_ticks,
            priority=priority,
        )

    def start_scheduled(
        self,
        target_time: datetime,
        callback: Optional[Callable[[SignalEvent], None]] = None,
        timer_name: str = "scheduled",
        priority: int = 150,
    ) -> str:
        """
        Schedule event at specific time.

        Args:
            target_time: When to fire event
            callback: Callback when time reached
            timer_name: Timer identifier
            priority: Event priority

        Returns:
            Timer ID
        """
        timer_id = self._generate_timer_id()
        self._timer_states[timer_id] = True

        def scheduled_loop():
            # Wait until target time
            now = datetime.now()
            if target_time > now:
                wait_seconds = (target_time - now).total_seconds()
                time.sleep(wait_seconds)

            # Check if still active
            if not self._timer_states.get(timer_id, False):
                return

            # Send event
            event = self.send_timer_event(
                timer_name=timer_name,
                tick_count=1,
                priority=priority,
                metadata={"target_time": target_time.isoformat()},
            )

            # Call callback
            if callback:
                try:
                    callback(event)
                except Exception as e:
                    print(f"Scheduled timer callback error: {e}")

            # Cleanup
            if timer_id in self._timers:
                del self._timers[timer_id]
            if timer_id in self._timer_states:
                del self._timer_states[timer_id]

        thread = threading.Thread(target=scheduled_loop, daemon=True)
        thread.start()
        self._timers[timer_id] = thread

        return timer_id

    def start_daily(
        self,
        hour: int,
        minute: int = 0,
        callback: Optional[Callable[[SignalEvent], None]] = None,
        timer_name: str = "daily",
        priority: int = 150,
    ) -> str:
        """
        Start daily recurring event.

        Args:
            hour: Hour (0-23)
            minute: Minute (0-59)
            callback: Callback for each occurrence
            timer_name: Timer identifier
            priority: Event priority

        Returns:
            Timer ID
        """
        timer_id = self._generate_timer_id()
        self._timer_states[timer_id] = True

        def daily_loop():
            while self._timer_states.get(timer_id, False):
                # Calculate next occurrence
                now = datetime.now()
                target = now.replace(hour=hour, minute=minute, second=0, microsecond=0)

                if target <= now:
                    # Already passed today, schedule for tomorrow
                    target += timedelta(days=1)

                # Wait until target time
                wait_seconds = (target - datetime.now()).total_seconds()
                if wait_seconds > 0:
                    time.sleep(wait_seconds)

                # Check if still active
                if not self._timer_states.get(timer_id, False):
                    break

                # Send event
                event = self.send_timer_event(
                    timer_name=timer_name,
                    tick_count=1,
                    priority=priority,
                    metadata={"scheduled_time": target.isoformat()},
                )

                # Call callback
                if callback:
                    try:
                        callback(event)
                    except Exception as e:
                        print(f"Daily timer callback error: {e}")

            # Cleanup
            if timer_id in self._timers:
                del self._timers[timer_id]
            if timer_id in self._timer_states:
                del self._timer_states[timer_id]

        thread = threading.Thread(target=daily_loop, daemon=True)
        thread.start()
        self._timers[timer_id] = thread

        return timer_id

    def stop_timer(self, timer_id: str) -> bool:
        """
        Stop timer.

        Args:
            timer_id: Timer ID returned by start_*()

        Returns:
            True if timer was found and stopped
        """
        if timer_id not in self._timer_states:
            return False

        self._timer_states[timer_id] = False

        # Wait for thread to finish
        if timer_id in self._timers:
            thread = self._timers[timer_id]
            thread.join(timeout=1.0)

        return True

    def stop_all_timers(self):
        """Stop all active timers."""
        timer_ids = list(self._timer_states.keys())
        for timer_id in timer_ids:
            self.stop_timer(timer_id)

    def get_active_timers(self) -> list:
        """Get list of active timer IDs."""
        return list(self._timer_states.keys())

    def _generate_timer_id(self) -> str:
        """Generate unique timer ID."""
        timer_id = f"timer_{self._next_timer_id}"
        self._next_timer_id += 1
        return timer_id


__all__ = [
    "TimerAdapter",
]
