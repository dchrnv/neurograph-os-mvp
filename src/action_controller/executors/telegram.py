"""
Telegram Actions - Send messages via Telegram Bot API.
"""

import logging
from typing import Dict, Any, Optional
from datetime import datetime

from ..base import Action, ActionResult, ActionStatus, ActionPriority


logger = logging.getLogger(__name__)


class TelegramSendAction(Action):
    """
    Send message via Telegram bot.

    This action sends a message to a Telegram chat using the bot API.
    Requires telegram bot instance in context.
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority,
        chat_id: Optional[str] = None,
        text: Optional[str] = None,
        parse_mode: str = "Markdown",
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize Telegram send action.

        Args:
            action_id: Unique action identifier
            action_type: Action type
            priority: Execution priority
            chat_id: Target chat ID (can be overridden from context)
            text: Message text (can be overridden from context)
            parse_mode: Telegram parse mode
            metadata: Additional metadata
        """
        super().__init__(action_id, action_type, priority, metadata)
        self.chat_id = chat_id
        self.text = text
        self.parse_mode = parse_mode

    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute Telegram message send.

        Args:
            context: Execution context with:
                - telegram_bot: Telegram bot instance
                - chat_id: Target chat ID (optional, overrides init)
                - response_text: Message text (optional, overrides init)

        Returns:
            ActionResult with send status
        """
        start_time = datetime.now()

        try:
            # Get bot from context
            bot = context.get("telegram_bot")
            if not bot:
                raise ValueError("telegram_bot not found in context")

            # Get chat_id (context overrides init)
            chat_id = context.get("chat_id", self.chat_id)
            if not chat_id:
                raise ValueError("chat_id not provided")

            # Get text (context overrides init)
            text = context.get("response_text", self.text)
            if not text:
                raise ValueError("text not provided")

            # Send message
            message = await bot.send_message(
                chat_id=int(chat_id),
                text=text,
                parse_mode=self.parse_mode
            )

            execution_time = (datetime.now() - start_time).total_seconds() * 1000

            logger.info(f"Sent Telegram message to chat {chat_id} ({execution_time:.1f}ms)")

            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.COMPLETED,
                success=True,
                data={
                    "message_id": message.message_id,
                    "chat_id": chat_id,
                    "text": text
                },
                execution_time_ms=execution_time
            )

        except Exception as e:
            logger.error(f"Error sending Telegram message: {e}", exc_info=True)
            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.FAILED,
                success=False,
                error=str(e)
            )

    def can_execute(self, context: Dict[str, Any]) -> bool:
        """
        Check if can send Telegram message.

        Args:
            context: Execution context

        Returns:
            True if telegram_bot and chat_id available
        """
        has_bot = context.get("telegram_bot") is not None
        has_chat_id = context.get("chat_id") or self.chat_id
        has_text = context.get("response_text") or self.text
        return has_bot and has_chat_id and has_text


class TelegramReplyAction(Action):
    """
    Reply to a Telegram message.

    Similar to TelegramSendAction but replies to a specific message.
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority,
        chat_id: Optional[str] = None,
        reply_to_message_id: Optional[int] = None,
        text: Optional[str] = None,
        parse_mode: str = "Markdown",
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize Telegram reply action.

        Args:
            action_id: Unique action identifier
            action_type: Action type
            priority: Execution priority
            chat_id: Target chat ID
            reply_to_message_id: Message ID to reply to
            text: Reply text
            parse_mode: Telegram parse mode
            metadata: Additional metadata
        """
        super().__init__(action_id, action_type, priority, metadata)
        self.chat_id = chat_id
        self.reply_to_message_id = reply_to_message_id
        self.text = text
        self.parse_mode = parse_mode

    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute Telegram reply.

        Args:
            context: Execution context with:
                - telegram_bot: Telegram bot instance
                - chat_id: Target chat ID
                - reply_to_message_id: Message to reply to
                - response_text: Reply text

        Returns:
            ActionResult with send status
        """
        start_time = datetime.now()

        try:
            # Get bot from context
            bot = context.get("telegram_bot")
            if not bot:
                raise ValueError("telegram_bot not found in context")

            # Get parameters (context overrides init)
            chat_id = context.get("chat_id", self.chat_id)
            reply_to_message_id = context.get("reply_to_message_id", self.reply_to_message_id)
            text = context.get("response_text", self.text)

            if not all([chat_id, reply_to_message_id, text]):
                raise ValueError("Missing required parameters: chat_id, reply_to_message_id, or text")

            # Send reply
            message = await bot.send_message(
                chat_id=int(chat_id),
                text=text,
                reply_to_message_id=int(reply_to_message_id),
                parse_mode=self.parse_mode
            )

            execution_time = (datetime.now() - start_time).total_seconds() * 1000

            logger.info(f"Sent Telegram reply to message {reply_to_message_id} ({execution_time:.1f}ms)")

            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.COMPLETED,
                success=True,
                data={
                    "message_id": message.message_id,
                    "chat_id": chat_id,
                    "reply_to_message_id": reply_to_message_id,
                    "text": text
                },
                execution_time_ms=execution_time
            )

        except Exception as e:
            logger.error(f"Error sending Telegram reply: {e}", exc_info=True)
            return ActionResult(
                action_id=self.action_id,
                status=ActionStatus.FAILED,
                success=False,
                error=str(e)
            )

    def can_execute(self, context: Dict[str, Any]) -> bool:
        """
        Check if can send Telegram reply.

        Args:
            context: Execution context

        Returns:
            True if all required parameters available
        """
        has_bot = context.get("telegram_bot") is not None
        has_chat_id = context.get("chat_id") or self.chat_id
        has_reply_id = context.get("reply_to_message_id") or self.reply_to_message_id
        has_text = context.get("response_text") or self.text
        return has_bot and has_chat_id and has_reply_id and has_text
