"""
SystemAdapter - for system monitoring and metrics

Provides convenient methods for collecting and sending system metrics,
resource usage, health checks, and alerts.
"""

from typing import Optional, Dict, Any, Callable
import time
import threading
from ..gateway import SignalGateway
from ..models import SignalEvent


class SystemAdapter:
    """
    Adapter for system monitoring and metrics.

    Handles:
    - Resource metrics (CPU, memory, disk, network)
    - Health checks
    - Alerts and anomalies
    - Periodic monitoring
    - Custom metrics

    Usage:
        gateway = SignalGateway()
        gateway.initialize()

        adapter = SystemAdapter(gateway)

        # Send single metric
        adapter.send_metric("cpu_percent", 45.7)

        # Send resource snapshot
        adapter.send_resource_snapshot()

        # Start periodic monitoring
        adapter.start_monitoring(interval_seconds=5)
    """

    def __init__(
        self,
        gateway: SignalGateway,
        sensor_id: str = "builtin.system_monitor",
        default_priority: int = 100,
    ):
        """
        Initialize SystemAdapter.

        Args:
            gateway: SignalGateway instance
            sensor_id: Sensor ID for metrics
            default_priority: Default priority for metrics
        """
        self.gateway = gateway
        self.sensor_id = sensor_id
        self.default_priority = default_priority

        # Monitoring state
        self._monitoring = False
        self._monitor_thread = None
        self._monitor_interval = 5.0

        # Custom metric collectors
        self._custom_collectors: Dict[str, Callable[[], float]] = {}

    def send_metric(
        self,
        metric_name: str,
        metric_value: float,
        priority: Optional[int] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Send single metric.

        Args:
            metric_name: Metric name (cpu_percent, memory_mb, etc.)
            metric_value: Metric value
            priority: Override default priority
            metadata: Additional metadata

        Returns:
            Created SignalEvent
        """
        msg_priority = priority if priority is not None else self.default_priority

        # Elevate priority for critical values
        if self._is_critical(metric_name, metric_value):
            msg_priority = max(msg_priority, 200)

        return self.gateway.push_system(
            metric_name=metric_name,
            metric_value=metric_value,
            sensor_id=self.sensor_id,
            priority=msg_priority,
            metadata=metadata or {},
        )

    def send_resource_snapshot(self) -> Dict[str, SignalEvent]:
        """
        Send snapshot of system resources.

        Requires psutil package:
            pip install psutil

        Returns:
            Dict mapping metric name to SignalEvent
        """
        try:
            import psutil
        except ImportError:
            raise ImportError("psutil required for resource monitoring: pip install psutil")

        events = {}

        # CPU
        cpu_percent = psutil.cpu_percent(interval=0.1)
        events["cpu_percent"] = self.send_metric("cpu_percent", cpu_percent)

        # Memory
        mem = psutil.virtual_memory()
        events["memory_percent"] = self.send_metric("memory_percent", mem.percent)
        events["memory_available_mb"] = self.send_metric("memory_available_mb", mem.available / 1024 / 1024)

        # Disk
        disk = psutil.disk_usage('/')
        events["disk_percent"] = self.send_metric("disk_percent", disk.percent)
        events["disk_free_gb"] = self.send_metric("disk_free_gb", disk.free / 1024 / 1024 / 1024)

        # Network I/O
        net = psutil.net_io_counters()
        events["network_bytes_sent"] = self.send_metric("network_bytes_sent", float(net.bytes_sent))
        events["network_bytes_recv"] = self.send_metric("network_bytes_recv", float(net.bytes_recv))

        return events

    def send_alert(
        self,
        alert_name: str,
        severity: str = "warning",
        message: str = "",
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Send system alert.

        Args:
            alert_name: Alert identifier
            severity: critical | warning | info
            message: Alert message
            metadata: Additional context

        Returns:
            Created SignalEvent
        """
        severity_priority = {
            "critical": 250,
            "warning": 200,
            "info": 150,
        }

        priority = severity_priority.get(severity, 150)

        alert_metadata = metadata or {}
        alert_metadata["severity"] = severity
        alert_metadata["message"] = message
        alert_metadata["alert_type"] = alert_name

        return self.gateway.push_system(
            metric_name=f"alert.{alert_name}",
            metric_value=1.0,  # Alert active
            sensor_id=self.sensor_id,
            priority=priority,
            metadata=alert_metadata,
        )

    def register_custom_metric(
        self,
        metric_name: str,
        collector: Callable[[], float],
    ):
        """
        Register custom metric collector.

        Args:
            metric_name: Metric name
            collector: Function that returns metric value

        Example:
            def get_queue_size():
                return len(my_queue)

            adapter.register_custom_metric("queue_size", get_queue_size)
        """
        self._custom_collectors[metric_name] = collector

    def send_custom_metrics(self) -> Dict[str, SignalEvent]:
        """
        Send all registered custom metrics.

        Returns:
            Dict mapping metric name to SignalEvent
        """
        events = {}

        for metric_name, collector in self._custom_collectors.items():
            try:
                value = collector()
                events[metric_name] = self.send_metric(metric_name, value)
            except Exception as e:
                print(f"Warning: custom metric '{metric_name}' collector failed: {e}")

        return events

    # ═══════════════════════════════════════════════════════════════════════════════
    # PERIODIC MONITORING
    # ═══════════════════════════════════════════════════════════════════════════════

    def start_monitoring(
        self,
        interval_seconds: float = 5.0,
        include_resources: bool = True,
        include_custom: bool = True,
    ):
        """
        Start periodic monitoring in background thread.

        Args:
            interval_seconds: Monitoring interval
            include_resources: Send resource metrics (CPU, memory, etc.)
            include_custom: Send custom metrics
        """
        if self._monitoring:
            print("Warning: monitoring already running")
            return

        self._monitoring = True
        self._monitor_interval = interval_seconds

        def monitor_loop():
            while self._monitoring:
                try:
                    if include_resources:
                        self.send_resource_snapshot()

                    if include_custom:
                        self.send_custom_metrics()

                except Exception as e:
                    print(f"Error in monitoring loop: {e}")

                time.sleep(self._monitor_interval)

        self._monitor_thread = threading.Thread(target=monitor_loop, daemon=True)
        self._monitor_thread.start()

        print(f"System monitoring started (interval={interval_seconds}s)")

    def stop_monitoring(self):
        """Stop periodic monitoring."""
        if not self._monitoring:
            return

        self._monitoring = False

        if self._monitor_thread:
            self._monitor_thread.join(timeout=2.0)
            self._monitor_thread = None

        print("System monitoring stopped")

    def is_monitoring(self) -> bool:
        """Check if monitoring is active."""
        return self._monitoring

    # ═══════════════════════════════════════════════════════════════════════════════
    # UTILITIES
    # ═══════════════════════════════════════════════════════════════════════════════

    def _is_critical(self, metric_name: str, value: float) -> bool:
        """
        Determine if metric value is critical.

        Args:
            metric_name: Metric name
            value: Metric value

        Returns:
            True if critical
        """
        critical_thresholds = {
            "cpu_percent": 90.0,
            "memory_percent": 90.0,
            "disk_percent": 95.0,
        }

        threshold = critical_thresholds.get(metric_name)
        if threshold is None:
            return False

        return value >= threshold


__all__ = [
    "SystemAdapter",
]
