"""
API Key Storage

Persistent storage for API keys with file-based backend.
In production: migrate to database (PostgreSQL, Redis, etc.)
"""

import json
import secrets
import hashlib
from pathlib import Path
from typing import Dict, List, Optional
from datetime import datetime, timedelta
import threading

from ..models.auth import APIKey


class APIKeyStorage:
    """
    File-based API key storage.

    Features:
    - Persistent JSON storage
    - Secure key generation with prefix system
    - Key hashing (only hash stored, not full key)
    - Automatic expiration checking
    - Thread-safe operations
    """

    def __init__(self, storage_path: str = "data/api_keys.json"):
        self.storage_path = Path(storage_path)
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self._lock = threading.Lock()
        self._keys: Dict[str, Dict] = {}
        self._load()

    def _load(self):
        """Load keys from file."""
        if self.storage_path.exists():
            with open(self.storage_path, 'r') as f:
                data = json.load(f)
                self._keys = data.get('keys', {})

    def _save(self):
        """Save keys to file."""
        with open(self.storage_path, 'w') as f:
            json.dump({'keys': self._keys}, f, indent=2, default=str)

    def _hash_key(self, key: str) -> str:
        """Hash API key for storage (SHA256)."""
        return hashlib.sha256(key.encode()).hexdigest()

    def generate_key(
        self,
        name: str,
        scopes: List[str],
        rate_limit: int = 1000,
        expires_in_days: Optional[int] = None,
        environment: str = "live"  # "live" or "test"
    ) -> tuple[str, APIKey]:
        """
        Generate new API key.

        Args:
            name: Human-readable key name
            scopes: Permission scopes
            rate_limit: Requests per minute limit
            expires_in_days: Expiration in days (None = no expiration)
            environment: Key environment (live/test)

        Returns:
            Tuple of (full_key, api_key_model)

        Note: Full key is only returned once! Store it securely.
        """
        with self._lock:
            # Generate secure random key
            prefix = "ng_live_" if environment == "live" else "ng_test_"
            random_part = secrets.token_urlsafe(32)  # 43 chars base64
            full_key = f"{prefix}{random_part}"

            # Hash for storage
            key_hash = self._hash_key(full_key)

            # Generate unique ID
            key_id = secrets.token_hex(8)  # 16 chars

            # Calculate expiration
            expires_at = None
            if expires_in_days:
                expires_at = datetime.utcnow() + timedelta(days=expires_in_days)

            # Create key model
            api_key = APIKey(
                key_id=key_id,
                key_prefix=full_key[:16],  # Show first 16 chars for identification
                name=name,
                scopes=scopes,
                rate_limit=rate_limit,
                created_at=datetime.utcnow(),
                expires_at=expires_at,
                last_used_at=None,
                disabled=False
            )

            # Store (save hash, not full key)
            self._keys[key_id] = {
                'key_hash': key_hash,
                'key_prefix': api_key.key_prefix,
                'name': api_key.name,
                'scopes': api_key.scopes,
                'rate_limit': api_key.rate_limit,
                'created_at': api_key.created_at.isoformat(),
                'expires_at': api_key.expires_at.isoformat() if api_key.expires_at else None,
                'last_used_at': None,
                'disabled': api_key.disabled
            }

            self._save()

            return full_key, api_key

    def verify_key(self, key: str) -> Optional[APIKey]:
        """
        Verify API key and return key info.

        Args:
            key: Full API key to verify

        Returns:
            APIKey model if valid, None if invalid/expired/disabled
        """
        with self._lock:
            key_hash = self._hash_key(key)

            # Find key by hash
            for key_id, key_data in self._keys.items():
                if key_data['key_hash'] == key_hash:
                    # Check if disabled
                    if key_data['disabled']:
                        return None

                    # Check expiration
                    if key_data['expires_at']:
                        expires_at = datetime.fromisoformat(key_data['expires_at'])
                        if datetime.utcnow() > expires_at:
                            return None

                    # Update last used
                    key_data['last_used_at'] = datetime.utcnow().isoformat()
                    self._save()

                    # Return key model
                    return APIKey(
                        key_id=key_id,
                        key_prefix=key_data['key_prefix'],
                        name=key_data['name'],
                        scopes=key_data['scopes'],
                        rate_limit=key_data['rate_limit'],
                        created_at=datetime.fromisoformat(key_data['created_at']),
                        expires_at=datetime.fromisoformat(key_data['expires_at']) if key_data['expires_at'] else None,
                        last_used_at=datetime.fromisoformat(key_data['last_used_at']) if key_data['last_used_at'] else None,
                        disabled=key_data['disabled']
                    )

            return None

    def list_keys(self) -> List[APIKey]:
        """List all API keys (without full key values)."""
        with self._lock:
            keys = []
            for key_id, key_data in self._keys.items():
                keys.append(APIKey(
                    key_id=key_id,
                    key_prefix=key_data['key_prefix'],
                    name=key_data['name'],
                    scopes=key_data['scopes'],
                    rate_limit=key_data['rate_limit'],
                    created_at=datetime.fromisoformat(key_data['created_at']),
                    expires_at=datetime.fromisoformat(key_data['expires_at']) if key_data['expires_at'] else None,
                    last_used_at=datetime.fromisoformat(key_data['last_used_at']) if key_data['last_used_at'] else None,
                    disabled=key_data['disabled']
                ))
            return keys

    def get_key(self, key_id: str) -> Optional[APIKey]:
        """Get API key by ID."""
        with self._lock:
            key_data = self._keys.get(key_id)
            if not key_data:
                return None

            return APIKey(
                key_id=key_id,
                key_prefix=key_data['key_prefix'],
                name=key_data['name'],
                scopes=key_data['scopes'],
                rate_limit=key_data['rate_limit'],
                created_at=datetime.fromisoformat(key_data['created_at']),
                expires_at=datetime.fromisoformat(key_data['expires_at']) if key_data['expires_at'] else None,
                last_used_at=datetime.fromisoformat(key_data['last_used_at']) if key_data['last_used_at'] else None,
                disabled=key_data['disabled']
            )

    def revoke_key(self, key_id: str) -> bool:
        """
        Revoke (disable) API key.

        Args:
            key_id: Key ID to revoke

        Returns:
            True if revoked, False if not found
        """
        with self._lock:
            if key_id not in self._keys:
                return False

            self._keys[key_id]['disabled'] = True
            self._save()
            return True

    def delete_key(self, key_id: str) -> bool:
        """
        Permanently delete API key.

        Args:
            key_id: Key ID to delete

        Returns:
            True if deleted, False if not found
        """
        with self._lock:
            if key_id not in self._keys:
                return False

            del self._keys[key_id]
            self._save()
            return True

    def update_key(
        self,
        key_id: str,
        name: Optional[str] = None,
        scopes: Optional[List[str]] = None,
        rate_limit: Optional[int] = None
    ) -> bool:
        """
        Update API key metadata.

        Args:
            key_id: Key ID to update
            name: New name (optional)
            scopes: New scopes (optional)
            rate_limit: New rate limit (optional)

        Returns:
            True if updated, False if not found
        """
        with self._lock:
            if key_id not in self._keys:
                return False

            if name is not None:
                self._keys[key_id]['name'] = name
            if scopes is not None:
                self._keys[key_id]['scopes'] = scopes
            if rate_limit is not None:
                self._keys[key_id]['rate_limit'] = rate_limit

            self._save()
            return True


# Global instance
_api_key_storage: Optional[APIKeyStorage] = None


def get_api_key_storage() -> APIKeyStorage:
    """Get global API key storage instance."""
    global _api_key_storage
    if _api_key_storage is None:
        _api_key_storage = APIKeyStorage()
    return _api_key_storage
