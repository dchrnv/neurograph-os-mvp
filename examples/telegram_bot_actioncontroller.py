#!/usr/bin/env python3
"""
Telegram Bot with ActionController - v0.56.0

Demonstrates full integration:
- Gateway v2.0 (sensory interface)
- ActionController (response generation)
- SignalPipeline (end-to-end orchestration)

Setup:
1. pip install python-telegram-bot
2. export TELEGRAM_BOT_TOKEN="your_token"
3. python examples/telegram_bot_actioncontroller.py
"""

import os
import sys
import asyncio
import logging

try:
    from telegram import Update
    from telegram.ext import Application, CommandHandler, MessageHandler, filters, ContextTypes
except ImportError:
    print("Error: pip install python-telegram-bot")
    sys.exit(1)

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from src.integration import SignalPipeline
from src.action_controller import ActionPriority
from src.action_controller.executors import TextResponseAction, LoggingAction, MetricsAction


# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class ActionControllerTelegramBot:
    """
    Telegram bot powered by ActionController and SignalPipeline.

    Features:
    - Full Gateway v2.0 integration
    - ActionController for response generation
    - Hot path: immediate Telegram responses
    - Cold path: logging, metrics
    - End-to-end signal processing pipeline
    """

    def __init__(self, token: str):
        """
        Initialize bot.

        Args:
            token: Telegram bot token from @BotFather
        """
        self.token = token

        # Create SignalPipeline (Gateway + ActionController)
        self.pipeline = SignalPipeline()

        # Register action types
        self._register_actions()

        # Configure action paths
        self._configure_paths()

        # Get stats
        self.metrics_action = None

        print("=" * 60)
        print("Telegram Bot - ActionController Integration")
        print("=" * 60)
        print()
        print("Features:")
        print("  • SignalPipeline (Gateway → ActionController)")
        print("  • Hot Path: Text responses")
        print("  • Cold Path: Logging, Metrics")
        print("  • Full end-to-end flow")
        print()

    def _register_actions(self):
        """Register available action types."""
        # Hot path: text responses
        self.pipeline.register_action(
            action_type="text_response",
            action_class=TextResponseAction,
            priority=ActionPriority.HIGH,
            tags=["response", "user_facing"]
        )

        # Cold path: logging
        self.pipeline.register_action(
            action_type="logging",
            action_class=LoggingAction,
            priority=ActionPriority.LOW,
            log_file="telegram_bot_actions.log",
            tags=["system", "monitoring"]
        )

        # Cold path: metrics
        self.metrics_action = MetricsAction(
            action_id="metrics_global",
            action_type="metrics",
            priority=ActionPriority.BACKGROUND
        )
        self.pipeline.register_action(
            action_type="metrics",
            action_class=MetricsAction,
            priority=ActionPriority.BACKGROUND,
            tags=["system", "analytics"]
        )

        logger.info("Registered 3 action types")

    def _configure_paths(self):
        """Configure hot/cold path routing."""
        self.pipeline.configure_actions(
            hot_path=["text_response"],  # Immediate responses
            cold_path=["logging", "metrics"]  # Background tasks
        )

        logger.info("Configured action paths")

    async def start_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /start command."""
        welcome_msg = (
            "🤖 *ActionController Bot*\\n\\n"
            "Powered by:\\n"
            "• Gateway v2\\.0 \\- Sensory interface\\n"
            "• ActionController \\- Response engine\\n"
            "• SignalPipeline \\- Full integration\\n\\n"
            "Try:\\n"
            "/stats \\- View statistics\\n"
            "/test \\- Test pipeline\\n\\n"
            "Just chat with me naturally\\! 💬"
        )

        await update.message.reply_text(welcome_msg, parse_mode='MarkdownV2')

    async def stats_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /stats command."""
        # Get pipeline stats
        stats = self.pipeline.get_stats()

        msg = (
            f"📊 *Statistics*\\n\\n"
            f"*Pipeline:*\\n"
            f"• Total processed: {stats['pipeline']['total_processed']}\\n"
            f"• Without Core: {stats['pipeline']['without_core']}\\n\\n"
            f"*Gateway:*\\n"
            f"• Events: {stats['gateway']['total_events']}\\n"
            f"• NeuroTick: {stats['gateway']['neuro_tick']}\\n\\n"
            f"*ActionController:*\\n"
            f"• Total executions: {stats['controller']['total_executions']}\\n"
            f"• Hot path: {stats['controller']['hot_path_executed']}\\n"
            f"• Cold path: {stats['controller']['cold_path_executed']}\\n"
            f"• Failed: {stats['controller']['failed_actions']}"
        )

        await update.message.reply_text(msg, parse_mode='MarkdownV2')

    async def test_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /test command - test full pipeline."""
        await update.message.reply_text("🔄 Testing full pipeline...")

        # Process through pipeline
        result = await self.pipeline.process_text(
            text="This is a test message",
            user_id=str(update.message.from_user.id),
            chat_id=str(update.message.chat.id),
            priority=220,
            metadata={
                "username": update.message.from_user.username or "unknown",
                "is_test": True
            }
        )

        # Show results
        msg = (
            f"✅ *Pipeline Test Complete*\\n\\n"
            f"*Signal Event:*\\n"
            f"• Event ID: `{str(result['signal_event'].event_id)[:16]}\\.\\.\\. `\\n"
            f"• Priority: {result['signal_event'].routing.priority}\\n"
            f"• Vector: `[{result['signal_event'].semantic.vector[0]:.2f}, \\.\\.\\.]`\\n\\n"
            f"*Actions:*\\n"
            f"• Hot path: {result['action_results']['stats']['hot_path_executed']}\\n"
            f"• Cold path: {result['action_results']['stats']['cold_path_queued']}\\n\\n"
            f"*Timing:*\\n"
            f"• Total: {result['stats']['total_time_ms']:.2f}ms"
        )

        await update.message.reply_text(msg, parse_mode='MarkdownV2')

    async def handle_message(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle regular text messages."""
        # Process through SignalPipeline
        result = await self.pipeline.process_text(
            text=update.message.text,
            user_id=str(update.message.from_user.id),
            chat_id=str(update.message.chat.id),
            priority=200,
            metadata={
                "username": update.message.from_user.username or "unknown",
                "message_id": update.message.message_id
            }
        )

        # Extract response from hot path results
        response_text = None
        for action_result in result['action_results']['hot_path_results']:
            if action_result.success and action_result.data:
                response_text = action_result.data.get('response_text')
                break

        # Send response
        if response_text:
            await update.message.reply_text(response_text)
        else:
            # Fallback response
            await update.message.reply_text(
                f"✅ Processed (tick={result['signal_event'].temporal.neuro_tick})"
            )

    async def error_handler(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle errors."""
        logger.error(f"Error: {context.error}", exc_info=context.error)

        if update and update.message:
            await update.message.reply_text(
                "❌ Oops! Something went wrong. Try again?"
            )

    def run(self):
        """Run the bot."""
        print("Starting bot...")
        print()

        # Create application
        application = Application.builder().token(self.token).build()

        # Register handlers
        application.add_handler(CommandHandler("start", self.start_command))
        application.add_handler(CommandHandler("stats", self.stats_command))
        application.add_handler(CommandHandler("test", self.test_command))
        application.add_handler(
            MessageHandler(filters.TEXT & ~filters.COMMAND, self.handle_message)
        )

        # Register error handler
        application.add_error_handler(self.error_handler)

        # Run bot
        print("✅ Bot is running with ActionController!")
        print("Press Ctrl+C to stop")
        print()

        application.run_polling(allowed_updates=Update.ALL_TYPES)


def main():
    """Main entry point."""
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
    bot = ActionControllerTelegramBot(token)

    try:
        bot.run()
    except KeyboardInterrupt:
        print("\\n\\nBot stopped by user")
        print(f"Pipeline stats: {bot.pipeline.get_stats()}")


if __name__ == "__main__":
    main()
