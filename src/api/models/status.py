"""
Status Models

Models for system status and health endpoints.
"""

from pydantic import BaseModel, Field
from typing import Dict, Optional
from enum import Enum


class ComponentState(str, Enum):
    """Component state enum."""
    RUNNING = "running"
    STOPPED = "stopped"
    ERROR = "error"
    INITIALIZING = "initializing"


class ComponentStatus(BaseModel):
    """Status of a system component."""

    state: ComponentState = Field(..., description="Component state")
    uptime_seconds: Optional[float] = Field(None, description="Uptime in seconds")
    error_message: Optional[str] = Field(None, description="Error message if in error state")


class SystemMetrics(BaseModel):
    """System metrics."""

    tokens_total: int = Field(0, description="Total tokens in system")
    tokens_active: int = Field(0, description="Active tokens")
    connections_total: int = Field(0, description="Total connections")
    connections_active: int = Field(0, description="Active connections")
    memory_usage_mb: float = Field(0.0, description="Memory usage in MB")
    cpu_usage_percent: float = Field(0.0, description="CPU usage percentage")


class StatusResponse(BaseModel):
    """System status response."""

    state: str = Field("initializing", description="Overall system state")
    uptime_seconds: float = Field(0.0, description="System uptime in seconds")
    tokens: Dict[str, int] = Field(
        default_factory=lambda: {"total": 0, "active": 0},
        description="Token statistics"
    )
    connections: Dict[str, int] = Field(
        default_factory=lambda: {"total": 0, "active": 0},
        description="Connection statistics"
    )
    memory_usage_mb: float = Field(0.0, description="Memory usage in MB")
    cpu_usage_percent: float = Field(0.0, description="CPU usage percentage")
    components: Dict[str, str] = Field(
        default_factory=dict,
        description="Component states"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "state": "running",
                "uptime_seconds": 3600.5,
                "tokens": {"total": 50000, "active": 12847},
                "connections": {"total": 1247832, "active": 523419},
                "memory_usage_mb": 847.3,
                "cpu_usage_percent": 23.5,
                "components": {
                    "runtime": "running",
                    "gateway": "running",
                    "intuition_engine": "running",
                    "guardian": "running"
                }
            }
        }


class HealthResponse(BaseModel):
    """Health check response."""

    status: str = Field("healthy", description="Health status")
    uptime_seconds: float = Field(0.0, description="Uptime in seconds")
    version: str = Field("1.0.0", description="API version")


class ReadinessResponse(BaseModel):
    """Readiness check response."""

    ready: bool = Field(False, description="Whether system is ready")
    checks: Dict[str, str] = Field(
        default_factory=dict,
        description="Individual component readiness checks"
    )

    class Config:
        json_schema_extra = {
            "example": {
                "ready": True,
                "checks": {
                    "runtime": "ok",
                    "bootstrap": "ok",
                    "gateway": "ok"
                }
            }
        }
