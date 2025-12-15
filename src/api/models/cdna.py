"""
CDNA Models

Request and response models for CDNA (Cognitive DNA) endpoints.
Ported from MVP API with enhancements for production use.
"""

from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Any


class ProfileInfo(BaseModel):
    """Information about a CDNA profile."""

    name: str = Field(..., description="Profile display name")
    scales: List[float] = Field(..., description="Dimension scales (8 values)")
    description: str = Field(..., description="Profile description")
    plasticity: float = Field(..., description="Plasticity level (0.0-1.0)")
    evolution_rate: float = Field(..., description="Evolution rate (0.0-1.0)")
    restricted: Optional[bool] = Field(False, description="Is profile restricted")
    max_change: Optional[float] = Field(None, description="Maximum change allowed")

    class Config:
        json_schema_extra = {
            "example": {
                "name": "Explorer",
                "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
                "description": "Свободная структура, высокая пластичность",
                "plasticity": 0.8,
                "evolution_rate": 0.5,
                "restricted": False
            }
        }


class CDNAConfig(BaseModel):
    """Current CDNA configuration."""

    version: str = Field(..., description="CDNA version")
    profile: str = Field(..., description="Active profile ID")
    dimension_scales: List[float] = Field(..., min_length=8, max_length=8, description="Current dimension scales")
    timestamp: Optional[str] = Field(None, description="Last update timestamp (ISO 8601)")

    class Config:
        json_schema_extra = {
            "example": {
                "version": "2.1.0",
                "profile": "explorer",
                "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
                "timestamp": "2024-12-15T10:30:00Z"
            }
        }


class CDNAUpdateRequest(BaseModel):
    """Request to update CDNA configuration."""

    profile: Optional[str] = Field(None, description="Profile to switch to")
    dimension_scales: Optional[List[float]] = Field(
        None,
        min_length=8,
        max_length=8,
        description="Custom dimension scales"
    )
    should_validate: bool = Field(True, description="Validate before applying")

    class Config:
        json_schema_extra = {
            "example": {
                "profile": "creative",
                "should_validate": True
            }
        }


class CDNAStatusResponse(BaseModel):
    """Response for CDNA status."""

    cdna: CDNAConfig = Field(..., description="Current CDNA configuration")
    quarantine: Dict[str, Any] = Field(..., description="Quarantine state")
    history_count: int = Field(..., description="Number of history entries")

    class Config:
        json_schema_extra = {
            "example": {
                "cdna": {
                    "version": "2.1.0",
                    "profile": "explorer",
                    "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
                    "timestamp": "2024-12-15T10:30:00Z"
                },
                "quarantine": {
                    "active": False,
                    "time_left": 300,
                    "metrics": {}
                },
                "history_count": 5
            }
        }


class CDNAProfilesResponse(BaseModel):
    """Response for profiles list."""

    profiles: Dict[str, ProfileInfo] = Field(..., description="Available profiles")
    current: str = Field(..., description="Current active profile")

    class Config:
        json_schema_extra = {
            "example": {
                "profiles": {
                    "explorer": {
                        "name": "Explorer",
                        "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
                        "description": "High plasticity",
                        "plasticity": 0.8,
                        "evolution_rate": 0.5
                    }
                },
                "current": "explorer"
            }
        }


class ProfileSwitchResponse(BaseModel):
    """Response for profile switch."""

    success: bool = Field(..., description="Operation success")
    old_profile: str = Field(..., description="Previous profile")
    new_profile: str = Field(..., description="New active profile")
    scales: List[float] = Field(..., description="New dimension scales")


class QuarantineStatus(BaseModel):
    """Quarantine mode status."""

    active: bool = Field(..., description="Is quarantine active")
    time_left: int = Field(..., description="Seconds remaining")
    metrics: Dict[str, int] = Field(..., description="Quarantine metrics")

    class Config:
        json_schema_extra = {
            "example": {
                "active": True,
                "time_left": 240,
                "metrics": {
                    "memory_growth": 0,
                    "connection_breaks": 0,
                    "token_churn": 0
                }
            }
        }


class QuarantineStartResponse(BaseModel):
    """Response for starting quarantine."""

    success: bool = Field(..., description="Operation success")
    message: str = Field(..., description="Status message")
    duration: int = Field(..., description="Quarantine duration in seconds")


class QuarantineStopResponse(BaseModel):
    """Response for stopping quarantine."""

    success: bool = Field(..., description="Operation success")
    message: str = Field(..., description="Status message")
    applied: bool = Field(..., description="Were changes applied")


class ValidationResult(BaseModel):
    """Result of CDNA validation."""

    valid: bool = Field(..., description="Is configuration valid")
    warnings: List[str] = Field(default_factory=list, description="Validation warnings")
    errors: List[str] = Field(default_factory=list, description="Validation errors")

    class Config:
        json_schema_extra = {
            "example": {
                "valid": True,
                "warnings": ["Dimension 4 value 25.0 in caution zone"],
                "errors": []
            }
        }


class ValidateRequest(BaseModel):
    """Request to validate dimension scales."""

    scales: List[float] = Field(..., min_length=8, max_length=8, description="Scales to validate")

    class Config:
        json_schema_extra = {
            "example": {
                "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0]
            }
        }


class CDNAHistoryResponse(BaseModel):
    """Response for CDNA history."""

    history: List[Dict[str, Any]] = Field(..., description="Configuration history")
    total: int = Field(..., description="Total history entries")

    class Config:
        json_schema_extra = {
            "example": {
                "history": [
                    {
                        "action": "profile_switch",
                        "from": "explorer",
                        "to": "creative",
                        "timestamp": "2024-12-15T10:30:00Z"
                    }
                ],
                "total": 10
            }
        }


class CDNAExportResponse(BaseModel):
    """Response for CDNA export."""

    success: bool = Field(..., description="Export success")
    data: Dict[str, Any] = Field(..., description="Exported configuration")
    filename: str = Field(..., description="Suggested filename")


class CDNAResetResponse(BaseModel):
    """Response for CDNA reset."""

    success: bool = Field(..., description="Reset success")
    message: str = Field(..., description="Status message")
    cdna: CDNAConfig = Field(..., description="New configuration")
