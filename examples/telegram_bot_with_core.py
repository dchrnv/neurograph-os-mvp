#!/usr/bin/env python3
"""
Telegram Bot with Rust Core - v0.57.0

Full integration: Gateway v2.0 → Rust SignalSystem → ActionController

This bot demonstrates the complete NeuroGraph OS pipeline:
- Gateway encodes messages to 8D vectors
- Rust Core processes signals (pattern matching, novelty detection)
- ActionController generates responses based on Core results

Setup:
1. Build Rust Core: maturin develop --features python-bindings --release
2. pip install python-telegram-bot
3. export TELEGRAM_BOT_TOKEN="your_token"
4. python examples/telegram_bot_with_core.py
"""

import os
import sys
import asyncio
import logging

# Add Rust Core to path
sys.path.insert(0, 'src/core_rust/target/release')

try:
    from telegram import Update
    from telegram.ext import Application, CommandHandler, MessageHandler, filters, ContextTypes
except ImportError:
    print("Error: pip install python-telegram-bot")
    sys.exit(1)

try:
    import _core
except ImportError:
    print("Error: Rust Core not built")
    print("Run: cd src/core_rust && maturin develop --features python-bindings --release")
    sys.exit(1)

# Add project root to path
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


class CorePoweredTelegramBot:
    """
    Telegram bot powered by Rust Core + ActionController.

    Architecture:
        Telegram Message
              ↓
        SignalPipeline.process_text()
              ├─→ Gateway → SignalEvent (8D encoding)
              ├─→ Rust Core → ProcessingResult (pattern matching)
              └─→ ActionController → Response
              ↓
        Telegram Response
    """

    def __init__(self, token: str):
        """Initialize bot with Rust Core."""
        self.token = token

        # Create Rust Core
        self.core = _core.SignalSystem()
        logger.info("✓ Rust Core created")

        # Create SignalPipeline with Core
        self.pipeline = SignalPipeline(core_system=self.core)
        logger.info("✓ Pipeline created with Rust Core")

        # Register actions
        self._register_actions()
        self._configure_paths()

        print("=" * 60)
        print("Telegram Bot - Rust Core Integration (v0.57.0)")
        print("=" * 60)
        print()
        print("Features:")
        print("  • Rust SignalSystem for event processing")
        print("  • Pattern matching & novelty detection")
        print("  • ActionController for response generation")
        print("  • Full end-to-end pipeline")
        print()
        print("Performance:")
        print("  • Core processing: <1μs")
        print("  • Total latency: <1ms")
        print("  • Throughput: 5000+ msg/sec")
        print()

    def _register_actions(self):
        """Register action types."""
        self.pipeline.register_action(
            action_type="text_response",
            action_class=TextResponseAction,
            priority=ActionPriority.HIGH
        )

        self.pipeline.register_action(
            action_type="logging",
            action_class=LoggingAction,
            priority=ActionPriority.LOW,
            log_file="telegram_core_bot.log"
        )

        self.pipeline.register_action(
            action_type="metrics",
            action_class=MetricsAction,
            priority=ActionPriority.BACKGROUND
        )

        logger.info("✓ Registered 3 action types")

    def _configure_paths(self):
        """Configure action paths."""
        self.pipeline.configure_actions(
            hot_path=["text_response"],
            cold_path=["logging", "metrics"]
        )
        logger.info("✓ Configured action paths")

    async def start_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /start command."""
        welcome_msg = (
            "🤖 *Rust Core\\-Powered Bot*\\n\\n"
            "Full NeuroGraph OS stack:\\n"
            "• Gateway v2\\.0 \\(8D encoding\\)\\n"
            "• Rust SignalSystem \\(pattern matching\\)\\n"
            "• ActionController \\(responses\\)\\n\\n"
            "*Commands:*\\n"
            "/stats \\- View statistics\\n"
            "/core \\- Core information\\n"
            "/test \\- Test full pipeline\\n\\n"
            "Just chat naturally\\! 💬"
        )

        await update.message.reply_text(welcome_msg, parse_mode='MarkdownV2')

    async def stats_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /stats command."""
        # Pipeline stats
        pipeline_stats = self.pipeline.get_stats()

        # Core stats
        core_stats = self.core.get_stats()

        msg = (
            f"📊 *Statistics*\\n\\n"
            f"*Pipeline:*\\n"
            f"• Processed: {pipeline_stats['pipeline']['total_processed']}\\n"
            f"• With Core: {pipeline_stats['pipeline']['with_core']}\\n\\n"
            f"*Rust Core:*\\n"
            f"• Total events: {core_stats['total_events']}\\n"
            f"• Avg time: {core_stats['avg_processing_time_us']:.2f}μs\\n"
            f"• Subscribers: {self.core.subscriber_count()}\\n\\n"
            f"*ActionController:*\\n"
            f"• Executions: {pipeline_stats['controller']['total_executions']}\\n"
            f"• Hot path: {pipeline_stats['controller']['hot_path_executed']}\\n"
            f"• Cold path: {pipeline_stats['controller']['cold_path_executed']}"
        )

        await update.message.reply_text(msg, parse_mode='MarkdownV2')

    async def core_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /core command - show Core information."""
        core_stats = self.core.get_stats()

        msg = (
            f"⚙️ *Rust Core Info*\\n\\n"
            f"*Status:* Active ✅\\n"
            f"*Performance:*\\n"
            f"• Avg processing: {core_stats['avg_processing_time_us']:.2f}μs\\n"
            f"• Total events: {core_stats['total_events']}\\n\\n"
            f"*Capabilities:*\\n"
            f"• Pattern matching\\n"
            f"• Novelty detection\\n"
            f"• Neighbor finding\\n"
            f"• Action triggering\\n"
            f"• Event subscriptions"
        )

        await update.message.reply_text(msg, parse_mode='MarkdownV2')

    async def test_command(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle /test command."""
        await update.message.reply_text("🔄 Testing full pipeline with Core...")

        # Process through full pipeline
        result = await self.pipeline.process_text(
            text="This is a test message for Rust Core",
            user_id=str(update.message.from_user.id),
            chat_id=str(update.message.chat.id),
            priority=220
        )

        # Extract Core result
        core_result = result['processing_result']

        msg = (
            f"✅ *Pipeline Test Complete*\\n\\n"
            f"*Signal Event:*\\n"
            f"• Event ID: `{str(result['signal_event'].event_id)[:16]}\\.\\.\\. `\\n"
            f"• Priority: {result['signal_event'].routing.priority}\\n\\n"
            f"*Rust Core Processing:*\\n"
            f"• From Core: {core_result['from_core']} ✅\\n"
            f"• Token ID: {core_result['token_id']}\\n"
            f"• Is Novel: {core_result['is_novel']}\\n"
            f"• Neighbors: {len(core_result['neighbors'])}\\n"
            f"• Processing: {core_result['processing_time_us']}μs\\n\\n"
            f"*Actions:*\\n"
            f"• Hot path: {result['action_results']['stats']['hot_path_executed']}\\n"
            f"• Cold path: {result['action_results']['stats']['cold_path_queued']}\\n\\n"
            f"*Timing:*\\n"
            f"• Total: {result['stats']['total_time_ms']:.2f}ms\\n"
            f"• Core: {result['stats']['core_time_ms']:.2f}ms"
        )

        await update.message.reply_text(msg, parse_mode='MarkdownV2')

    async def handle_message(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle regular messages."""
        # Process through full pipeline
        result = await self.pipeline.process_text(
            text=update.message.text,
            user_id=str(update.message.from_user.id),
            chat_id=str(update.message.chat.id),
            priority=200
        )

        # Extract Core processing result
        core_result = result['processing_result']

        # Generate response with Core info
        response_lines = []

        # Add Core info if novel
        if core_result.get('is_novel'):
            response_lines.append("🆕 Novel pattern detected!")

        # Add neighbor info if any
        if core_result.get('neighbors'):
            response_lines.append(f"🔗 Found {len(core_result['neighbors'])} similar patterns")

        # Add basic response
        response_lines.append(f"✅ Processed (Core: {core_result['processing_time_us']}μs)")

        response_text = "\\n".join(response_lines)

        await update.message.reply_text(response_text, parse_mode='MarkdownV2')

    async def error_handler(self, update: Update, context: ContextTypes.DEFAULT_TYPE):
        """Handle errors."""
        logger.error(f"Error: {context.error}", exc_info=context.error)

        if update and update.message:
            await update.message.reply_text(
                "❌ Error processing message. Please try again."
            )

    def run(self):
        """Run the bot."""
        print("Starting bot with Rust Core...")
        print()

        # Create application
        application = Application.builder().token(self.token).build()

        # Register handlers
        application.add_handler(CommandHandler("start", self.start_command))
        application.add_handler(CommandHandler("stats", self.stats_command))
        application.add_handler(CommandHandler("core", self.core_command))
        application.add_handler(CommandHandler("test", self.test_command))
        application.add_handler(
            MessageHandler(filters.TEXT & ~filters.COMMAND, self.handle_message)
        )

        # Error handler
        application.add_error_handler(self.error_handler)

        # Run
        print("✅ Bot running with Rust Core integration!")
        print("Press Ctrl+C to stop")
        print()

        application.run_polling(allowed_updates=Update.ALL_TYPES)


def main():
    """Main entry point."""
    # Get token
    token = os.getenv("TELEGRAM_BOT_TOKEN")

    if not token:
        print("Error: TELEGRAM_BOT_TOKEN not set")
        print()
        print("Steps:")
        print("1. Talk to @BotFather on Telegram")
        print("2. Create bot with /newbot")
        print("3. Copy token")
        print("4. export TELEGRAM_BOT_TOKEN='your_token'")
        print("5. Run this script")
        sys.exit(1)

    # Create and run bot
    bot = CorePoweredTelegramBot(token)

    try:
        bot.run()
    except KeyboardInterrupt:
        print("\\n\\nBot stopped")
        print(f"Final stats: {bot.pipeline.get_stats()}")


if __name__ == "__main__":
    main()
