"""
Error handlers for NeuroGraph API.

Provides centralized error handling with structured responses.
"""

from fastapi import Request, status
from fastapi.responses import JSONResponse
from fastapi.exceptions import RequestValidationError
from pydantic import ValidationError
import logging

from .exceptions import NeuroGraphException
from .logging_config import get_logger

logger = get_logger(__name__, component="error_handler")


async def neurograph_exception_handler(
    request: Request,
    exc: NeuroGraphException
) -> JSONResponse:
    """
    Handle custom NeuroGraph exceptions.

    Returns structured error response with error code, message, and details.
    """
    logger.warning(
        f"{exc.error_code}: {exc.message}",
        extra={
            "event": "exception_raised",
            "error_code": exc.error_code,
            "status_code": exc.status_code,
            "path": request.url.path,
            "method": request.method,
            "details": exc.details
        }
    )

    return JSONResponse(
        status_code=exc.status_code,
        content=exc.detail,  # Already structured
        headers=exc.headers
    )


async def validation_exception_handler(
    request: Request,
    exc: RequestValidationError
) -> JSONResponse:
    """
    Handle Pydantic validation errors.

    Transforms validation errors into structured NeuroGraph format.
    """
    # Extract validation errors
    errors = []
    for error in exc.errors():
        field = ".".join(str(loc) for loc in error["loc"])
        errors.append({
            "field": field,
            "message": error["msg"],
            "type": error["type"]
        })

    logger.warning(
        f"Validation error: {len(errors)} field(s)",
        extra={
            "event": "validation_error",
            "path": request.url.path,
            "method": request.method,
            "errors": errors
        }
    )

    return JSONResponse(
        status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
        content={
            "success": False,
            "error": {
                "code": "VALIDATION_ERROR",
                "message": "Request validation failed",
                "details": {
                    "errors": errors,
                    "count": len(errors)
                }
            }
        }
    )


async def generic_exception_handler(
    request: Request,
    exc: Exception
) -> JSONResponse:
    """
    Handle unexpected exceptions.

    Logs full error details but returns generic message to client.
    """
    logger.error(
        f"Unexpected error: {type(exc).__name__}: {str(exc)}",
        extra={
            "event": "unexpected_exception",
            "exception_type": type(exc).__name__,
            "path": request.url.path,
            "method": request.method
        },
        exc_info=True
    )

    # Return generic error (don't leak implementation details)
    return JSONResponse(
        status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
        content={
            "success": False,
            "error": {
                "code": "INTERNAL_SERVER_ERROR",
                "message": "An unexpected error occurred. Please try again later.",
                "details": {
                    "type": type(exc).__name__ if logger.level <= logging.DEBUG else None
                }
            }
        }
    )
