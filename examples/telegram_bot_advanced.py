#!/usr/bin/env python3
"""
Advanced Telegram Bot Example - Gateway v2.0 with Subscriptions

Demonstrates:
- TelegramAdapter integration
- Subscription filters for event routing
- Multiple handlers (analytics, logging, action triggers)
- High-priority message detection
- Sentiment analysis integration
- Full end-to-end flow

Setup:
1. pip install python-telegram-bot
2. export TELEGRAM_BOT_TOKEN="your_token"
3. python examples/telegram_bot_advanced.py
"""

import os
import sys
import asyncio
from datetime import datetime
from typing import Dict, List

try:
    from telegram import Update
    from telegram.ext import Application, CommandHandler, MessageHandler, filters, ContextTypes
except ImportError:
    print("Error: pip install python-telegram-bot")
    sys.exit(1)

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from src.gateway import SignalGateway, EncoderType
from src.gateway.adapters import TelegramAdapter, SystemAdapter
from src.gateway.filters import SubscriptionFilter
from src.gateway.filters.examples import (
    telegram_user_messages_filter,
    telegram_high_priority_filter,
)
from src.gateway.models import SignalEvent


class EventSubscriber:
    """Base class for event subscribers"""

    def __init__(self, name: str, filter_spec: SubscriptionFilter):
        self.name = name
        self.filter = filter_spec
        self.events_handled = 0

    def matches(self, event: SignalEvent) -> bool:
        """Check if event matches this subscriber's filter"""
        return self.filter.matches(event)

    async def handle(self, event: SignalEvent, bot_app):
        """Handle matched event (override in subclass)"""
        self.events_handled += 1


class AnalyticsSubscriber(EventSubscriber):
    """Tracks analytics for all messages"""

    def __init__(self):
        super().__init__(
            "Analytics",
            SubscriptionFilter({"event_type": {"$wildcard": "signal.input.*"}})
        )
        self.total_messages = 0
        self.total_commands = 0
        self.users = set()

    async def handle(self, event: SignalEvent, bot_app):
        await super().handle(event, bot_app)

        self.total_messages += 1

        # Track users
        if "user_id" in event.payload.metadata:
            self.users.add(event.payload.metadata["user_id"])

        # Track commands
        if event.payload.metadata.get("is_command"):
            self.total_commands += 1

        print(f"📊 [Analytics] Total: {self.total_messages} msgs, {len(self.users)} users")


class HighPrioritySubscriber(EventSubscriber):
    """Handles high-priority messages"""

    def __init__(self, bot_instance):
        super().__init__(
            "HighPriority",
            telegram_high_priority_filter()
        )
        self.bot_instance = bot_instance

    async def handle(self, event: SignalEvent, bot_app):
        await super().handle(event, bot_app)

        chat_id = event.payload.metadata.get("chat_id")
        if not chat_id:
            return

        print(f"🔥 [HighPriority] Urgent message detected in chat {chat_id}")

        # Send notification to user
        await bot_app.bot.send_message(
            chat_id=int(chat_id),
            text="⚡ *High priority message detected!* Your message is being processed with urgency.",
            parse_mode='Markdown'
        )


class SentimentSubscriber(EventSubscriber):
    """Analyzes sentiment and responds accordingly"""

    def __init__(self, bot_instance):
        super().__init__(
            "Sentiment",
            SubscriptionFilter({"source.modality": "text"})
        )
        self.bot_instance = bot_instance
        self.sentiment_counts = {"positive": 0, "negative": 0, "neutral": 0}

    async def handle(self, event: SignalEvent, bot_app):
        await super().handle(event, bot_app)

        # Get polarity from semantic vector (dimension 0)
        # Note: This assumes TEXT_TFIDF encoding, not SENTIMENT_SIMPLE
        # For real sentiment, would need to use sentiment encoder
        polarity = event.semantic.vector[0] if event.semantic.vector else 0.5

        # Classify
        if polarity > 0.6:
            sentiment = "positive"
            emoji = "😊"
        elif polarity < 0.4:
            sentiment = "negative"
            emoji = "😔"
        else:
            sentiment = "neutral"
            emoji = "😐"

        self.sentiment_counts[sentiment] += 1

        print(f"💭 [Sentiment] {sentiment} ({polarity:.2f})")

        # Respond to very positive/negative messages
        chat_id = event.payload.metadata.get("chat_id")
        if chat_id and (polarity > 0.8 or polarity < 0.2):
            if polarity > 0.8:
                msg = f"{emoji} I sense very positive energy in your message!"
            else:
                msg = f"{emoji} I detect some concern. How can I help?"

            await bot_app.bot.send_message(
                chat_id=int(chat_id),
                text=msg
            )


class LoggingSubscriber(EventSubscriber):
    """Logs all events to file"""

    def __init__(self, log_file: str = "gateway_events.log"):
        super().__init__(
            "Logging",
            SubscriptionFilter({"event_type": {"$wildcard": "signal.*"}})
        )
        self.log_file = log_file

    async def handle(self, event: SignalEvent, bot_app):
        await super().handle(event, bot_app)

        log_entry = (
            f"{datetime.now().isoformat()} | "
            f"Event: {event.event_id[:8]} | "
            f"Type: {event.event_type} | "
            f"Tick: {event.temporal.neuro_tick} | "
            f"Text: {str(event.payload.data)[:50]}\n"
        )

        with open(self.log_file, "a") as f:
            f.write(log_entry)


class AdvancedTelegramBot:
    """
    Advanced Telegram bot with subscription system.

    Features:
    - Multiple event subscribers
    - Analytics tracking
    - High-priority detection
    - Sentiment analysis
    - Event logging
    - Subscription filter demonstration
    """

    def __init__(self, token: str):
        self.token = token

        # Initialize Gateway
        self.gateway = SignalGateway()
        self.gateway.initialize()

        # Initialize adapters
        self.telegram_adapter = TelegramAdapter(self.gateway)
        self.system_adapter = SystemAdapter(self.gateway)

        # Initialize subscribers
        self.subscribers: List[EventSubscriber] = []
        self._init_subscribers()

        print("=" * 60)
        print("Advanced Telegram Bot - Gateway v2.0")
        print("=" * 60)
        print()
        print(f"Initialized {len(self.subscribers)} subscribers:")
        for sub in self.subscribers:
            print(f"  • {sub.name}")
        print()

    def _init_subscribers(self):
        """Initialize event subscribers"""
        self.subscribers = [
            AnalyticsSubscriber(),
            HighPrioritySubscriber(self),
            SentimentSubscriber(self),
            LoggingSubscriber(),
        ]

    async def notify_subscribers(self, event: SignalEvent, bot_app):
        """Notify all matching subscribers about event"""
        for subscriber in self.subscribers:
            if subscriber.matches(event):
                try:
                    await subscriber.handle(event, bot_app)
                except Exception as e:
                    print(f"Error in subscriber {subscriber.name}: {e}")

    async def start_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /start"""
        event = self.telegram_adapter.handle_telegram_update(update)
        await self.notify_subscribers(event, context.application)

        msg = (
            "🤖 *Advanced NeuroGraph Bot*\n\n"
            "Features:\n"
            "• Real-time event processing\n"
            "• Sentiment detection\n"
            "• Priority routing\n"
            "• Analytics tracking\n"
            "• Subscription filters\n\n"
            "Try:\n"
            "/stats - View statistics\n"
            "/subscribers - List active subscribers\n"
            "/priority <text> - Send high-priority message\n\n"
            "Just chat with me naturally! 💬"
        )

        await update.message.reply_text(msg, parse_mode='Markdown')

    async def stats_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /stats"""
        event = self.telegram_adapter.handle_telegram_update(update)
        await self.notify_subscribers(event, context.application)

        # Gateway stats
        gw_stats = self.gateway.get_stats()

        # Subscriber stats
        analytics = next((s for s in self.subscribers if s.name == "Analytics"), None)
        sentiment = next((s for s in self.subscribers if s.name == "Sentiment"), None)

        msg = (
            f"📊 *Gateway Statistics*\n\n"
            f"*Gateway:*\n"
            f"• Events: {gw_stats['total_events']}\n"
            f"• NeuroTick: {gw_stats['neuro_tick']}\n"
            f"• Sensors: {gw_stats['registered_sensors']}\n\n"
        )

        if analytics:
            msg += (
                f"*Analytics:*\n"
                f"• Total messages: {analytics.total_messages}\n"
                f"• Commands: {analytics.total_commands}\n"
                f"• Unique users: {len(analytics.users)}\n\n"
            )

        if sentiment:
            total_sentiment = sum(sentiment.sentiment_counts.values())
            if total_sentiment > 0:
                msg += (
                    f"*Sentiment:*\n"
                    f"• Positive: {sentiment.sentiment_counts['positive']}\n"
                    f"• Neutral: {sentiment.sentiment_counts['neutral']}\n"
                    f"• Negative: {sentiment.sentiment_counts['negative']}\n"
                )

        await update.message.reply_text(msg, parse_mode='Markdown')

    async def subscribers_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /subscribers"""
        event = self.telegram_adapter.handle_telegram_update(update)
        await self.notify_subscribers(event, context.application)

        msg = f"*Active Subscribers ({len(self.subscribers)}):*\n\n"

        for sub in self.subscribers:
            msg += f"📌 *{sub.name}*\n"
            msg += f"  Events handled: {sub.events_handled}\n"
            msg += f"  Filter: `{sub.filter.filter_dict}`\n\n"

        await update.message.reply_text(msg, parse_mode='Markdown')

    async def priority_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /priority - send high-priority message"""
        if not context.args:
            await update.message.reply_text("Usage: /priority <your message>")
            return

        text = " ".join(context.args)
        chat_id = str(update.message.chat.id)
        user_id = str(update.message.from_user.id)

        # Send as high-priority
        event = self.telegram_adapter.handle_message(
            text=text,
            user_id=user_id,
            chat_id=chat_id,
            priority=220,  # High priority
            tags=["urgent", "user_priority"]
        )

        await self.notify_subscribers(event, context.application)

        await update.message.reply_text(
            f"⚡ Sent with high priority (priority=220)\n"
            f"Event: `{event.event_id[:16]}...`",
            parse_mode='Markdown'
        )

    async def handle_message(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle regular messages"""
        event = self.telegram_adapter.handle_telegram_update(update)
        await self.notify_subscribers(event, context.application)

        # Simple acknowledgment
        vector_summary = f"[{event.semantic.vector[0]:.2f}, {event.semantic.vector[1]:.2f}, ...]"

        await update.message.reply_text(
            f"✅ Processed (tick={event.temporal.neuro_tick}, vec={vector_summary})"
        )

    async def error_handler(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle errors"""
        print(f"Error: {context.error}")

        if update and update.message:
            await update.message.reply_text("❌ Error occurred. Try again?")

    def run(self):
        """Run the bot"""
        application = Application.builder().token(self.token).build()

        # Register handlers
        application.add_handler(CommandHandler("start", self.start_command))
        application.add_handler(CommandHandler("stats", self.stats_command))
        application.add_handler(CommandHandler("subscribers", self.subscribers_command))
        application.add_handler(CommandHandler("priority", self.priority_command))
        application.add_handler(
            MessageHandler(filters.TEXT & ~filters.COMMAND, self.handle_message)
        )

        application.add_error_handler(self.error_handler)

        print("✅ Bot is running with subscription system!")
        print("Press Ctrl+C to stop\n")

        application.run_polling(allowed_updates=Update.ALL_TYPES)


def main():
    token = os.getenv("TELEGRAM_BOT_TOKEN")

    if not token:
        print("Error: TELEGRAM_BOT_TOKEN not set")
        print("Steps:")
        print("1. Talk to @BotFather")
        print("2. Create bot with /newbot")
        print("3. export TELEGRAM_BOT_TOKEN='token'")
        sys.exit(1)

    bot = AdvancedTelegramBot(token)

    try:
        bot.run()
    except KeyboardInterrupt:
        print("\n\nBot stopped")
        print("\nSubscriber statistics:")
        for sub in bot.subscribers:
            print(f"  {sub.name}: {sub.events_handled} events")


if __name__ == "__main__":
    main()
