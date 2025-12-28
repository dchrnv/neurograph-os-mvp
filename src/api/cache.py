"""
Simple in-memory cache for NeuroGraph API.

Provides caching for frequently accessed data:
- User permissions (RBAC)
- API key validations
- Token blacklist checks

For production: Replace with Redis or Memcached for distributed caching.
"""

import time
from typing import Any, Optional, Dict, Tuple
from dataclasses import dataclass
from collections import OrderedDict
import threading


@dataclass
class CacheEntry:
    """Cache entry with value and metadata."""
    value: Any
    expires_at: float
    hits: int = 0


class LRUCache:
    """
    Thread-safe LRU (Least Recently Used) cache with TTL support.

    Features:
    - Maximum size limit (evicts least recently used items)
    - TTL (Time To Live) for automatic expiration
    - Thread-safe operations
    - Hit/miss statistics
    """

    def __init__(self, max_size: int = 1000, default_ttl: int = 300):
        """
        Initialize LRU cache.

        Args:
            max_size: Maximum number of items in cache
            default_ttl: Default TTL in seconds (0 = no expiration)
        """
        self.max_size = max_size
        self.default_ttl = default_ttl
        self._cache: OrderedDict[str, CacheEntry] = OrderedDict()
        self._lock = threading.RLock()

        # Statistics
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: str) -> Optional[Any]:
        """
        Get value from cache.

        Args:
            key: Cache key

        Returns:
            Cached value or None if not found/expired
        """
        with self._lock:
            entry = self._cache.get(key)

            if entry is None:
                self._misses += 1
                return None

            # Check if expired
            if entry.expires_at > 0 and time.time() > entry.expires_at:
                # Expired - remove and return None
                del self._cache[key]
                self._misses += 1
                return None

            # Hit - move to end (most recently used)
            self._cache.move_to_end(key)
            entry.hits += 1
            self._hits += 1

            return entry.value

    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        """
        Set value in cache.

        Args:
            key: Cache key
            value: Value to cache
            ttl: TTL in seconds (None = use default, 0 = no expiration)
        """
        with self._lock:
            if ttl is None:
                ttl = self.default_ttl

            expires_at = time.time() + ttl if ttl > 0 else 0

            # Remove old entry if exists
            if key in self._cache:
                del self._cache[key]

            # Add new entry
            self._cache[key] = CacheEntry(
                value=value,
                expires_at=expires_at,
                hits=0
            )

            # Move to end (most recently used)
            self._cache.move_to_end(key)

            # Evict if over max size
            while len(self._cache) > self.max_size:
                # Remove least recently used (first item)
                self._cache.popitem(last=False)
                self._evictions += 1

    def delete(self, key: str):
        """Delete key from cache."""
        with self._lock:
            if key in self._cache:
                del self._cache[key]

    def clear(self):
        """Clear all cache entries."""
        with self._lock:
            self._cache.clear()
            self._hits = 0
            self._misses = 0
            self._evictions = 0

    def get_stats(self) -> Dict[str, Any]:
        """
        Get cache statistics.

        Returns:
            Dict with cache stats
        """
        with self._lock:
            total_requests = self._hits + self._misses
            hit_rate = (self._hits / total_requests * 100) if total_requests > 0 else 0

            return {
                "size": len(self._cache),
                "max_size": self.max_size,
                "hits": self._hits,
                "misses": self._misses,
                "evictions": self._evictions,
                "hit_rate": round(hit_rate, 2),
                "total_requests": total_requests,
            }

    def cleanup_expired(self) -> int:
        """
        Remove all expired entries.

        Returns:
            Number of entries removed
        """
        with self._lock:
            now = time.time()
            expired_keys = [
                key
                for key, entry in self._cache.items()
                if entry.expires_at > 0 and now > entry.expires_at
            ]

            for key in expired_keys:
                del self._cache[key]

            return len(expired_keys)


# Global cache instances
permissions_cache = LRUCache(max_size=500, default_ttl=300)  # 5 minutes
api_key_cache = LRUCache(max_size=1000, default_ttl=60)  # 1 minute
token_validation_cache = LRUCache(max_size=2000, default_ttl=60)  # 1 minute


def get_cached_permissions(user_id: str) -> Optional[list[str]]:
    """
    Get cached user permissions.

    Args:
        user_id: User ID

    Returns:
        List of permissions or None if not cached
    """
    return permissions_cache.get(f"permissions:{user_id}")


def cache_permissions(user_id: str, permissions: list[str], ttl: int = 300):
    """
    Cache user permissions.

    Args:
        user_id: User ID
        permissions: List of permissions
        ttl: TTL in seconds (default: 5 minutes)
    """
    permissions_cache.set(f"permissions:{user_id}", permissions, ttl)


def invalidate_permissions(user_id: str):
    """
    Invalidate cached permissions for user.

    Args:
        user_id: User ID
    """
    permissions_cache.delete(f"permissions:{user_id}")


def get_all_cache_stats() -> Dict[str, Dict[str, Any]]:
    """
    Get statistics for all caches.

    Returns:
        Dict with stats for each cache
    """
    return {
        "permissions": permissions_cache.get_stats(),
        "api_keys": api_key_cache.get_stats(),
        "token_validation": token_validation_cache.get_stats(),
    }


def cleanup_all_caches():
    """Cleanup expired entries from all caches."""
    permissions_cache.cleanup_expired()
    api_key_cache.cleanup_expired()
    token_validation_cache.cleanup_expired()


# Example usage
if __name__ == "__main__":
    # Test cache
    cache = LRUCache(max_size=3, default_ttl=5)

    # Add items
    cache.set("user:1", {"name": "Alice"})
    cache.set("user:2", {"name": "Bob"})
    cache.set("user:3", {"name": "Charlie"})

    # Get items
    print(cache.get("user:1"))  # Hit
    print(cache.get("user:2"))  # Hit
    print(cache.get("user:999"))  # Miss

    # Add one more (should evict user:3 - least recently used)
    cache.set("user:4", {"name": "Dave"})
    print(cache.get("user:3"))  # Miss (evicted)

    # Stats
    print("\nCache stats:")
    print(cache.get_stats())

    # Test expiration
    cache.set("temp", {"data": "test"}, ttl=1)
    print("\nBefore expiration:", cache.get("temp"))
    time.sleep(2)
    print("After expiration:", cache.get("temp"))

    print("\nFinal stats:")
    print(cache.get_stats())
