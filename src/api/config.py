
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
API Configuration

Environment variables and settings for the FastAPI application.
"""

from pydantic_settings import BaseSettings
from typing import List
import os


class Settings(BaseSettings):
    """API settings loaded from environment variables."""

    # Environment
    ENVIRONMENT: str = "development"
    DEBUG: bool = True

    # Logging (v0.52.0)
    LOG_LEVEL: str = "INFO"  # DEBUG, INFO, WARNING, ERROR, CRITICAL
    LOG_JSON_FORMAT: bool = True  # JSON format for structured logging
    LOG_CORRELATION_TRACKING: bool = True  # Enable correlation ID tracking
    LOG_REQUEST_BODY: bool = False  # Log request bodies (security risk!)
    LOG_RESPONSE_BODY: bool = False  # Log response bodies

    # API Configuration
    API_V1_PREFIX: str = "/api/v1"
    PROJECT_NAME: str = "NeuroGraph API"
    VERSION: str = "1.0.0"

    # CORS (v0.58.0 - improved security)
    # In production, set CORS_ORIGINS env var to your actual domains
    # Example: CORS_ORIGINS='["https://app.example.com", "https://admin.example.com"]'
    CORS_ORIGINS: List[str] = [
        "http://localhost:3000",
        "http://localhost:8000",
        "http://127.0.0.1:3000",
        "http://127.0.0.1:8000",
    ]
    # Production warning: Wildcard CORS is a security risk!
    CORS_ALLOW_ALL_ORIGINS: bool = False  # Set to False in production!

    # Server
    HOST: str = "0.0.0.0"
    PORT: int = 8000
    WORKERS: int = 4

    # Security (v0.58.0 - JWT authentication)
    SECRET_KEY: str = "dev-secret-key-change-in-production"
    JWT_SECRET_KEY: str = "dev-jwt-secret-CHANGE-THIS-IN-PRODUCTION"
    JWT_REFRESH_SECRET_KEY: str = "dev-refresh-secret-CHANGE-THIS-IN-PRODUCTION"
    JWT_ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 15  # 15 minutes for access tokens
    REFRESH_TOKEN_EXPIRE_DAYS: int = 7  # 7 days for refresh tokens

    # Password validation (v0.58.0)
    PASSWORD_MIN_LENGTH: int = 8
    PASSWORD_MAX_LENGTH: int = 128

    # Rate Limiting
    RATE_LIMIT_ENABLED: bool = True
    RATE_LIMIT_PER_MINUTE: int = 100

    # NeuroGraph Runtime
    NEUROGRAPH_BOOTSTRAP_PATH: str = os.path.expanduser("~/data/glove.6B.50d.txt")
    NEUROGRAPH_BOOTSTRAP_LIMIT: int = 50000
    NEUROGRAPH_DIMENSIONS: int = 50
    NEUROGRAPH_GRID_SIZE: int = 1000

    # Storage Backend (v0.51.0 - RuntimeStorage by default)
    STORAGE_BACKEND: str = "runtime"  # "memory" or "runtime"

    # Token Storage
    TOKEN_STORAGE_MAX_SIZE: int = 1000000  # Maximum number of tokens
    TOKEN_AUTO_CLEANUP: bool = True  # Auto-cleanup inactive tokens
    TOKEN_CLEANUP_INTERVAL: int = 3600  # Cleanup interval in seconds

    # Grid Configuration
    GRID_ENABLED: bool = True  # Enable Grid functionality
    GRID_MAX_INSTANCES: int = 100  # Maximum number of grid instances
    GRID_DEFAULT_BUCKET_SIZE: float = 10.0  # Default spatial bucket size
    GRID_DEFAULT_DENSITY_THRESHOLD: float = 0.5  # Default density threshold
    GRID_DEFAULT_MIN_FIELD_NODES: int = 3  # Default min nodes for field

    # CDNA Configuration
    CDNA_ENABLED: bool = True  # Enable CDNA functionality
    CDNA_DEFAULT_PROFILE: str = "explorer"  # Default CDNA profile
    CDNA_HISTORY_LIMIT: int = 1000  # Maximum history entries
    CDNA_QUARANTINE_DURATION: int = 300  # Quarantine duration in seconds

    # Feature Flags (for gradual rollout)
    ENABLE_NEW_TOKEN_API: bool = True  # Enable new Token API
    ENABLE_NEW_GRID_API: bool = True  # Enable new Grid API
    ENABLE_NEW_CDNA_API: bool = True  # Enable new CDNA API

    class Config:
        env_file = ".env"
        case_sensitive = True


# Global settings instance
settings = Settings()
