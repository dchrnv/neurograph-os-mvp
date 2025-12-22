#!/usr/bin/env python3
"""
Simple Telegram Bot Example - Gateway v2.0

Demonstrates:
- TelegramAdapter integration
- Message handling
- Command handling
- Subscription filters for responses
- End-to-end flow: Telegram → Gateway → (Future: Core → Response)

Setup:
1. Install python-telegram-bot:
   pip install python-telegram-bot

2. Get bot token from @BotFather on Telegram

3. Set environment variable:
   export TELEGRAM_BOT_TOKEN="your_token_here"

4. Run:
   python examples/telegram_bot_simple.py

Usage in Telegram:
- /start - Start conversation
- /help - Show help
- /stats - Show Gateway statistics
- Any text - Send to Gateway for processing
"""

import os
import sys
import asyncio
from datetime import datetime

# Check if telegram library is available
try:
    from telegram import Update
    from telegram.ext import (
        Application,
        CommandHandler,
        MessageHandler,
        filters,
        ContextTypes,
    )
except ImportError:
    print("Error: python-telegram-bot required")
    print("Install: pip install python-telegram-bot")
    sys.exit(1)

# Add project to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from src.gateway import SignalGateway
from src.gateway.adapters import TelegramAdapter
from src.gateway.filters import SubscriptionFilter
from src.gateway.filters.examples import telegram_user_messages_filter


class TelegramBot:
    """
    Simple Telegram bot powered by Gateway v2.0.

    Flow:
    1. User sends message to Telegram
    2. TelegramAdapter converts to SignalEvent
    3. Gateway processes and encodes to 8D
    4. (Future: Core processes and triggers actions)
    5. Bot responds
    """

    def __init__(self, token: str):
        """
        Initialize bot.

        Args:
            token: Telegram bot token from @BotFather
        """
        self.token = token

        # Initialize Gateway
        self.gateway = SignalGateway()
        self.gateway.initialize()

        # Initialize Telegram adapter
        self.adapter = TelegramAdapter(self.gateway)

        # Create subscription filter for user messages
        self.message_filter = telegram_user_messages_filter()

        # Track processed events (for demo purposes)
        self.processed_events = []

    async def start_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /start command"""
        # Process through Gateway
        event = self.adapter.handle_telegram_update(update)
        self.processed_events.append(event)

        # Respond
        welcome_msg = (
            "🤖 *NeuroGraph Gateway Bot*\n\n"
            "Connected to Gateway v2.0!\n\n"
            "Commands:\n"
            "/start - Show this message\n"
            "/help - Get help\n"
            "/stats - Show statistics\n"
            "/reset - Reset conversation\n\n"
            "Just send me any text and I'll process it through the Gateway! 🚀"
        )

        await update.message.reply_text(
            welcome_msg,
            parse_mode='Markdown'
        )

    async def help_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /help command"""
        event = self.adapter.handle_telegram_update(update)
        self.processed_events.append(event)

        help_msg = (
            "*Gateway v2.0 Features:*\n\n"
            "✅ Text encoding (TF-IDF)\n"
            "✅ 8D semantic vectors\n"
            "✅ Conversation tracking\n"
            "✅ Priority & urgency\n"
            "✅ Event filtering\n"
            "✅ Metadata extraction\n\n"
            "Send me a message and I'll show you how it's processed!"
        )

        await update.message.reply_text(help_msg, parse_mode='Markdown')

    async def stats_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /stats command"""
        event = self.adapter.handle_telegram_update(update)
        self.processed_events.append(event)

        # Get Gateway stats
        stats = self.gateway.get_stats()

        # Get conversation info
        chat_id = str(update.message.chat.id)
        active_convs = self.adapter.get_active_conversations()

        stats_msg = (
            f"📊 *Gateway Statistics*\n\n"
            f"Total Events: {stats['total_events']}\n"
            f"NeuroTick: {stats['neuro_tick']}\n"
            f"Registered Sensors: {stats['registered_sensors']}\n"
            f"Active Conversations: {len(active_convs)}\n\n"
            f"*Your Conversation:*\n"
            f"Chat ID: `{chat_id}`\n"
            f"Sequence: `{event.temporal.sequence_id}`\n"
            f"Messages: {len([e for e in self.processed_events if str(e.payload.metadata.get('chat_id')) == chat_id])}"
        )

        await update.message.reply_text(stats_msg, parse_mode='Markdown')

    async def reset_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /reset command"""
        chat_id = str(update.message.chat.id)

        # Reset conversation
        self.adapter.reset_conversation(chat_id)

        await update.message.reply_text(
            "🔄 Conversation reset! Starting new thread."
        )

    async def handle_message(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle regular text messages"""
        # Process through Gateway
        event = self.adapter.handle_telegram_update(update)
        self.processed_events.append(event)

        # Extract info for response
        text = event.payload.data
        vector = event.semantic.vector
        priority = event.routing.priority
        urgency = event.energy.urgency
        neuro_tick = event.temporal.neuro_tick

        # Format 8D vector (show first 4 dimensions)
        vector_str = f"[{vector[0]:.2f}, {vector[1]:.2f}, {vector[2]:.2f}, {vector[3]:.2f}, ...]"

        # Build response
        response = (
            f"✅ *Message processed!*\n\n"
            f"📝 Text: _{text[:50]}{'...' if len(text) > 50 else ''}_\n\n"
            f"*Gateway Processing:*\n"
            f"• 8D Vector: `{vector_str}`\n"
            f"• Priority: {priority}\n"
            f"• Urgency: {urgency:.2f}\n"
            f"• NeuroTick: {neuro_tick}\n"
            f"• Encoding: {event.semantic.encoding_method}\n\n"
            f"Event ID: `{event.event_id[:16]}...`"
        )

        # Check if matches filter
        if self.message_filter.matches(event):
            response += "\n\n✨ *Matched subscription filter!*"

        await update.message.reply_text(response, parse_mode='Markdown')

    async def error_handler(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle errors"""
        print(f"Error: {context.error}")

        if update and update.message:
            await update.message.reply_text(
                "❌ Oops! Something went wrong. Try again?"
            )

    def run(self):
        """Run the bot"""
        print("=" * 60)
        print("NeuroGraph Gateway - Telegram Bot")
        print("=" * 60)
        print()
        print("Gateway initialized:")
        print(f"  Sensors: {self.gateway.get_stats()['registered_sensors']}")
        print(f"  Adapter: TelegramAdapter")
        print()
        print("Starting bot...")
        print()

        # Create application
        application = Application.builder().token(self.token).build()

        # Register handlers
        application.add_handler(CommandHandler("start", self.start_command))
        application.add_handler(CommandHandler("help", self.help_command))
        application.add_handler(CommandHandler("stats", self.stats_command))
        application.add_handler(CommandHandler("reset", self.reset_command))
        application.add_handler(
            MessageHandler(filters.TEXT & ~filters.COMMAND, self.handle_message)
        )

        # Register error handler
        application.add_error_handler(self.error_handler)

        # Run bot
        print("✅ Bot is running!")
        print("Press Ctrl+C to stop")
        print()

        application.run_polling(allowed_updates=Update.ALL_TYPES)


def main():
    """Main entry point"""
    # Get token from environment
    token = os.getenv("TELEGRAM_BOT_TOKEN")

    if not token:
        print("Error: TELEGRAM_BOT_TOKEN environment variable not set")
        print()
        print("Steps:")
        print("1. Talk to @BotFather on Telegram")
        print("2. Create a new bot with /newbot")
        print("3. Copy the token")
        print("4. Set environment variable:")
        print("   export TELEGRAM_BOT_TOKEN='your_token_here'")
        print("5. Run this script again")
        sys.exit(1)

    # Create and run bot
    bot = TelegramBot(token)

    try:
        bot.run()
    except KeyboardInterrupt:
        print("\n\nBot stopped by user")
        print(f"Total events processed: {len(bot.processed_events)}")


if __name__ == "__main__":
    main()
