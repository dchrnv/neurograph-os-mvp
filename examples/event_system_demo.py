"""
NeuroGraph OS - Event System Demo

This example demonstrates how to use the event system in NeuroGraph OS.
"""
import asyncio
import logging
from datetime import datetime

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# Import event system components
from core.events import (
    Event, EventType, EventCategory, EventPriority,
    EventBus, get_event_bus, start_event_bus, stop_event_bus
)

async def token_created_handler(event):
    """Handle TOKEN_CREATED events."""
    logger.info(f"🔔 [TOKEN_CREATED] Received token: {event.payload['token_id']}")
    logger.info(f"   Content: {event.payload['content']}")
    logger.info(f"   Timestamp: {datetime.fromtimestamp(event.timestamp).isoformat()}")

async def token_updated_handler(event):
    """Handle TOKEN_UPDATED events."""
    logger.info(f"🔄 [TOKEN_UPDATED] Updated token: {event.payload['token_id']}")
    for field, (old_val, new_val) in event.payload['changes'].items():
        logger.info(f"   {field}: {old_val} → {new_val}")

async def error_handler(event):
    """Handle ERROR events."""
    logger.error(f"❌ [ERROR] {event.payload.get('error', 'Unknown error')}")
    if 'traceback' in event.payload:
        logger.debug(f"Traceback: {event.payload['traceback']}")

async def setup_event_handlers(bus):
    """Register event handlers with the event bus."""
    # Subscribe to token events
    await bus.subscribe(EventType.TOKEN_CREATED, token_created_handler)
    await bus.subscribe(EventType.TOKEN_UPDATED, token_updated_handler)
    await bus.subscribe(EventType.ERROR, error_handler)
    logger.info("✅ Event handlers registered")

async def simulate_token_operations():
    """Simulate token operations that generate events."""
    try:
        bus = get_event_bus()
        
        # Simulate token creation
        token_id = "tok_12345"
        await bus.publish(Event(
            type=EventType.TOKEN_CREATED,
            category=EventCategory.TOKEN,
            source="token_service",
            payload={
                "token_id": token_id,
                "content": "example_token",
                "token_type": "example"
            }
        ))
        
        # Simulate token update
        await asyncio.sleep(1)  # Simulate processing time
        await bus.publish(Event(
            type=EventType.TOKEN_UPDATED,
            category=EventCategory.TOKEN,
            source="token_service",
            payload={
                "token_id": token_id,
                "changes": {
                    "content": ("example_token", "updated_token"),
                    "status": ("active", "inactive")
                }
            }
        ))
        
    except Exception as e:
        logger.exception("Error in token operations")
        await bus.publish(Event(
            type=EventType.ERROR,
            category=EventCategory.SYSTEM,
            source="demo_script",
            payload={
                "error": str(e),
                "operation": "simulate_token_operations"
            }
        ))

async def main():
    """Main function to demonstrate the event system."""
    logger.info("🚀 Starting NeuroGraph OS Event System Demo")
    
    # Initialize and start the event bus
    bus = await start_event_bus(
        max_queue_size=1000,
        enable_metrics=True,
        log_events=True
    )
    
    try:
        # Set up event handlers
        await setup_event_handlers(bus)
        
        # Run the demo
        await simulate_token_operations()
        
        # Keep the application running to process events
        await asyncio.sleep(2)
        
        # Show metrics
        metrics = bus.get_metrics()
        logger.info("\n📊 Event Bus Metrics:")
        logger.info(f"Total events published: {metrics.total_published}")
        logger.info(f"Total events processed: {metrics.total_processed}")
        logger.info(f"Active subscriptions: {metrics.active_subscriptions}")
        
    except Exception as e:
        logger.exception("Error in main")
    finally:
        # Clean up
        await stop_event_bus()
        logger.info("\n🛑 Event System Demo Completed")

if __name__ == "__main__":
    asyncio.run(main())
