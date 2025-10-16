"""
FastAPI WebSocket routes and endpoints.
"""

from fastapi import APIRouter, WebSocket, WebSocketDisconnect, Query, Header
from typing import Optional
import json

from src.infrastructure.websocket.server import (
    connection_manager,
    WebSocketMessage
)
from src.infrastructure.websocket.handlers import message_handler_registry
from src.core.utils.logger import get_logger

logger = get_logger(__name__)

router = APIRouter()


@router.websocket("/ws")
async def websocket_endpoint(
    websocket: WebSocket,
    client_id: Optional[str] = Query(None),
    token: Optional[str] = Header(None)
):
    """
    Main WebSocket endpoint for real-time communication.
    
    Query Parameters:
        client_id: Optional client identifier
        
    Headers:
        token: Optional authentication token
        
    Message Format:
        {
            "id": "message-id",
            "type": "message-type",
            "payload": {...},
            "timestamp": 1234567890
        }
    """
    connection_id = None
    
    try:
        # TODO: Add authentication logic here
        # if token:
        #     user = await authenticate_token(token)
        
        # Connect
        connection_id = await connection_manager.connect(
            websocket,
            client_id=client_id,
            metadata={"user_agent": websocket.headers.get("user-agent")}
        )
        
        logger.info(f"WebSocket connected: {connection_id}")
        
        # Message loop
        while True:
            try:
                # Receive message
                data = await websocket.receive_text()
                message_data = json.loads(data)
                
                # Validate and parse message
                message = WebSocketMessage(**message_data)
                
                logger.debug(f"Received message: {message.type} from {connection_id}")
                
                # Handle message
                await message_handler_registry.handle_message(
                    connection_id,
                    message,
                    connection_manager
                )
                
            except json.JSONDecodeError as e:
                logger.error(f"Invalid JSON from {connection_id}: {e}")
                await connection_manager.send_to_connection(
                    connection_id,
                    WebSocketMessage(
                        type="error",
                        payload={"message": "Invalid JSON format"}
                    )
                )
            
            except WebSocketDisconnect:
                logger.info(f"Client disconnected: {connection_id}")
                break
            
            except Exception as e:
                logger.error(f"Error handling message from {connection_id}: {e}")
                await connection_manager.send_to_connection(
                    connection_id,
                    WebSocketMessage(
                        type="error",
                        payload={"message": str(e)}
                    )
                )
    
    except WebSocketDisconnect:
        logger.info(f"Client disconnected during handshake: {client_id}")
    
    except Exception as e:
        logger.error(f"WebSocket error: {e}")
    
    finally:
        if connection_id:
            await connection_manager.disconnect(connection_id)


@router.get("/ws/connections")
async def get_connections():
    """Get list of active WebSocket connections."""
    connections = connection_manager.get_connections()
    
    return {
        "count": len(connections),
        "connections": [
            {
                "connection_id": conn.connection_id,
                "client_id": conn.client_id,
                "connected_at": conn.connected_at.isoformat(),
                "subscriptions": list(conn.subscriptions),
                "metadata": conn.metadata
            }
            for conn in connections
        ]
    }


@router.get("/ws/stats")
async def get_websocket_stats():
    """Get WebSocket server statistics."""
    connections = connection_manager.get_connections()
    
    # Count subscriptions per topic
    topic_counts = {}
    for conn in connections:
        for topic in conn.subscriptions:
            topic_counts[topic] = topic_counts.get(topic, 0) + 1
    
    return {
        "active_connections": connection_manager.get_connection_count(),
        "topics": topic_counts,
        "total_subscriptions": sum(len(conn.subscriptions) for conn in connections)
    }


@router.post("/ws/broadcast")
async def broadcast_message(message: dict):
    """
    Broadcast a message to all connected clients.
    
    Body:
        {
            "type": "message-type",
            "payload": {...}
        }
    """
    ws_message = WebSocketMessage(
        type=message.get("type", "broadcast"),
        payload=message.get("payload", {})
    )
    
    await connection_manager.broadcast(ws_message)
    
    return {
        "status": "broadcasted",
        "recipients": connection_manager.get_connection_count()
    }


@router.post("/ws/publish/{topic}")
async def publish_to_topic(topic: str, message: dict):
    """
    Publish message to specific topic subscribers.
    
    Path:
        topic: Topic name
        
    Body:
        {
            "type": "message-type",
            "payload": {...}
        }
    """
    ws_message = WebSocketMessage(
        type=message.get("type", "broadcast"),
        payload=message.get("payload", {})
    )
    
    await connection_manager.publish_to_topic(topic, ws_message)
    
    # Count subscribers
    subscriber_count = len(connection_manager.topic_subscribers.get(topic, set()))
    
    return {
        "status": "published",
        "topic": topic,
        "subscribers": subscriber_count
    }