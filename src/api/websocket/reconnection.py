
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.

"""
WebSocket Reconnection Tokens

Allows clients to seamlessly reconnect and restore their session state.
"""

import time
import secrets
from typing import Dict, Optional, Set
from dataclasses import dataclass, field
from datetime import datetime

from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_reconnection")


@dataclass
class ReconnectionSession:
    """
    Session state for reconnection.

    Stores client state so it can be restored after disconnect.
    """

    client_id: str
    user_id: Optional[str]
    reconnection_token: str
    created_at: float
    expires_at: float
    subscriptions: Set[str] = field(default_factory=set)
    metadata: Dict[str, any] = field(default_factory=dict)

    def is_expired(self) -> bool:
        """Check if session is expired."""
        return time.time() > self.expires_at

    def is_valid(self) -> bool:
        """Check if session is valid (not expired)."""
        return not self.is_expired()


class ReconnectionManager:
    """
    Manages reconnection tokens and session restoration.

    When a client disconnects, a reconnection token is generated.
    The client can use this token to reconnect and restore their session.
    """

    def __init__(self, token_ttl: int = 300):
        """
        Initialize reconnection manager.

        Args:
            token_ttl: Token time-to-live in seconds (default: 5 minutes)
        """
        self.token_ttl = token_ttl
        self._sessions: Dict[str, ReconnectionSession] = {}
        self._token_to_session: Dict[str, str] = {}  # token -> client_id

        logger.info(
            "Reconnection manager initialized",
            extra={
                "event": "reconnection_manager_init",
                "token_ttl": token_ttl,
            }
        )

    def create_reconnection_token(
        self,
        client_id: str,
        user_id: Optional[str] = None,
        subscriptions: Optional[Set[str]] = None,
        metadata: Optional[Dict] = None,
    ) -> str:
        """
        Create a reconnection token for a disconnected client.

        Args:
            client_id: Original client identifier
            user_id: User identifier
            subscriptions: Active channel subscriptions
            metadata: Additional session metadata

        Returns:
            Reconnection token
        """
        # Generate secure token
        token = secrets.token_urlsafe(32)

        # Create session
        now = time.time()
        session = ReconnectionSession(
            client_id=client_id,
            user_id=user_id,
            reconnection_token=token,
            created_at=now,
            expires_at=now + self.token_ttl,
            subscriptions=subscriptions or set(),
            metadata=metadata or {},
        )

        # Store session
        self._sessions[client_id] = session
        self._token_to_session[token] = client_id

        logger.info(
            f"Created reconnection token for {client_id}",
            extra={
                "event": "reconnection_token_created",
                "client_id": client_id,
                "user_id": user_id,
                "expires_in": self.token_ttl,
                "subscriptions_count": len(session.subscriptions),
            }
        )

        return token

    def validate_reconnection_token(self, token: str) -> Optional[ReconnectionSession]:
        """
        Validate a reconnection token and return the session.

        Args:
            token: Reconnection token

        Returns:
            ReconnectionSession if valid, None otherwise
        """
        # Check if token exists
        client_id = self._token_to_session.get(token)
        if not client_id:
            logger.warning(
                "Invalid reconnection token",
                extra={
                    "event": "reconnection_token_invalid",
                    "token_prefix": token[:8] if token else None,
                }
            )
            return None

        # Get session
        session = self._sessions.get(client_id)
        if not session:
            logger.warning(
                "Reconnection session not found",
                extra={
                    "event": "reconnection_session_not_found",
                    "client_id": client_id,
                }
            )
            return None

        # Check if expired
        if session.is_expired():
            logger.info(
                f"Reconnection token expired for {client_id}",
                extra={
                    "event": "reconnection_token_expired",
                    "client_id": client_id,
                    "expired_at": datetime.fromtimestamp(session.expires_at).isoformat(),
                }
            )
            # Clean up expired session
            self._remove_session(client_id, token)
            return None

        logger.info(
            f"Valid reconnection token for {client_id}",
            extra={
                "event": "reconnection_token_valid",
                "client_id": client_id,
                "user_id": session.user_id,
            }
        )

        return session

    def restore_session(self, token: str, new_client_id: str) -> Optional[ReconnectionSession]:
        """
        Restore a session using reconnection token.

        Args:
            token: Reconnection token
            new_client_id: New client identifier for the reconnected session

        Returns:
            Restored session or None if invalid
        """
        # Validate token
        session = self.validate_reconnection_token(token)
        if not session:
            return None

        old_client_id = session.client_id

        # Update session with new client_id
        session.client_id = new_client_id

        # Remove old mappings
        if old_client_id in self._sessions:
            del self._sessions[old_client_id]
        if token in self._token_to_session:
            del self._token_to_session[token]

        # Store with new client_id
        self._sessions[new_client_id] = session

        logger.info(
            f"Session restored: {old_client_id} → {new_client_id}",
            extra={
                "event": "session_restored",
                "old_client_id": old_client_id,
                "new_client_id": new_client_id,
                "user_id": session.user_id,
                "subscriptions_restored": len(session.subscriptions),
            }
        )

        return session

    def invalidate_token(self, token: str):
        """
        Invalidate a reconnection token.

        Args:
            token: Reconnection token to invalidate
        """
        client_id = self._token_to_session.get(token)
        if client_id:
            self._remove_session(client_id, token)
            logger.info(
                f"Reconnection token invalidated for {client_id}",
                extra={
                    "event": "reconnection_token_invalidated",
                    "client_id": client_id,
                }
            )

    def _remove_session(self, client_id: str, token: str):
        """Remove session and token mappings."""
        if client_id in self._sessions:
            del self._sessions[client_id]
        if token in self._token_to_session:
            del self._token_to_session[token]

    def cleanup_expired_sessions(self):
        """Remove all expired sessions."""
        now = time.time()
        expired = []

        for client_id, session in self._sessions.items():
            if session.is_expired():
                expired.append((client_id, session.reconnection_token))

        for client_id, token in expired:
            self._remove_session(client_id, token)

        if expired:
            logger.info(
                f"Cleaned up {len(expired)} expired reconnection sessions",
                extra={
                    "event": "reconnection_cleanup",
                    "sessions_removed": len(expired),
                }
            )

        return len(expired)

    def get_session_info(self, client_id: str) -> Optional[dict]:
        """
        Get session information for a client.

        Args:
            client_id: Client identifier

        Returns:
            Session info dictionary or None
        """
        session = self._sessions.get(client_id)
        if not session:
            return None

        return {
            "client_id": session.client_id,
            "user_id": session.user_id,
            "created_at": datetime.fromtimestamp(session.created_at).isoformat(),
            "expires_at": datetime.fromtimestamp(session.expires_at).isoformat(),
            "is_valid": session.is_valid(),
            "subscriptions": list(session.subscriptions),
            "metadata": session.metadata,
        }

    def get_stats(self) -> dict:
        """
        Get reconnection manager statistics.

        Returns:
            Statistics dictionary
        """
        now = time.time()
        valid_sessions = sum(1 for s in self._sessions.values() if s.is_valid())
        expired_sessions = len(self._sessions) - valid_sessions

        return {
            "total_sessions": len(self._sessions),
            "valid_sessions": valid_sessions,
            "expired_sessions": expired_sessions,
            "tokens_issued": len(self._token_to_session),
        }


# Global reconnection manager instance
reconnection_manager = ReconnectionManager(token_ttl=300)  # 5 minutes
