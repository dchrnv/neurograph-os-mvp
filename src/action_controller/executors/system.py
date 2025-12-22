"""
System Actions - Logging, metrics, and system operations.
"""

import logging
from typing import Dict, Any, Optional
from datetime import datetime
import json

from ..base import Action, ActionResult, ActionStatus, ActionPriority


logger = logging.getLogger(__name__)


class LoggingAction(Action):
    """
    Log signal processing events.

    This action logs signal events and processing results
    for monitoring, debugging, and analytics.
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority,
        log_level: str = "INFO",
        log_file: Optional[str] = None,
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize logging action.

        Args:
            action_id: Unique action identifier
            action_type: Action type
            priority: Execution priority
            log_level: Logging level (DEBUG, INFO, WARNING, ERROR)
            log_file: Optional file to log to
            metadata: Additional metadata
        """
        super().__init__(action_id, action_type, priority, metadata)
        self.log_level = log_level.upper()
        self.log_file = log_file

    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute logging.

        Args:
            context: Execution context

        Returns:
            ActionResult with logging status
        """
        start_time = datetime.now()

        try:
            signal_event = context.get("signal_event")
            processing_result = context.get("processing_result", {})

            # Build log entry
            log_entry = {
                "timestamp": start_time.isoformat(),
                "event_id": getattr(signal_event, "event_id", "unknown") if signal_event else "unknown",
                "processing": {
                    "token_id": processing_result.get("token_id"),
                    "is_novel": processing_result.get("is_novel"),
                    "triggered_actions": processing_result.get("triggered_actions", [])
                }
            }

            # Add signal data if available
            if signal_event:
                if hasattr(signal_event, "payload") and hasattr(signal_event.payload, "data"):
                    log_entry["data"] = str(signal_event.payload.data)[:200]  # Truncate

            # Log to console
            log_message = json.dumps(log_entry)

            if self.log_level == "DEBUG":
                logger.debug(log_message)
            elif self.log_level == "INFO":
                logger.info(log_message)
            elif self.log_level == "WARNING":
                logger.warning(log_message)
            elif self.log_level == "ERROR":
                logger.error(log_message)

            # Log to file if specified
            if self.log_file:
                with open(self.log_file, "a") as f:
                    f.write(log_message + "\n")

            execution_time = (datetime.now() - start_time).total_seconds() * 1000

            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.COMPLETED,
                success=True,
                data={"log_entry": log_entry},
                execution_time_ms=execution_time
            )

        except Exception as e:
            logger.error(f"Error in logging action: {e}", exc_info=True)
            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.FAILED,
                success=False,
                error=str(e)
            )

    def can_execute(self, context: Dict[str, Any]) -> bool:
        """Can always execute logging."""
        return True


class MetricsAction(Action):
    """
    Track metrics for signal processing.

    This action collects and tracks metrics about signal processing
    for monitoring and analytics.
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority,
        metrics_collector: Optional[Any] = None,
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize metrics action.

        Args:
            action_id: Unique action identifier
            action_type: Action type
            priority: Execution priority
            metrics_collector: Optional metrics collector instance
            metadata: Additional metadata
        """
        super().__init__(action_id, action_type, priority, metadata)
        self.metrics_collector = metrics_collector
        self._internal_metrics = {
            "total_events": 0,
            "novel_events": 0,
            "triggered_actions_total": 0,
        }

    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute metrics collection.

        Args:
            context: Execution context

        Returns:
            ActionResult with metrics status
        """
        start_time = datetime.now()

        try:
            signal_event = context.get("signal_event")
            processing_result = context.get("processing_result", {})

            # Update internal metrics
            self._internal_metrics["total_events"] += 1

            if processing_result.get("is_novel"):
                self._internal_metrics["novel_events"] += 1

            triggered_count = len(processing_result.get("triggered_actions", []))
            self._internal_metrics["triggered_actions_total"] += triggered_count

            # Send to external metrics collector if available
            if self.metrics_collector:
                try:
                    self.metrics_collector.increment("signal_events_total")

                    if processing_result.get("is_novel"):
                        self.metrics_collector.increment("signal_events_novel")

                    if triggered_count > 0:
                        self.metrics_collector.increment("signal_triggered_actions", triggered_count)

                    # Track processing time if available
                    processing_time = processing_result.get("processing_time_us")
                    if processing_time:
                        self.metrics_collector.histogram("signal_processing_time_us", processing_time)

                except Exception as e:
                    logger.warning(f"Error sending to metrics collector: {e}")

            execution_time = (datetime.now() - start_time).total_seconds() * 1000

            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.COMPLETED,
                success=True,
                data={"metrics": self._internal_metrics.copy()},
                execution_time_ms=execution_time
            )

        except Exception as e:
            logger.error(f"Error in metrics action: {e}", exc_info=True)
            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.FAILED,
                success=False,
                error=str(e)
            )

    def can_execute(self, context: Dict[str, Any]) -> bool:
        """Can always execute metrics."""
        return True

    def get_metrics(self) -> Dict[str, Any]:
        """
        Get current metrics.

        Returns:
            Dictionary with metric values
        """
        return self._internal_metrics.copy()

    def reset_metrics(self):
        """Reset internal metrics."""
        self._internal_metrics = {
            "total_events": 0,
            "novel_events": 0,
            "triggered_actions_total": 0,
        }
