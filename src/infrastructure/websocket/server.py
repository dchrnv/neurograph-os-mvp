"""
WebSocket server implementation for real-time communication.
"""

import asyncio
import json
from typing import Dict, Set, Optional, Any, List
from datetime import datetime
from uuid import UUID, uuid4

from fastapi import WebSocket, WebSocketDisconnect, status
from pydantic import BaseModel, Field

from src.core.utils.logger import get_logger

logger = get_logger(__name__)


class WebSocketMessage(BaseModel):
    """WebSocket message structure."""
    
    id: str = Field(default_factory=lambda: str(uuid4()))
    type: str
    payload: Dict[str, Any]
    timestamp: int = Field(default_factory=lambda: int(datetime.now().timestamp() * 1000))
    sender_id: Optional[str] = None
    
    class Config:
        json_encoders = {
            UUID: str,
            datetime: lambda v: v.isoformat()
        }


class ConnectionInfo(BaseModel):
    """Connection metadata."""
    
    connection_id: str
    client_id: Optional[str] = None
    connected_at: datetime = Field(default_factory=datetime.now)
    last_heartbeat: datetime = Field(default_factory=datetime.now)
    metadata: Dict[str, Any] = Field(default_factory=dict)
    subscriptions: Set[str] = Field(default_factory=set)


class WebSocketConnectionManager:
    """Manages WebSocket connections and message routing."""
    
    def __init__(self):
        """Initialize connection manager."""
        self.active_connections: Dict[str, WebSocket] = {}
        self.connection_info: Dict[str, ConnectionInfo] = {}
        self.topic_subscribers: Dict[str, Set[str]] = {}
        self._lock = asyncio.Lock()
        self._heartbeat_task: Optional[asyncio.Task] = None
        self._heartbeat_interval = 30  # seconds
        
    async def start(self):
        """Start the connection manager."""
        self._heartbeat_task = asyncio.create_task(self._heartbeat_loop())
        logger.info("WebSocket connection manager started")
    
    async def stop(self):
        """Stop the connection manager and close all connections."""
        if self._heartbeat_task:
            self._heartbeat_task.cancel()
            try:
                await self._heartbeat_task
            except asyncio.CancelledError:
                pass
        
        # Close all connections
        for connection_id in list(self.active_connections.keys()):
            await self.disconnect(connection_id)
        
        logger.info("WebSocket connection manager stopped")
    
    async def connect(
        self, 
        websocket: WebSocket,
        connection_id: Optional[str] = None,
        client_id: Optional[str] = None,
        metadata: Optional[Dict[str, Any]] = None
    ) -> str:
        """
        Accept and register a new WebSocket connection.
        
        Args:
            websocket: WebSocket instance
            connection_id: Optional connection ID (generated if not provided)
            client_id: Optional client identifier
            metadata: Optional connection metadata
            
        Returns:
            Connection ID
        """
        await websocket.accept()
        
        connection_id = connection_id or str(uuid4())
        
        async with self._lock:
            self.active_connections[connection_id] = websocket
            self.connection_info[connection_id] = ConnectionInfo(
                connection_id=connection_id,
                client_id=client_id,
                metadata=metadata or {}
            )
        
        logger.info(f"WebSocket connection established: {connection_id} (client: {client_id})")
        
        # Send connection confirmation
        await self.send_to_connection(
            connection_id,
            WebSocketMessage(
                type="connection.established",
                payload={
                    "connection_id": connection_id,
                    "timestamp": int(datetime.now().timestamp() * 1000)
                }
            )
        )
        
        return connection_id
    
    async def disconnect(self, connection_id: str):
        """
        Disconnect and cleanup a WebSocket connection.
        
        Args:
            connection_id: Connection ID to disconnect
        """
        async with self._lock:
            if connection_id in self.active_connections:
                websocket = self.active_connections.pop(connection_id)
                info = self.connection_info.pop(connection_id, None)
                
                # Unsubscribe from all topics
                if info:
                    for topic in info.subscriptions:
                        if topic in self.topic_subscribers:
                            self.topic_subscribers[topic].discard(connection_id)
                
                try:
                    await websocket.close()
                except Exception as e:
                    logger.warning(f"Error closing WebSocket {connection_id}: {e}")
                
                logger.info(f"WebSocket disconnected: {connection_id}")
    
    async def send_to_connection(
        self, 
        connection_id: str, 
        message: WebSocketMessage
    ) -> bool:
        """
        Send message to specific connection.
        
        Args:
            connection_id: Target connection ID
            message: Message to send
            
        Returns:
            True if sent successfully, False otherwise
        """
        if connection_id not in self.active_connections:
            return False
        
        websocket = self.active_connections[connection_id]
        
        try:
            await websocket.send_json(message.model_dump())
            return True
        except Exception as e:
            logger.error(f"Error sending to {connection_id}: {e}")
            await self.disconnect(connection_id)
            return False
    
    async def broadcast(
        self, 
        message: WebSocketMessage,
        exclude: Optional[Set[str]] = None
    ):
        """
        Broadcast message to all connections.
        
        Args:
            message: Message to broadcast
            exclude: Set of connection IDs to exclude
        """
        exclude = exclude or set()
        
        tasks = []
        for connection_id in self.active_connections:
            if connection_id not in exclude:
                tasks.append(self.send_to_connection(connection_id, message))
        
        if tasks:
            await asyncio.gather(*tasks, return_exceptions=True)
    
    async def publish_to_topic(self, topic: str, message: WebSocketMessage):
        """
        Publish message to all subscribers of a topic.
        
        Args:
            topic: Topic name
            message: Message to publish
        """
        if topic not in self.topic_subscribers:
            return
        
        subscribers = self.topic_subscribers[topic].copy()
        
        tasks = []
        for connection_id in subscribers:
            tasks.append(self.send_to_connection(connection_id, message))
        
        if tasks:
            await asyncio.gather(*tasks, return_exceptions=True)
    
    async def subscribe(self, connection_id: str, topic: str) -> bool:
        """
        Subscribe connection to a topic.
        
        Args:
            connection_id: Connection ID
            topic: Topic to subscribe to
            
        Returns:
            True if subscribed successfully
        """
        if connection_id not in self.connection_info:
            return False
        
        async with self._lock:
            if topic not in self.topic_subscribers:
                self.topic_subscribers[topic] = set()
            
            self.topic_subscribers[topic].add(connection_id)
            self.connection_info[connection_id].subscriptions.add(topic)
        
        logger.debug(f"Connection {connection_id} subscribed to {topic}")
        return True
    
    async def unsubscribe(self, connection_id: str, topic: str) -> bool:
        """
        Unsubscribe connection from a topic.
        
        Args:
            connection_id: Connection ID
            topic: Topic to unsubscribe from
            
        Returns:
            True if unsubscribed successfully
        """
        if connection_id not in self.connection_info:
            return False
        
        async with self._lock:
            if topic in self.topic_subscribers:
                self.topic_subscribers[topic].discard(connection_id)
            
            self.connection_info[connection_id].subscriptions.discard(topic)
        
        logger.debug(f"Connection {connection_id} unsubscribed from {topic}")
        return True
    
    async def update_heartbeat(self, connection_id: str):
        """Update last heartbeat timestamp for connection."""
        if connection_id in self.connection_info:
            self.connection_info[connection_id].last_heartbeat = datetime.now()
    
    async def _heartbeat_loop(self):
        """Background task to check connection health."""
        while True:
            try:
                await asyncio.sleep(self._heartbeat_interval)
                
                now = datetime.now()
                stale_connections = []
                
                for connection_id, info in self.connection_info.items():
                    # Check if connection is stale (no heartbeat for 2x interval)
                    if (now - info.last_heartbeat).total_seconds() > (self._heartbeat_interval * 2):
                        stale_connections.append(connection_id)
                
                # Disconnect stale connections
                for connection_id in stale_connections:
                    logger.warning(f"Disconnecting stale connection: {connection_id}")
                    await self.disconnect(connection_id)
                
            except asyncio.CancelledError:
                break
            except Exception as e:
                logger.error(f"Error in heartbeat loop: {e}")
    
    def get_connection_count(self) -> int:
        """Get number of active connections."""
        return len(self.active_connections)
    
    def get_connections(self) -> List[ConnectionInfo]:
        """Get list of all connection info."""
        return list(self.connection_info.values())
    
    def get_connection_info(self, connection_id: str) -> Optional[ConnectionInfo]:
        """Get info for specific connection."""
        return self.connection_info.get(connection_id)


# Global connection manager instance
connection_manager = WebSocketConnectionManager()