"""
Middleware package for NeuroGraph API.

Note: Most middleware are in ../middleware.py (legacy structure).
This package contains new middleware (v0.58.0+).
"""

# Legacy middleware from middleware.py
from ..middleware import (
    CorrelationIDMiddleware,
    RequestLoggingMiddleware,
    ErrorLoggingMiddleware
)

# New middleware from this package
from .rate_limit import RateLimitMiddleware
from .security import (
    SecurityHeadersMiddleware,
    RequestSizeLimitMiddleware,
    InputSanitizationMiddleware
)

__all__ = [
    "CorrelationIDMiddleware",
    "RequestLoggingMiddleware",
    "ErrorLoggingMiddleware",
    "RateLimitMiddleware",
    "SecurityHeadersMiddleware",
    "RequestSizeLimitMiddleware",
    "InputSanitizationMiddleware",
]
