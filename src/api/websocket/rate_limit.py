
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
WebSocket Rate Limiting

Token bucket rate limiting for WebSocket messages to prevent spam/abuse.
"""

import time
from typing import Dict, Optional
from dataclasses import dataclass
from collections import defaultdict

from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_ratelimit")


@dataclass
class TokenBucket:
    """
    Token bucket for rate limiting.

    Each client has a bucket that refills at a steady rate.
    """

    capacity: int  # Maximum tokens
    refill_rate: float  # Tokens added per second
    tokens: float  # Current tokens
    last_refill: float  # Last refill timestamp

    def consume(self, tokens: int = 1) -> bool:
        """
        Try to consume tokens from the bucket.

        Args:
            tokens: Number of tokens to consume

        Returns:
            True if tokens were consumed, False if bucket is empty
        """
        # Refill bucket based on time elapsed
        now = time.time()
        elapsed = now - self.last_refill
        refill_amount = elapsed * self.refill_rate

        self.tokens = min(self.capacity, self.tokens + refill_amount)
        self.last_refill = now

        # Try to consume
        if self.tokens >= tokens:
            self.tokens -= tokens
            return True

        return False

    def get_wait_time(self, tokens: int = 1) -> float:
        """
        Get time to wait until tokens are available.

        Args:
            tokens: Number of tokens needed

        Returns:
            Seconds to wait (0 if tokens available now)
        """
        if self.tokens >= tokens:
            return 0.0

        tokens_needed = tokens - self.tokens
        return tokens_needed / self.refill_rate


class WebSocketRateLimiter:
    """
    Rate limiter for WebSocket connections.

    Implements token bucket algorithm with different limits for different message types.
    """

    def __init__(
        self,
        default_capacity: int = 60,  # 60 messages
        default_refill_rate: float = 10.0,  # 10 messages/second
        burst_capacity: int = 20,  # Allow bursts of 20 messages
    ):
        """
        Initialize rate limiter.

        Args:
            default_capacity: Default bucket capacity
            default_refill_rate: Default refill rate (messages/second)
            burst_capacity: Extra capacity for bursts
        """
        self.default_capacity = default_capacity
        self.default_refill_rate = default_refill_rate
        self.burst_capacity = burst_capacity

        # Client buckets: client_id -> TokenBucket
        self._buckets: Dict[str, TokenBucket] = {}

        # Message type specific limits
        self._message_limits = {
            "ping": (120, 2.0),  # 120 capacity, 2/sec (120 pings/min)
            "subscribe": (30, 1.0),  # 30 capacity, 1/sec (prevent spam subscriptions)
            "unsubscribe": (30, 1.0),  # 30 capacity, 1/sec
            "get_subscriptions": (10, 0.5),  # 10 capacity, 0.5/sec
            "default": (default_capacity, default_refill_rate),
        }

        logger.info(
            "WebSocket rate limiter initialized",
            extra={
                "event": "ratelimit_init",
                "default_capacity": default_capacity,
                "default_refill_rate": default_refill_rate,
                "burst_capacity": burst_capacity,
            }
        )

    def _get_bucket(self, client_id: str, message_type: str = "default") -> TokenBucket:
        """
        Get or create bucket for client and message type.

        Args:
            client_id: Client identifier
            message_type: Type of message

        Returns:
            TokenBucket instance
        """
        bucket_key = f"{client_id}:{message_type}"

        if bucket_key not in self._buckets:
            # Get limits for message type
            capacity, refill_rate = self._message_limits.get(
                message_type, self._message_limits["default"]
            )

            # Add burst capacity
            total_capacity = capacity + self.burst_capacity

            # Create new bucket (start full)
            self._buckets[bucket_key] = TokenBucket(
                capacity=total_capacity,
                refill_rate=refill_rate,
                tokens=total_capacity,  # Start full
                last_refill=time.time(),
            )

        return self._buckets[bucket_key]

    def check_rate_limit(
        self,
        client_id: str,
        message_type: str = "default",
        tokens: int = 1,
    ) -> tuple[bool, Optional[float]]:
        """
        Check if message is allowed under rate limit.

        Args:
            client_id: Client identifier
            message_type: Type of message
            tokens: Number of tokens to consume

        Returns:
            Tuple of (allowed: bool, retry_after: Optional[float])
            retry_after is seconds to wait if not allowed
        """
        bucket = self._get_bucket(client_id, message_type)

        if bucket.consume(tokens):
            return True, None

        # Rate limit exceeded
        retry_after = bucket.get_wait_time(tokens)

        logger.warning(
            f"Rate limit exceeded for {client_id}",
            extra={
                "event": "ratelimit_exceeded",
                "client_id": client_id,
                "message_type": message_type,
                "retry_after": retry_after,
                "tokens_available": bucket.tokens,
            }
        )

        return False, retry_after

    def reset_client(self, client_id: str):
        """
        Reset all buckets for a client (on disconnect).

        Args:
            client_id: Client identifier
        """
        # Remove all buckets for this client
        keys_to_remove = [key for key in self._buckets.keys() if key.startswith(f"{client_id}:")]

        for key in keys_to_remove:
            del self._buckets[key]

        if keys_to_remove:
            logger.debug(
                f"Reset rate limit buckets for {client_id}",
                extra={
                    "event": "ratelimit_reset",
                    "client_id": client_id,
                    "buckets_removed": len(keys_to_remove),
                }
            )

    def get_client_stats(self, client_id: str) -> Dict[str, dict]:
        """
        Get rate limit statistics for a client.

        Args:
            client_id: Client identifier

        Returns:
            Dictionary with stats per message type
        """
        stats = {}

        for key, bucket in self._buckets.items():
            if key.startswith(f"{client_id}:"):
                message_type = key.split(":", 1)[1]
                stats[message_type] = {
                    "capacity": bucket.capacity,
                    "tokens_available": bucket.tokens,
                    "refill_rate": bucket.refill_rate,
                    "utilization": 1.0 - (bucket.tokens / bucket.capacity),
                }

        return stats

    def cleanup_stale_buckets(self, max_age: float = 3600):
        """
        Remove buckets that haven't been used recently.

        Args:
            max_age: Maximum age in seconds (default 1 hour)
        """
        now = time.time()
        stale_keys = []

        for key, bucket in self._buckets.items():
            if now - bucket.last_refill > max_age:
                stale_keys.append(key)

        for key in stale_keys:
            del self._buckets[key]

        if stale_keys:
            logger.info(
                f"Cleaned up {len(stale_keys)} stale rate limit buckets",
                extra={
                    "event": "ratelimit_cleanup",
                    "buckets_removed": len(stale_keys),
                }
            )


# Global rate limiter instance
rate_limiter = WebSocketRateLimiter(
    default_capacity=60,  # 60 messages
    default_refill_rate=10.0,  # 10/sec = 600/min
    burst_capacity=20,  # Allow bursts
)
