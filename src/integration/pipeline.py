"""
SignalPipeline - Full end-to-end signal processing pipeline.
"""

import logging
from typing import Dict, Any, Optional
from datetime import datetime

from src.gateway import SignalGateway
from src.action_controller import ActionController


logger = logging.getLogger(__name__)


class SignalPipeline:
    """
    Full signal processing pipeline.

    Orchestrates:
    1. Gateway (Python) - Sensory interface, encoding
    2. Core (Rust) - Signal processing, pattern matching [Optional]
    3. ActionController (Python) - Response generation

    Flow:
        Input → Gateway.push_*() → SignalEvent
                                 ↓
                            [Core.emit()] → ProcessingResult  [If Core available]
                                 ↓
                        ActionController.process() → Actions
                                 ↓
                              Response

    Usage:
        pipeline = SignalPipeline()
        result = await pipeline.process_text("Hello!", user_id="user_123")
    """

    def __init__(
        self,
        gateway: Optional[SignalGateway] = None,
        controller: Optional[ActionController] = None,
        core_system: Optional[Any] = None
    ):
        """
        Initialize pipeline.

        Args:
            gateway: SignalGateway instance (creates new if None)
            controller: ActionController instance (creates new if None)
            core_system: Optional Rust Core SignalSystem
        """
        self.gateway = gateway or SignalGateway(core_system=core_system)
        self.controller = controller or ActionController()
        self.core_system = core_system

        # Initialize gateway if not already
        if not hasattr(self.gateway, '_encoders') or not self.gateway._encoders:
            self.gateway.initialize()

        # Statistics
        self._total_processed = 0
        self._with_core = 0
        self._without_core = 0

        logger.info(f"SignalPipeline initialized (core={'enabled' if core_system else 'disabled'})")

    async def process_text(
        self,
        text: str,
        user_id: Optional[str] = None,
        chat_id: Optional[str] = None,
        priority: Optional[int] = None,
        sequence_id: Optional[str] = None,
        metadata: Optional[Dict[str, Any]] = None,
        action_context: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Process text input through full pipeline.

        Args:
            text: Input text
            user_id: User identifier
            chat_id: Chat identifier
            priority: Signal priority
            sequence_id: Conversation sequence ID
            metadata: Additional metadata
            action_context: Context for action execution (e.g., telegram_bot)

        Returns:
            Dictionary with:
            {
                "signal_event": SignalEvent,
                "processing_result": ProcessingResult (if Core available),
                "action_results": ActionController results,
                "stats": {...}
            }
        """
        start_time = datetime.now()

        # Merge user_id/chat_id into metadata
        combined_metadata = metadata or {}
        if user_id:
            combined_metadata["user_id"] = user_id
        if chat_id:
            combined_metadata["chat_id"] = chat_id

        # Step 1: Gateway - Create SignalEvent
        signal_event = self.gateway.push_text(
            text=text,
            priority=priority or 200,
            sequence_id=sequence_id,
            metadata=combined_metadata
        )

        logger.debug(f"Gateway created event: {signal_event.event_id}")

        # Step 2: Core - Process signal (if available)
        processing_result = await self._process_through_core(signal_event)

        # Step 3: ActionController - Execute actions
        action_results = await self._execute_actions(
            signal_event=signal_event,
            processing_result=processing_result,
            action_context=action_context or {}
        )

        # Update statistics
        self._total_processed += 1
        if processing_result.get("from_core"):
            self._with_core += 1
        else:
            self._without_core += 1

        total_time = (datetime.now() - start_time).total_seconds() * 1000

        return {
            "signal_event": signal_event,
            "processing_result": processing_result,
            "action_results": action_results,
            "stats": {
                "total_time_ms": total_time,
                "gateway_time_ms": processing_result.get("gateway_time_ms", 0),
                "core_time_ms": processing_result.get("core_time_ms", 0),
                "action_time_ms": action_results.get("stats", {}).get("execution_time_ms", 0)
            }
        }

    async def _process_through_core(
        self,
        signal_event: Any
    ) -> Dict[str, Any]:
        """
        Process signal through Rust Core if available.

        Args:
            signal_event: SignalEvent from Gateway

        Returns:
            ProcessingResult dictionary
        """
        if not self.core_system:
            # No Core available - simulate processing
            logger.debug("Core not available, simulating processing")
            return {
                "from_core": False,
                "token_id": None,
                "is_novel": False,
                "neighbors": [],
                "triggered_actions": [],
                "anomaly_score": 0.0,
                "processing_time_us": 0,
                "core_time_ms": 0
            }

        try:
            start_time = datetime.now()

            # Call Rust Core with correct API
            result = self.core_system.emit(
                event_type=signal_event.event_type,
                vector=list(signal_event.semantic.vector),  # Convert to list
                priority=signal_event.routing.priority
            )

            core_time = (datetime.now() - start_time).total_seconds() * 1000

            logger.debug(f"Core processed in {core_time:.2f}ms (token={result.get('token_id')})")

            return {
                **result,
                "from_core": True,
                "core_time_ms": core_time
            }

        except Exception as e:
            logger.error(f"Error processing through Core: {e}", exc_info=True)
            # Fallback to simulated processing
            return {
                "from_core": False,
                "error": str(e),
                "token_id": None,
                "is_novel": False,
                "neighbors": [],
                "triggered_actions": [],
                "anomaly_score": 0.0,
                "processing_time_us": 0,
                "core_time_ms": 0
            }

    async def _execute_actions(
        self,
        signal_event: Any,
        processing_result: Dict[str, Any],
        action_context: Dict[str, Any]
    ) -> Dict[str, Any]:
        """
        Execute actions via ActionController.

        Args:
            signal_event: SignalEvent
            processing_result: Processing result from Core
            action_context: Additional context for actions

        Returns:
            ActionController results
        """
        try:
            # Build context for ActionController
            context = {
                "signal_event": signal_event,
                "processing_result": processing_result,
                **action_context  # Merge additional context (e.g., telegram_bot, chat_id)
            }

            # Process through ActionController
            result = await self.controller.process(
                signal_event=signal_event,
                processing_result=processing_result,
                metadata=action_context
            )

            return result

        except Exception as e:
            logger.error(f"Error executing actions: {e}", exc_info=True)
            return {
                "hot_path_results": [],
                "cold_path_queued": [],
                "stats": {
                    "hot_path_executed": 0,
                    "cold_path_queued": 0,
                    "execution_time_ms": 0
                },
                "error": str(e)
            }

    def configure_actions(
        self,
        hot_path: Optional[list] = None,
        cold_path: Optional[list] = None
    ):
        """
        Configure default action paths.

        Args:
            hot_path: Action types for hot path (immediate)
            cold_path: Action types for cold path (background)
        """
        if hot_path:
            self.controller.selector.set_default_hot_path(hot_path)
            logger.info(f"Hot path configured: {hot_path}")

        if cold_path:
            self.controller.selector.set_default_cold_path(cold_path)
            logger.info(f"Cold path configured: {cold_path}")

    def register_action(
        self,
        action_type: str,
        action_class: Any,
        **kwargs
    ):
        """
        Register an action type.

        Args:
            action_type: Action type identifier
            action_class: Action class
            **kwargs: Additional registration parameters
        """
        self.controller.registry.register(
            action_type=action_type,
            action_class=action_class,
            **kwargs
        )
        logger.info(f"Registered action: {action_type}")

    def get_stats(self) -> Dict[str, Any]:
        """
        Get pipeline statistics.

        Returns:
            Dictionary with statistics
        """
        return {
            "pipeline": {
                "total_processed": self._total_processed,
                "with_core": self._with_core,
                "without_core": self._without_core,
            },
            "gateway": self.gateway.get_stats(),
            "controller": self.controller.get_stats(),
        }

    async def shutdown(self):
        """Shutdown pipeline gracefully."""
        logger.info("Shutting down SignalPipeline")
        await self.controller.shutdown()
        logger.info("SignalPipeline shutdown complete")
