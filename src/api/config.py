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

    # API Configuration
    API_V1_PREFIX: str = "/api/v1"
    PROJECT_NAME: str = "NeuroGraph API"
    VERSION: str = "1.0.0"

    # CORS
    CORS_ORIGINS: List[str] = [
        "http://localhost:3000",
        "http://localhost:8000",
        "http://127.0.0.1:3000",
        "http://127.0.0.1:8000",
    ]

    # Server
    HOST: str = "0.0.0.0"
    PORT: int = 8000
    WORKERS: int = 4

    # Security
    SECRET_KEY: str = "dev-secret-key-change-in-production"
    JWT_ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 60

    # Rate Limiting
    RATE_LIMIT_ENABLED: bool = True
    RATE_LIMIT_PER_MINUTE: int = 100

    # NeuroGraph Runtime
    NEUROGRAPH_BOOTSTRAP_PATH: str = os.path.expanduser("~/data/glove.6B.50d.txt")
    NEUROGRAPH_BOOTSTRAP_LIMIT: int = 50000
    NEUROGRAPH_DIMENSIONS: int = 50
    NEUROGRAPH_GRID_SIZE: int = 1000

    class Config:
        env_file = ".env"
        case_sensitive = True


# Global settings instance
settings = Settings()
