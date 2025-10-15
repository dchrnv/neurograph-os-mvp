"""
Utility functions for working with the NeuroGraph OS Event System.
"""
import asyncio
import logging
import yaml
import os
from pathlib import Path
from typing import Dict, Any, Optional, Callable, Awaitable, TypeVar, Type

from .event import Event, EventType, EventCategory
from .event_bus import EventBus
from .global_bus import get_event_bus, start_event_bus, stop_event_bus

logger = logging.getLogger(__name__)

T = TypeVar('T')

class EventSystem:
    """A utility class for working with the event system."""
    
    _instance = None
    _bus: Optional[EventBus] = None
    _config: Dict[str, Any] = {}
    
    def __new__(cls):
        if cls._instance is None:
            cls._instance = super(EventSystem, cls).__new__(cls)
        return cls._instance
    
    @classmethod
    async def initialize(cls, config_path: Optional[str] = None, **kwargs):
        """Initialize the event system with the given configuration."""
        if cls._bus is not None:
            logger.warning("Event system is already initialized")
            return cls._bus
            
        # Load configuration
        cls._config = await cls._load_config(config_path, **kwargs)
        
        # Initialize the event bus
        cls._bus = await start_event_bus(
            max_queue_size=cls._config.get('max_queue_size', 10000),
            worker_count=cls._config.get('worker_count', 4),
            enable_metrics=cls._config.get('enable_metrics', True),
            log_events=cls._config.get('log_events', False)
        )
        
        logger.info(f"Event system initialized with {cls._config.get('worker_count')} workers")
        return cls._bus
    
    @classmethod
    async def shutdown(cls):
        """Shut down the event system."""
        if cls._bus is not None:
            await stop_event_bus()
            cls._bus = None
            logger.info("Event system shut down")
    
    @classmethod
    def get_bus(cls) -> EventBus:
        """Get the event bus instance."""
        if cls._bus is None:
            raise RuntimeError("Event system is not initialized. Call initialize() first.")
        return cls._bus
    
    @classmethod
    async def _load_config(cls, config_path: Optional[str] = None, **overrides) -> Dict[str, Any]:
        """Load configuration from a YAML file."""
        # Default configuration
        config = {
            'max_queue_size': 10000,
            'worker_count': 4,
            'batch_size': 100,
            'max_retries': 3,
            'enable_metrics': True,
            'log_events': False,
            'log_level': 'INFO',
            'publish_timeout': 5.0,
            'shutdown_timeout': 10.0
        }
        
        # Try to load from file if path is provided
        if config_path and os.path.exists(config_path):
            try:
                with open(config_path, 'r') as f:
                    file_config = yaml.safe_load(f)
                    # Merge with default config
                    config.update(file_config.get('default', {}))
                    
                    # Apply environment-specific settings
                    env = os.getenv('ENV', 'development')
                    if env in file_config:
                        config.update(file_config[env])
            except Exception as e:
                logger.warning(f"Failed to load event bus config from {config_path}: {e}")
        
        # Apply any overrides
        config.update(overrides)
        return config

# Convenience functions
async def publish_event(
    event_type: EventType,
    category: EventCategory,
    source: str,
    payload: Optional[Dict[str, Any]] = None,
    **kwargs
) -> None:
    """Publish an event to the event bus."""
    bus = get_event_bus()
    event = Event(
        type=event_type,
        category=category,
        source=source,
        payload=payload or {},
        **kwargs
    )
    await bus.publish(event)

async def subscribe(
    event_type: EventType,
    handler: Callable[[Event], Awaitable[None]],
    filter_func: Optional[Callable[[Event], bool]] = None
) -> None:
    """Subscribe to events of a specific type."""
    bus = get_event_bus()
    await bus.subscribe(event_type, handler, filter_func)

def subscribe_sync(
    event_type: EventType,
    filter_func: Optional[Callable[[Event], bool]] = None
) -> Callable[[Callable[[Event], Awaitable[None]]], Callable[[Event], Awaitable[None]]]:
    """Decorator for synchronous subscription to events."""
    def decorator(handler: Callable[[Event], Awaitable[None]]) -> Callable[[Event], Awaitable[None]]:
        asyncio.create_task(subscribe(event_type, handler, filter_func))
        return handler
    return decorator
