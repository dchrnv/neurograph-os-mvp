"""
Response Models

Standard response wrappers for all API endpoints.
"""

from pydantic import BaseModel, Field
from typing import Any, Optional, Dict
from datetime import datetime
import uuid


class MetaData(BaseModel):
    """Response metadata."""

    request_id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    processing_time_ms: Optional[float] = None
    version: str = "1.0.0"
    timestamp: datetime = Field(default_factory=datetime.utcnow)

    class Config:
        json_encoders = {
            datetime: lambda v: v.isoformat() + 'Z'
        }


class ErrorResponse(BaseModel):
    """Error response structure."""

    code: str = Field(..., description="Error code (e.g., VALIDATION_ERROR)")
    message: str = Field(..., description="Human-readable error message")
    details: Optional[Dict[str, Any]] = Field(None, description="Additional error details")


class SuccessResponse(BaseModel):
    """Simple success response structure."""

    success: bool = Field(True, description="Success status")
    message: str = Field(..., description="Success message")
    data: Optional[Dict[str, Any]] = Field(None, description="Optional data")


class ApiResponse(BaseModel):
    """Standard API response wrapper."""

    success: bool = Field(..., description="Whether the request succeeded")
    data: Optional[Any] = Field(None, description="Response data")
    meta: MetaData = Field(default_factory=MetaData, description="Response metadata")
    error: Optional[ErrorResponse] = Field(None, description="Error information if failed")

    @classmethod
    def success_response(cls, data: Any, processing_time_ms: Optional[float] = None):
        """Create a successful response."""
        meta = MetaData(processing_time_ms=processing_time_ms)
        return cls(success=True, data=data, meta=meta, error=None)

    @classmethod
    def error_response(
        cls,
        code: str,
        message: str,
        details: Optional[Dict[str, Any]] = None,
        processing_time_ms: Optional[float] = None
    ):
        """Create an error response."""
        meta = MetaData(processing_time_ms=processing_time_ms)
        error = ErrorResponse(code=code, message=message, details=details)
        return cls(success=False, data=None, meta=meta, error=error)
