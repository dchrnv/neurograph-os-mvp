"""
Tests for the Event System Demo.
"""
import asyncio
import pytest
from datetime import datetime
from unittest.mock import AsyncMock, patch, MagicMock

# Import the module to test
import event_system_demo as demo
from core.events import Event, EventType, EventCategory

@pytest.fixture
def event_bus():
    """Fixture that provides a mock event bus."""
    bus = AsyncMock()
    with patch('event_system_demo.get_event_bus', return_value=bus):
        with patch('event_system_demo.start_event_bus', return_value=bus):
            with patch('event_system_demo.stop_event_bus'):
                yield bus

@pytest.fixture
def mock_logger():
    """Fixture that mocks the logger."""
    with patch('event_system_demo.logger') as mock_logger:
        yield mock_logger

@pytest.mark.asyncio
async def test_token_created_handler():
    """Test the token_created_handler function."""
    # Create a test event
    test_event = Event(
        type=EventType.TOKEN_CREATED,
        category=EventCategory.TOKEN,
        source="test_source",
        payload={
            "token_id": "test_token_123",
            "content": "test_content",
            "timestamp": 1672531200  # 2023-01-01 00:00:00
        }
    )
    
    # Call the handler
    await demo.token_created_handler(test_event)
    
    # Verify the handler processes the event correctly
    # (We'll verify logging in a separate test with mock_logger)

@pytest.mark.asyncio
async def test_token_updated_handler():
    """Test the token_updated_handler function."""
    # Create a test event
    test_event = Event(
        type=EventType.TOKEN_UPDATED,
        category=EventCategory.TOKEN,
        source="test_source",
        payload={
            "token_id": "test_token_123",
            "changes": {
                "content": ("old_content", "new_content"),
                "status": ("active", "inactive")
            }
        }
    )
    
    # Call the handler
    await demo.token_updated_handler(test_event)

@pytest.mark.asyncio
async def test_error_handler():
    """Test the error_handler function."""
    # Create a test error event
    test_event = Event(
        type=EventType.ERROR,
        category=EventCategory.SYSTEM,
        source="test_source",
        payload={
            "error": "Test error message",
            "traceback": "Traceback (most recent call last):\n  File ..."
        }
    )
    
    # Call the handler
    await demo.error_handler(test_event)

@pytest.mark.asyncio
async def test_setup_event_handlers(event_bus):
    """Test the setup_event_handlers function."""
    # Call the setup function
    await demo.setup_event_handlers(event_bus)
    
    # Verify the correct subscriptions were made
    event_bus.subscribe.assert_any_await(EventType.TOKEN_CREATED, demo.token_created_handler)
    event_bus.subscribe.assert_any_await(EventType.TOKEN_UPDATED, demo.token_updated_handler)
    event_bus.subscribe.assert_any_await(EventType.ERROR, demo.error_handler)

@pytest.mark.asyncio
async def test_simulate_token_operations(event_bus):
    """Test the simulate_token_operations function."""
    # Call the function
    await demo.simulate_token_operations()
    
    # Verify the correct events were published
    event_bus.publish.assert_any_await()
    
    # Get all calls to publish
    publish_calls = [call[0][0] for call in event_bus.publish.await_args_list]
    
    # Verify TOKEN_CREATED event
    create_event = publish_calls[0]
    assert create_event.type == EventType.TOKEN_CREATED
    assert create_event.category == EventCategory.TOKEN
    assert create_event.payload["token_id"] == "tok_12345"
    
    # Verify TOKEN_UPDATED event
    update_event = publish_calls[1]
    assert update_event.type == EventType.TOKEN_UPDATED
    assert update_event.category == EventCategory.TOKEN
    assert "changes" in update_event.payload

@pytest.mark.asyncio
async def test_main_success(event_bus, mock_logger):
    """Test the main function with successful execution."""
    # Mock the event bus methods
    event_bus.get_metrics.return_value = MagicMock(
        total_published=2,
        total_processed=2,
        active_subscriptions=3
    )
    
    # Run the main function
    await demo.main()
    
    # Verify the event bus was started and stopped
    demo.start_event_bus.assert_called_once()
    demo.stop_event_bus.assert_awaited_once()
    
    # Verify metrics were logged
    mock_logger.info.assert_any_call("📊 Event Bus Metrics:")
    mock_logger.info.assert_any_call("Total events published: 2")
    mock_logger.info.assert_any_call("Total events processed: 2")
    mock_logger.info.assert_any_call("Active subscriptions: 3")

@pytest.mark.asyncio
async def test_main_with_exception(event_bus, mock_logger):
    """Test the main function with an exception."""
    # Make simulate_token_operations raise an exception
    with patch('event_system_demo.simulate_token_operations', 
              side_effect=Exception("Test error")):
        # Run the main function
        await demo.main()
    
    # Verify error was logged
    mock_logger.exception.assert_called_with("Error in main")
    
    # Verify the event bus was still stopped
    demo.stop_event_bus.assert_awaited_once()

if __name__ == "__main__":
    pytest.main(["-v", __file__])
