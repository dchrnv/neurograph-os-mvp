"""
ActionController - Main orchestrator for action execution.
"""

import asyncio
import logging
import uuid
from typing import Dict, List, Any, Optional
from datetime import datetime

from .base import Action, ActionResult, ActionStatus, ActionPriority
from .registry import ActionRegistry
from .selector import ActionSelector, PathType


logger = logging.getLogger(__name__)


class ActionController:
    """
    Main controller for action execution.

    Responsibilities:
    1. Receive ProcessingResult from SignalSystem
    2. Use ActionSelector to choose actions
    3. Route to Hot Path (immediate) or Cold Path (background)
    4. Execute actions via ActionRegistry
    5. Track execution results
    6. Handle errors and retries

    Flow:
        ProcessingResult → select_actions() → execute_hot_path() + queue_cold_path()
                                           ↓                      ↓
                                    [Text Response]        [Logging, Analytics]
    """

    def __init__(
        self,
        registry: Optional[ActionRegistry] = None,
        selector: Optional[ActionSelector] = None
    ):
        """
        Initialize ActionController.

        Args:
            registry: ActionRegistry instance (creates new if None)
            selector: ActionSelector instance (creates new if None)
        """
        self.registry = registry or ActionRegistry()
        self.selector = selector or ActionSelector()

        # Execution tracking
        self._execution_count = 0
        self._hot_path_count = 0
        self._cold_path_count = 0
        self._failed_count = 0

        # Cold path queue
        self._cold_path_queue: List[Dict[str, Any]] = []
        self._cold_path_task: Optional[asyncio.Task] = None
        self._cold_path_running = False

        logger.info("ActionController initialized")

    async def process(
        self,
        signal_event: Any,
        processing_result: Dict[str, Any],
        metadata: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Process signal event and execute appropriate actions.

        Args:
            signal_event: SignalEvent that was processed
            processing_result: ProcessingResult from SignalSystem
            metadata: Additional context

        Returns:
            Dictionary with execution results:
            {
                "hot_path_results": [...],
                "cold_path_queued": [...],
                "stats": {...}
            }
        """
        start_time = datetime.now()

        # Build context
        context = {
            "signal_event": signal_event,
            "processing_result": processing_result,
            "metadata": metadata or {},
            "timestamp": start_time.isoformat()
        }

        # Select actions
        selected = self.selector.select(context)

        # Execute hot path (immediate, synchronous)
        hot_results = await self._execute_hot_path(
            selected[PathType.HOT],
            context
        )

        # Queue cold path (background, asynchronous)
        cold_queued = self._queue_cold_path(
            selected[PathType.COLD],
            context
        )

        # Update stats
        self._execution_count += 1
        self._hot_path_count += len(hot_results)
        self._cold_path_count += len(cold_queued)

        execution_time = (datetime.now() - start_time).total_seconds() * 1000

        return {
            "hot_path_results": hot_results,
            "cold_path_queued": cold_queued,
            "stats": {
                "hot_path_executed": len(hot_results),
                "cold_path_queued": len(cold_queued),
                "execution_time_ms": execution_time
            }
        }

    async def _execute_hot_path(
        self,
        action_specs: List[Dict[str, Any]],
        context: Dict[str, Any]
    ) -> List[ActionResult]:
        """
        Execute hot path actions (immediate).

        Args:
            action_specs: List of action specifications
            context: Execution context

        Returns:
            List of ActionResults
        """
        if not action_specs:
            logger.debug("No hot path actions to execute")
            return []

        logger.info(f"Executing {len(action_specs)} hot path actions")

        results = []

        for spec in action_specs:
            action_type = spec["action_type"]
            action_id = f"{action_type}_{uuid.uuid4().hex[:8]}"

            try:
                # Create action
                action = self.registry.create_action(
                    action_type=action_type,
                    action_id=action_id,
                    priority=spec.get("priority", ActionPriority.NORMAL)
                )

                if not action:
                    logger.error(f"Failed to create action: {action_type}")
                    results.append(ActionResult(
                        action_id=action_id,
                        status=ActionStatus.FAILED,
                        success=False,
                        error=f"Failed to create action type: {action_type}"
                    ))
                    continue

                # Check if can execute
                if not action.can_execute(context):
                    logger.warning(f"Action {action_id} cannot execute in current context")
                    results.append(ActionResult(
                        action_id=action_id,
                        status=ActionStatus.CANCELLED,
                        success=False,
                        error="Action cannot execute in current context"
                    ))
                    continue

                # Execute
                action.status = ActionStatus.EXECUTING
                start_time = datetime.now()

                result = await action.execute(context)
                result.execution_time_ms = (datetime.now() - start_time).total_seconds() * 1000

                results.append(result)

                if result.success:
                    logger.info(f"Action {action_id} succeeded ({result.execution_time_ms:.1f}ms)")
                else:
                    logger.warning(f"Action {action_id} failed: {result.error}")
                    self._failed_count += 1

            except Exception as e:
                logger.error(f"Error executing action {action_id}: {e}", exc_info=True)
                self._failed_count += 1
                results.append(ActionResult(
                    action_id=action_id,
                    status=ActionStatus.FAILED,
                    success=False,
                    error=str(e)
                ))

        return results

    def _queue_cold_path(
        self,
        action_specs: List[Dict[str, Any]],
        context: Dict[str, Any]
    ) -> List[str]:
        """
        Queue cold path actions for background execution.

        Args:
            action_specs: List of action specifications
            context: Execution context

        Returns:
            List of queued action IDs
        """
        if not action_specs:
            logger.debug("No cold path actions to queue")
            return []

        logger.info(f"Queueing {len(action_specs)} cold path actions")

        queued_ids = []

        for spec in action_specs:
            action_type = spec["action_type"]
            action_id = f"{action_type}_{uuid.uuid4().hex[:8]}"

            self._cold_path_queue.append({
                "action_id": action_id,
                "action_type": action_type,
                "spec": spec,
                "context": context,
                "queued_at": datetime.now().isoformat()
            })

            queued_ids.append(action_id)

        # Start cold path worker if not running
        if not self._cold_path_running:
            asyncio.create_task(self._cold_path_worker())

        return queued_ids

    async def _cold_path_worker(self):
        """Background worker for cold path actions."""
        if self._cold_path_running:
            return

        self._cold_path_running = True
        logger.info("Cold path worker started")

        try:
            while self._cold_path_queue:
                item = self._cold_path_queue.pop(0)

                try:
                    action = self.registry.create_action(
                        action_type=item["action_type"],
                        action_id=item["action_id"],
                        priority=ActionPriority.BACKGROUND
                    )

                    if action and action.can_execute(item["context"]):
                        result = await action.execute(item["context"])
                        if result.success:
                            logger.debug(f"Cold path action {item['action_id']} succeeded")
                        else:
                            logger.warning(f"Cold path action {item['action_id']} failed: {result.error}")
                    else:
                        logger.warning(f"Skipping cold path action {item['action_id']}")

                except Exception as e:
                    logger.error(f"Error in cold path worker for {item['action_id']}: {e}")

                # Small delay between actions
                await asyncio.sleep(0.01)

        finally:
            self._cold_path_running = False
            logger.info("Cold path worker stopped")

    def get_stats(self) -> Dict[str, Any]:
        """
        Get controller statistics.

        Returns:
            Dictionary with execution stats
        """
        return {
            "total_executions": self._execution_count,
            "hot_path_executed": self._hot_path_count,
            "cold_path_executed": self._cold_path_count,
            "failed_actions": self._failed_count,
            "cold_path_queued": len(self._cold_path_queue),
            "cold_path_running": self._cold_path_running,
            "registry_stats": self.registry.get_stats(),
            "selector_stats": self.selector.get_stats(),
        }

    async def shutdown(self):
        """Shutdown controller gracefully."""
        logger.info("Shutting down ActionController")

        # Wait for cold path queue to drain
        max_wait = 10  # seconds
        waited = 0
        while self._cold_path_queue and waited < max_wait:
            logger.info(f"Waiting for {len(self._cold_path_queue)} cold path actions...")
            await asyncio.sleep(1)
            waited += 1

        if self._cold_path_queue:
            logger.warning(f"Shutdown with {len(self._cold_path_queue)} actions still queued")

        logger.info("ActionController shutdown complete")
