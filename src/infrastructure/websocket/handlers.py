"""
WebSocket message handlers for different message types.
"""

from typing import Dict, Any, Callable, Awaitable
from uuid import UUID

from .server import WebSocketMessage, WebSocketConnectionManager
from src.infrastructure.config import ConfigLoader
from src.infrastructure.persistence.database import DatabaseFactory
from src.infrastructure.persistence.repositories import RepositoryFactory
from src.core.utils.logger import get_logger

logger = get_logger(__name__)

# Type alias for message handler
MessageHandler = Callable[[str, WebSocketMessage, WebSocketConnectionManager], Awaitable[None]]


class MessageHandlerRegistry:
    """Registry for WebSocket message handlers."""
    
    def __init__(self):
        """Initialize handler registry."""
        self.handlers: Dict[str, MessageHandler] = {}
        self._register_default_handlers()
    
    def register(self, message_type: str, handler: MessageHandler):
        """Register a message handler."""
        self.handlers[message_type] = handler
        logger.debug(f"Registered handler for message type: {message_type}")
    
    def get_handler(self, message_type: str) -> MessageHandler:
        """Get handler for message type."""
        return self.handlers.get(message_type, self._default_handler)
    
    async def handle_message(
        self,
        connection_id: str,
        message: WebSocketMessage,
        manager: WebSocketConnectionManager
    ):
        """Route message to appropriate handler."""
        handler = self.get_handler(message.type)
        try:
            await handler(connection_id, message, manager)
        except Exception as e:
            logger.error(f"Error handling message type {message.type}: {e}")
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="error",
                    payload={
                        "message": str(e),
                        "original_message_id": message.id
                    }
                )
            )
    
    def _register_default_handlers(self):
        """Register default message handlers."""
        self.register("ping", handle_ping)
        self.register("subscribe", handle_subscribe)
        self.register("unsubscribe", handle_unsubscribe)
        self.register("token.create", handle_token_create)
        self.register("token.get", handle_token_get)
        self.register("token.list", handle_token_list)
        self.register("graph.connect", handle_graph_connect)
        self.register("graph.neighbors", handle_graph_neighbors)
    
    async def _default_handler(
        self,
        connection_id: str,
        message: WebSocketMessage,
        manager: WebSocketConnectionManager
    ):
        """Default handler for unknown message types."""
        logger.warning(f"Unknown message type: {message.type}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={
                    "message": f"Unknown message type: {message.type}",
                    "original_message_id": message.id
                }
            )
        )


# Handler implementations

async def handle_ping(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle ping message (heartbeat)."""
    await manager.update_heartbeat(connection_id)
    await manager.send_to_connection(
        connection_id,
        WebSocketMessage(
            type="pong",
            payload={"message_id": message.id}
        )
    )


async def handle_subscribe(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle topic subscription."""
    topic = message.payload.get("topic")
    if not topic:
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": "Topic is required"}
            )
        )
        return
    
    success = await manager.subscribe(connection_id, topic)
    
    await manager.send_to_connection(
        connection_id,
        WebSocketMessage(
            type="subscribed",
            payload={
                "topic": topic,
                "success": success
            }
        )
    )


async def handle_unsubscribe(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle topic unsubscription."""
    topic = message.payload.get("topic")
    if not topic:
        return
    
    success = await manager.unsubscribe(connection_id, topic)
    
    await manager.send_to_connection(
        connection_id,
        WebSocketMessage(
            type="unsubscribed",
            payload={
                "topic": topic,
                "success": success
            }
        )
    )


async def handle_token_create(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle token creation request."""
    try:
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        async with db_manager.session() as session:
            token_repo = RepositoryFactory.create_token_repository(session)
            
            from src.infrastructure.persistence.models import TokenModel
            from uuid import uuid4
            import time
            
            # Extract token data from payload
            payload = message.payload
            token = TokenModel(
                id=uuid4(),
                binary_data=b'\x00' * 64,
                coord_x=payload.get('coord_x', [0.0] * 8),
                coord_y=payload.get('coord_y', [0.0] * 8),
                coord_z=payload.get('coord_z', [0.0] * 8),
                flags=payload.get('flags', 0),
                weight=payload.get('weight', 1.0),
                timestamp=int(time.time() * 1000),
                token_type=payload.get('type', 'default'),
                metadata=payload.get('metadata', {})
            )
            
            token = await token_repo.create(token)
            
            # Send response
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="token.created",
                    payload={
                        "token_id": str(token.id),
                        "type": token.token_type,
                        "weight": token.weight
                    }
                )
            )
            
            # Broadcast to subscribers
            await manager.publish_to_topic(
                "tokens",
                WebSocketMessage(
                    type="token.created",
                    payload={"token_id": str(token.id)}
                )
            )
        
    except Exception as e:
        logger.error(f"Error creating token: {e}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": str(e)}
            )
        )


async def handle_token_get(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle token retrieval request."""
    try:
        token_id = message.payload.get("token_id")
        if not token_id:
            raise ValueError("token_id is required")
        
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        async with db_manager.session() as session:
            token_repo = RepositoryFactory.create_token_repository(session)
            token = await token_repo.get_by_id(UUID(token_id))
            
            if not token:
                await manager.send_to_connection(
                    connection_id,
                    WebSocketMessage(
                        type="token.not_found",
                        payload={"token_id": token_id}
                    )
                )
                return
            
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="token.data",
                    payload={
                        "token_id": str(token.id),
                        "type": token.token_type,
                        "weight": token.weight,
                        "flags": token.flags,
                        "coordinates": {
                            "x": token.coord_x,
                            "y": token.coord_y,
                            "z": token.coord_z
                        },
                        "metadata": token.metadata,
                        "created_at": token.created_at.isoformat()
                    }
                )
            )
    
    except Exception as e:
        logger.error(f"Error getting token: {e}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": str(e)}
            )
        )


async def handle_token_list(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle token list request."""
    try:
        limit = message.payload.get("limit", 10)
        offset = message.payload.get("offset", 0)
        
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        async with db_manager.session() as session:
            token_repo = RepositoryFactory.create_token_repository(session)
            tokens = await token_repo.get_all(limit=limit, offset=offset)
            
            token_list = [
                {
                    "token_id": str(token.id),
                    "type": token.token_type,
                    "weight": token.weight,
                    "created_at": token.created_at.isoformat()
                }
                for token in tokens
            ]
            
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="token.list",
                    payload={
                        "tokens": token_list,
                        "count": len(token_list),
                        "offset": offset
                    }
                )
            )
    
    except Exception as e:
        logger.error(f"Error listing tokens: {e}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": str(e)}
            )
        )


async def handle_graph_connect(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle graph connection creation."""
    try:
        source_id = message.payload.get("source_id")
        target_id = message.payload.get("target_id")
        
        if not source_id or not target_id:
            raise ValueError("source_id and target_id are required")
        
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        async with db_manager.session() as session:
            graph_repo = RepositoryFactory.create_graph_repository(session)
            
            connection = await graph_repo.create_connection(
                source_id=UUID(source_id),
                target_id=UUID(target_id),
                connection_type=message.payload.get("type", "generic"),
                weight=message.payload.get("weight", 1.0),
                bidirectional=message.payload.get("bidirectional", False)
            )
            
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="graph.connected",
                    payload={
                        "connection_id": str(connection.id),
                        "source_id": source_id,
                        "target_id": target_id
                    }
                )
            )
            
            # Broadcast to graph subscribers
            await manager.publish_to_topic(
                "graph",
                WebSocketMessage(
                    type="graph.connected",
                    payload={
                        "connection_id": str(connection.id),
                        "source_id": source_id,
                        "target_id": target_id
                    }
                )
            )
    
    except Exception as e:
        logger.error(f"Error creating connection: {e}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": str(e)}
            )
        )


async def handle_graph_neighbors(
    connection_id: str,
    message: WebSocketMessage,
    manager: WebSocketConnectionManager
):
    """Handle graph neighbors request."""
    try:
        token_id = message.payload.get("token_id")
        if not token_id:
            raise ValueError("token_id is required")
        
        direction = message.payload.get("direction", "both")
        
        config_loader = ConfigLoader()
        db_config = config_loader.load('infrastructure/database.yaml')
        db_manager = await DatabaseFactory.create_database_manager(db_config['database'])
        
        async with db_manager.session() as session:
            graph_repo = RepositoryFactory.create_graph_repository(session)
            neighbors = await graph_repo.get_neighbors(UUID(token_id), direction=direction)
            
            neighbor_list = [
                {
                    "connection_id": str(conn.id),
                    "source_id": str(conn.source_id),
                    "target_id": str(conn.target_id),
                    "type": conn.connection_type,
                    "weight": conn.weight
                }
                for conn in neighbors
            ]
            
            await manager.send_to_connection(
                connection_id,
                WebSocketMessage(
                    type="graph.neighbors",
                    payload={
                        "token_id": token_id,
                        "neighbors": neighbor_list,
                        "count": len(neighbor_list)
                    }
                )
            )
    
    except Exception as e:
        logger.error(f"Error getting neighbors: {e}")
        await manager.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="error",
                payload={"message": str(e)}
            )
        )


# Global handler registry
message_handler_registry = MessageHandlerRegistry()