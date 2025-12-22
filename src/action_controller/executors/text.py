"""
Text Response Actions - Generate text responses based on processing results.
"""

import logging
from typing import Dict, Any, Optional, Callable
from datetime import datetime

from ..base import Action, ActionResult, ActionStatus, ActionPriority


logger = logging.getLogger(__name__)


class TextResponseAction(Action):
    """
    Generate text response based on signal processing.

    This action generates a text response by analyzing:
    - Signal content (text, metadata)
    - Processing result (novelty, neighbors, triggered actions)
    - Context (conversation history, user info)

    Can be used as base for:
    - Chat bot responses
    - Telegram bot messages
    - Email responses
    - Any text-based interaction
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority,
        response_template: Optional[str] = None,
        response_generator: Optional[Callable] = None,
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize text response action.

        Args:
            action_id: Unique action identifier
            action_type: Action type
            priority: Execution priority
            response_template: Optional template string
            response_generator: Optional callable to generate response
            metadata: Additional metadata
        """
        super().__init__(action_id, action_type, priority, metadata)
        self.response_template = response_template
        self.response_generator = response_generator

    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute text response generation.

        Args:
            context: Execution context with:
                - signal_event: Original signal
                - processing_result: Processing result from Core
                - metadata: Additional context

        Returns:
            ActionResult with generated text response
        """
        start_time = datetime.now()

        try:
            signal_event = context.get("signal_event")
            processing_result = context.get("processing_result", {})

            # Generate response
            if self.response_generator:
                # Use custom generator
                response_text = self.response_generator(signal_event, processing_result, context)
            elif self.response_template:
                # Use template
                response_text = self._apply_template(signal_event, processing_result)
            else:
                # Generate default response
                response_text = self._generate_default_response(signal_event, processing_result)

            execution_time = (datetime.now() - start_time).total_seconds() * 1000

            logger.info(f"Generated text response ({len(response_text)} chars, {execution_time:.1f}ms)")

            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.COMPLETED,
                success=True,
                data={
                    "response_text": response_text,
                    "response_length": len(response_text)
                },
                execution_time_ms=execution_time
            )

        except Exception as e:
            logger.error(f"Error generating text response: {e}", exc_info=True)
            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.FAILED,
                success=False,
                error=str(e)
            )

    def can_execute(self, context: Dict[str, Any]) -> bool:
        """
        Check if response can be generated.

        Args:
            context: Execution context

        Returns:
            True if can execute
        """
        # Need at least signal_event
        return context.get("signal_event") is not None

    def _apply_template(
        self,
        signal_event: Any,
        processing_result: Dict[str, Any]
    ) -> str:
        """
        Apply template to generate response.

        Args:
            signal_event: Signal event
            processing_result: Processing result

        Returns:
            Generated text
        """
        # Extract variables for template
        variables = {
            "is_novel": processing_result.get("is_novel", False),
            "token_id": processing_result.get("token_id", "unknown"),
            "triggered_actions_count": len(processing_result.get("triggered_actions", [])),
        }

        # Try to extract text from signal
        if hasattr(signal_event, "payload"):
            if hasattr(signal_event.payload, "data"):
                variables["input_text"] = str(signal_event.payload.data)

        try:
            return self.response_template.format(**variables)
        except KeyError as e:
            logger.warning(f"Template variable missing: {e}")
            return self.response_template

    def _generate_default_response(
        self,
        signal_event: Any,
        processing_result: Dict[str, Any]
    ) -> str:
        """
        Generate default response.

        Args:
            signal_event: Signal event
            processing_result: Processing result

        Returns:
            Generated text
        """
        is_novel = processing_result.get("is_novel", False)
        triggered_actions = processing_result.get("triggered_actions", [])

        if is_novel:
            return "I sense something new! Processing your input..."
        elif triggered_actions:
            return f"Understood. This triggered {len(triggered_actions)} internal processes."
        else:
            return "Acknowledged."
