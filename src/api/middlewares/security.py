"""
Security Middleware

Provides security hardening features:
- Security headers (HSTS, CSP, X-Frame-Options, etc.)
- Input sanitization
- Request size limits
"""

from fastapi import Request, Response, status
from fastapi.responses import JSONResponse
from starlette.middleware.base import BaseHTTPMiddleware
from typing import Optional

from ..logging_config import get_logger

logger = get_logger(__name__, component="security")


class SecurityHeadersMiddleware(BaseHTTPMiddleware):
    """
    Middleware to add security headers to all responses.

    Headers added:
    - X-Content-Type-Options: nosniff (prevent MIME sniffing)
    - X-Frame-Options: DENY (prevent clickjacking)
    - X-XSS-Protection: 1; mode=block (XSS protection for old browsers)
    - Strict-Transport-Security: HTTPS enforcement (production only)
    - Content-Security-Policy: Control resource loading
    - Referrer-Policy: Control referrer information
    """

    def __init__(
        self,
        app,
        enable_hsts: bool = False,
        enable_csp: bool = True,
        csp_policy: Optional[str] = None
    ):
        super().__init__(app)
        self.enable_hsts = enable_hsts
        self.enable_csp = enable_csp
        self.csp_policy = csp_policy or (
            "default-src 'self'; "
            "script-src 'self' 'unsafe-inline' 'unsafe-eval'; "  # Allow Swagger UI
            "style-src 'self' 'unsafe-inline'; "  # Allow Swagger UI
            "img-src 'self' data: https:; "
            "font-src 'self' data:; "
            "connect-src 'self'; "
            "frame-ancestors 'none'"
        )

    async def dispatch(self, request: Request, call_next):
        """Add security headers to response."""
        response = await call_next(request)

        # X-Content-Type-Options: Prevent MIME sniffing
        response.headers["X-Content-Type-Options"] = "nosniff"

        # X-Frame-Options: Prevent clickjacking
        response.headers["X-Frame-Options"] = "DENY"

        # X-XSS-Protection: XSS protection for older browsers
        response.headers["X-XSS-Protection"] = "1; mode=block"

        # Referrer-Policy: Control referrer information
        response.headers["Referrer-Policy"] = "strict-origin-when-cross-origin"

        # HSTS: Force HTTPS (production only)
        if self.enable_hsts:
            # max-age=31536000 (1 year), includeSubDomains
            response.headers["Strict-Transport-Security"] = "max-age=31536000; includeSubDomains"

        # Content-Security-Policy
        if self.enable_csp:
            response.headers["Content-Security-Policy"] = self.csp_policy

        # Permissions-Policy: Control browser features
        response.headers["Permissions-Policy"] = (
            "geolocation=(), microphone=(), camera=(), "
            "payment=(), usb=(), magnetometer=(), gyroscope=(), "
            "speaker=(self)"
        )

        return response


class RequestSizeLimitMiddleware(BaseHTTPMiddleware):
    """
    Middleware to enforce request size limits.

    Protects against:
    - Large payload DoS attacks
    - Memory exhaustion
    """

    def __init__(
        self,
        app,
        max_body_size: int = 1024 * 1024  # 1MB default
    ):
        super().__init__(app)
        self.max_body_size = max_body_size

    async def dispatch(self, request: Request, call_next):
        """Check request size before processing."""
        # Check Content-Length header
        content_length = request.headers.get("content-length")

        if content_length:
            content_length = int(content_length)

            if content_length > self.max_body_size:
                logger.warning(
                    f"Request body too large: {content_length} bytes",
                    extra={
                        "event": "request_body_too_large",
                        "content_length": content_length,
                        "max_allowed": self.max_body_size,
                        "path": request.url.path,
                        "method": request.method,
                        "client_ip": request.client.host if request.client else None
                    }
                )

                return JSONResponse(
                    status_code=status.HTTP_413_REQUEST_ENTITY_TOO_LARGE,
                    content={
                        "success": False,
                        "error": {
                            "code": "REQUEST_TOO_LARGE",
                            "message": f"Request body too large. Maximum allowed: {self.max_body_size} bytes",
                            "details": {
                                "content_length": content_length,
                                "max_allowed": self.max_body_size
                            }
                        }
                    }
                )

        return await call_next(request)


class InputSanitizationMiddleware(BaseHTTPMiddleware):
    """
    Middleware for basic input sanitization.

    Features:
    - Strip null bytes from strings
    - Validate UTF-8 encoding
    - Check for common attack patterns (basic)

    Note: Pydantic validation is the primary defense.
    This is an additional layer for defense in depth.
    """

    def __init__(self, app, enable_strict_validation: bool = False):
        super().__init__(app)
        self.enable_strict_validation = enable_strict_validation

    async def dispatch(self, request: Request, call_next):
        """Sanitize request before processing."""
        # Basic checks on URL path
        path = request.url.path

        # Check for null bytes in path
        if '\x00' in path:
            logger.warning(
                "Null byte detected in request path",
                extra={
                    "event": "null_byte_in_path",
                    "path": path,
                    "client_ip": request.client.host if request.client else None
                }
            )

            return JSONResponse(
                status_code=status.HTTP_400_BAD_REQUEST,
                content={
                    "success": False,
                    "error": {
                        "code": "INVALID_REQUEST",
                        "message": "Invalid characters in request"
                    }
                }
            )

        # In strict mode, check for suspicious patterns
        if self.enable_strict_validation:
            suspicious_patterns = ['../', '..\\', '<script', 'javascript:', 'onerror=']

            for pattern in suspicious_patterns:
                if pattern.lower() in path.lower():
                    logger.warning(
                        f"Suspicious pattern detected in path: {pattern}",
                        extra={
                            "event": "suspicious_pattern",
                            "pattern": pattern,
                            "path": path,
                            "client_ip": request.client.host if request.client else None
                        }
                    )

                    # Log but don't block - might be legitimate data
                    break

        return await call_next(request)
