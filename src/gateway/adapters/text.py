"""
TextAdapter - for text-based inputs (Telegram, Slack, Discord, etc.)

Provides convenient methods for handling text messages from various
chat platforms with automatic metadata extraction and tagging.
"""

from typing import Optional, Dict, Any, List
from ..gateway import SignalGateway
from ..models import SignalEvent


class TextAdapter:
    """
    Adapter for text-based message sources.

    Handles:
    - Message normalization
    - Metadata extraction (user_id, chat_id, platform)
    - Automatic tagging
    - Conversation threading
    - Priority assignment

    Usage:
        gateway = SignalGateway()
        gateway.initialize()

        adapter = TextAdapter(gateway, source_name="telegram")

        # Handle Telegram message
        event = adapter.handle_message(
            text="Hello, bot!",
            user_id="12345",
            chat_id="67890",
            metadata={"username": "@user"}
        )
    """

    def __init__(
        self,
        gateway: SignalGateway,
        source_name: str = "text_chat",
        default_priority: int = 200,
        auto_tag: bool = True,
    ):
        """
        Initialize TextAdapter.

        Args:
            gateway: SignalGateway instance
            source_name: Source identifier (telegram, slack, discord, etc.)
            default_priority: Default priority for messages
            auto_tag: Automatically add source tags
        """
        self.gateway = gateway
        self.source_name = source_name
        self.default_priority = default_priority
        self.auto_tag = auto_tag

        # Track conversation sequences
        self._conversation_sequences: Dict[str, str] = {}
        self._conversation_counters: Dict[str, int] = {}

    def handle_message(
        self,
        text: str,
        user_id: Optional[str] = None,
        chat_id: Optional[str] = None,
        priority: Optional[int] = None,
        metadata: Optional[Dict[str, Any]] = None,
        tags: Optional[List[str]] = None,
    ) -> SignalEvent:
        """
        Handle incoming text message.

        Args:
            text: Message text
            user_id: User identifier
            chat_id: Chat/conversation identifier
            priority: Message priority (overrides default)
            metadata: Additional metadata
            tags: Custom tags

        Returns:
            Created SignalEvent
        """
        # Build metadata
        msg_metadata = metadata or {}

        if user_id:
            msg_metadata["user_id"] = user_id
        if chat_id:
            msg_metadata["chat_id"] = chat_id

        msg_metadata["source"] = self.source_name

        # Build tags
        msg_tags = tags or []
        if self.auto_tag:
            msg_tags.append(self.source_name)
            msg_tags.append("user_message")

        # Get or create conversation sequence
        sequence_id = None
        if chat_id:
            sequence_id = self._get_conversation_sequence(chat_id)

        # Determine priority
        msg_priority = priority if priority is not None else self.default_priority

        # Push to gateway
        event = self.gateway.push_text(
            text=text,
            priority=msg_priority,
            metadata=msg_metadata,
            sequence_id=sequence_id,
        )

        # Add tags to routing (post-creation)
        if msg_tags:
            event.routing.tags.extend(msg_tags)

        return event

    def handle_command(
        self,
        command: str,
        args: List[str],
        user_id: Optional[str] = None,
        chat_id: Optional[str] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Handle command message (e.g., /start, /help).

        Args:
            command: Command name (without /)
            args: Command arguments
            user_id: User identifier
            chat_id: Chat identifier
            metadata: Additional metadata

        Returns:
            Created SignalEvent
        """
        # Build command text
        text = f"/{command}"
        if args:
            text += " " + " ".join(args)

        # Build metadata
        cmd_metadata = metadata or {}
        cmd_metadata["command"] = command
        cmd_metadata["args"] = args
        cmd_metadata["is_command"] = True

        # Commands have higher priority
        return self.handle_message(
            text=text,
            user_id=user_id,
            chat_id=chat_id,
            priority=220,
            metadata=cmd_metadata,
            tags=["command", command],
        )

    def _get_conversation_sequence(self, chat_id: str) -> str:
        """
        Get or create conversation sequence ID for a chat.

        Args:
            chat_id: Chat identifier

        Returns:
            Sequence ID for this conversation
        """
        if chat_id not in self._conversation_sequences:
            # Get counter for this chat (starts at 0)
            counter = self._conversation_counters.get(chat_id, 0)
            self._conversation_sequences[chat_id] = f"conv_{self.source_name}_{chat_id}_{counter}"

        return self._conversation_sequences[chat_id]

    def reset_conversation(self, chat_id: str):
        """
        Reset conversation sequence for a chat (start new conversation).

        Args:
            chat_id: Chat identifier
        """
        if chat_id in self._conversation_sequences:
            del self._conversation_sequences[chat_id]
            # Increment counter for next conversation
            self._conversation_counters[chat_id] = self._conversation_counters.get(chat_id, 0) + 1

    def get_active_conversations(self) -> List[str]:
        """
        Get list of active conversation IDs.

        Returns:
            List of chat IDs with active conversations
        """
        return list(self._conversation_sequences.keys())


# ═══════════════════════════════════════════════════════════════════════════════
# PLATFORM-SPECIFIC ADAPTERS
# ═══════════════════════════════════════════════════════════════════════════════

class TelegramAdapter(TextAdapter):
    """
    Specialized adapter for Telegram Bot API.

    Usage:
        from telegram import Update
        from telegram.ext import ContextTypes

        gateway = SignalGateway()
        gateway.initialize()
        adapter = TelegramAdapter(gateway)

        async def handle_telegram_message(update: Update, context: ContextTypes.DEFAULT_TYPE):
            event = adapter.handle_telegram_update(update)
            # Process event...
    """

    def __init__(self, gateway: SignalGateway):
        super().__init__(gateway, source_name="telegram")

    def handle_telegram_update(self, update) -> Optional[SignalEvent]:
        """
        Handle Telegram Update object.

        Args:
            update: telegram.Update object

        Returns:
            SignalEvent or None if not a text message
        """
        if not update.message or not update.message.text:
            return None

        message = update.message
        user = message.from_user
        chat = message.chat

        # Check if command
        if message.text.startswith("/"):
            parts = message.text[1:].split()
            command = parts[0]
            args = parts[1:] if len(parts) > 1 else []

            return self.handle_command(
                command=command,
                args=args,
                user_id=str(user.id),
                chat_id=str(chat.id),
                metadata={
                    "username": user.username,
                    "first_name": user.first_name,
                    "chat_type": chat.type,
                },
            )
        else:
            return self.handle_message(
                text=message.text,
                user_id=str(user.id),
                chat_id=str(chat.id),
                metadata={
                    "username": user.username,
                    "first_name": user.first_name,
                    "chat_type": chat.type,
                    "message_id": message.message_id,
                },
            )


__all__ = [
    "TextAdapter",
    "TelegramAdapter",
]
