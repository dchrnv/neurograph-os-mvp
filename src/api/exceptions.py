"""
Custom exceptions for NeuroGraph API.

Provides structured error handling with detailed error codes and messages.
"""

from typing import Optional, Dict, Any
from fastapi import HTTPException, status


class NeuroGraphException(HTTPException):
    """
    Base exception for NeuroGraph API.

    Provides structured error responses with error codes,
    messages, and optional additional details.
    """

    def __init__(
        self,
        status_code: int,
        error_code: str,
        message: str,
        details: Optional[Dict[str, Any]] = None,
        headers: Optional[Dict[str, str]] = None
    ):
        self.error_code = error_code
        self.message = message
        self.details = details or {}

        # Create structured detail for FastAPI
        detail = {
            "success": False,
            "error": {
                "code": error_code,
                "message": message,
                "details": self.details
            }
        }

        super().__init__(
            status_code=status_code,
            detail=detail,
            headers=headers
        )


# Authentication Exceptions

class AuthenticationError(NeuroGraphException):
    """Base authentication error."""

    def __init__(
        self,
        error_code: str,
        message: str,
        details: Optional[Dict[str, Any]] = None
    ):
        super().__init__(
            status_code=status.HTTP_401_UNAUTHORIZED,
            error_code=error_code,
            message=message,
            details=details,
            headers={"WWW-Authenticate": "Bearer"}
        )


class InvalidCredentialsError(AuthenticationError):
    """Invalid username or password."""

    def __init__(self, details: Optional[Dict[str, Any]] = None):
        super().__init__(
            error_code="INVALID_CREDENTIALS",
            message="Invalid username or password",
            details=details
        )


class InvalidTokenError(AuthenticationError):
    """Invalid or expired JWT token."""

    def __init__(self, reason: str = "Token is invalid or expired"):
        super().__init__(
            error_code="INVALID_TOKEN",
            message=reason,
            details={"reason": reason}
        )


class TokenExpiredError(AuthenticationError):
    """JWT token has expired."""

    def __init__(self):
        super().__init__(
            error_code="TOKEN_EXPIRED",
            message="Access token has expired. Please refresh your token.",
            details={"action": "Use /api/v1/auth/refresh to get a new access token"}
        )


class InvalidAPIKeyError(AuthenticationError):
    """Invalid API key."""

    def __init__(self):
        super().__init__(
            error_code="INVALID_API_KEY",
            message="API key is invalid, expired, or revoked",
            details={"header": "X-API-Key"}
        )


# Authorization Exceptions

class AuthorizationError(NeuroGraphException):
    """Base authorization error."""

    def __init__(
        self,
        error_code: str,
        message: str,
        details: Optional[Dict[str, Any]] = None
    ):
        super().__init__(
            status_code=status.HTTP_403_FORBIDDEN,
            error_code=error_code,
            message=message,
            details=details
        )


class PermissionDeniedError(AuthorizationError):
    """User does not have required permission."""

    def __init__(self, permission: str, action: Optional[str] = None):
        details = {
            "required_permission": permission,
            "action": action or f"This action requires '{permission}' permission"
        }

        super().__init__(
            error_code="PERMISSION_DENIED",
            message=f"Permission denied: {permission} required",
            details=details
        )


class InsufficientPrivilegesError(AuthorizationError):
    """User role does not have sufficient privileges."""

    def __init__(self, required_role: str, current_role: str):
        super().__init__(
            error_code="INSUFFICIENT_PRIVILEGES",
            message=f"This action requires '{required_role}' role or higher",
            details={
                "required_role": required_role,
                "current_role": current_role
            }
        )


# Resource Exceptions

class ResourceError(NeuroGraphException):
    """Base resource error."""
    pass


class ResourceNotFoundError(ResourceError):
    """Requested resource not found."""

    def __init__(self, resource_type: str, resource_id: Any):
        super().__init__(
            status_code=status.HTTP_404_NOT_FOUND,
            error_code="RESOURCE_NOT_FOUND",
            message=f"{resource_type} not found",
            details={
                "resource_type": resource_type,
                "resource_id": str(resource_id)
            }
        )


class ResourceAlreadyExistsError(ResourceError):
    """Resource already exists."""

    def __init__(self, resource_type: str, identifier: str):
        super().__init__(
            status_code=status.HTTP_409_CONFLICT,
            error_code="RESOURCE_ALREADY_EXISTS",
            message=f"{resource_type} already exists",
            details={
                "resource_type": resource_type,
                "identifier": identifier
            }
        )


class ResourceLimitExceededError(ResourceError):
    """Resource limit exceeded."""

    def __init__(self, resource_type: str, limit: int, current: int):
        super().__init__(
            status_code=status.HTTP_400_BAD_REQUEST,
            error_code="RESOURCE_LIMIT_EXCEEDED",
            message=f"Maximum {resource_type} limit ({limit}) exceeded",
            details={
                "resource_type": resource_type,
                "limit": limit,
                "current": current
            }
        )


# Validation Exceptions

class ValidationError(NeuroGraphException):
    """Base validation error."""

    def __init__(
        self,
        error_code: str,
        message: str,
        details: Optional[Dict[str, Any]] = None
    ):
        super().__init__(
            status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
            error_code=error_code,
            message=message,
            details=details
        )


class InvalidInputError(ValidationError):
    """Invalid input data."""

    def __init__(self, field: str, reason: str):
        super().__init__(
            error_code="INVALID_INPUT",
            message=f"Invalid input: {field}",
            details={
                "field": field,
                "reason": reason
            }
        )


class MissingRequiredFieldError(ValidationError):
    """Required field is missing."""

    def __init__(self, field: str):
        super().__init__(
            error_code="MISSING_REQUIRED_FIELD",
            message=f"Required field missing: {field}",
            details={"field": field}
        )


# Rate Limiting Exceptions

class RateLimitExceededError(NeuroGraphException):
    """Rate limit exceeded."""

    def __init__(self, limit: int, retry_after: int = 1):
        super().__init__(
            status_code=status.HTTP_429_TOO_MANY_REQUESTS,
            error_code="RATE_LIMIT_EXCEEDED",
            message="Rate limit exceeded. Please try again later.",
            details={
                "rate_limit": limit,
                "retry_after": retry_after,
                "unit": "requests per minute"
            },
            headers={"Retry-After": str(retry_after)}
        )


# Service Exceptions

class ServiceError(NeuroGraphException):
    """Base service error."""
    pass


class ServiceUnavailableError(ServiceError):
    """Service temporarily unavailable."""

    def __init__(self, service_name: str, reason: Optional[str] = None):
        details = {"service": service_name}
        if reason:
            details["reason"] = reason

        super().__init__(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            error_code="SERVICE_UNAVAILABLE",
            message=f"{service_name} is currently unavailable",
            details=details
        )


class InternalServerError(ServiceError):
    """Internal server error."""

    def __init__(self, message: str = "An internal error occurred"):
        super().__init__(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            error_code="INTERNAL_SERVER_ERROR",
            message=message,
            details={}
        )


# Configuration Exceptions

class ConfigurationError(NeuroGraphException):
    """Configuration error."""

    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None):
        super().__init__(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            error_code="CONFIGURATION_ERROR",
            message=message,
            details=details or {}
        )
