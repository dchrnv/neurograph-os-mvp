
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
NeuroGraph REST API - FastAPI Application

Version: 1.0.0 (v0.49.0)
Base URL: http://localhost:8000/api/v1

Phase 1 & 2 Complete: Storage layer + Token/Grid/CDNA routers
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import time
from datetime import datetime

from .config import settings
from .routers import health, query, status, modules, metrics
from .routers import tokens, grid, cdna
from .models.response import ErrorResponse

# v0.52.0: Structured logging and middleware
from .logging_config import setup_logging, get_logger
from .middleware import (
    CorrelationIDMiddleware,
    RequestLoggingMiddleware,
    ErrorLoggingMiddleware
)

# Configure structured logging
setup_logging(
    level=settings.LOG_LEVEL,
    json_format=settings.LOG_JSON_FORMAT,
    correlation_tracking=settings.LOG_CORRELATION_TRACKING
)
logger = get_logger(__name__, service="api")

# Create FastAPI app
app = FastAPI(
    title="NeuroGraph API",
    description="REST API for NeuroGraph semantic knowledge system",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json",
)

# v0.52.0: Observability middlewares (order matters!)
# 1. Error logging (outermost - catches everything)
app.add_middleware(ErrorLoggingMiddleware, debug=settings.DEBUG)

# 2. Request logging (logs all requests/responses)
app.add_middleware(
    RequestLoggingMiddleware,
    skip_paths=["/health", "/api/v1/health", "/api/v1/health/ready"],
    log_request_body=settings.LOG_REQUEST_BODY,
    log_response_body=settings.LOG_RESPONSE_BODY
)

# 3. Correlation ID (sets context for logging)
app.add_middleware(CorrelationIDMiddleware)

# 4. CORS (innermost - before route handlers)
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.CORS_ORIGINS,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
# System routers
app.include_router(health.router, prefix="/api/v1", tags=["Health"])
app.include_router(status.router, prefix="/api/v1", tags=["Status"])
app.include_router(metrics.router, prefix="/api/v1", tags=["Metrics"])
app.include_router(modules.router, prefix="/api/v1", tags=["Modules"])

# Core functionality routers (Phase 2)
app.include_router(tokens.router, prefix="/api/v1", tags=["Tokens"])
app.include_router(grid.router, prefix="/api/v1", tags=["Grid"])
app.include_router(cdna.router, prefix="/api/v1", tags=["CDNA"])

# Query router (will be enhanced in Phase 4)
app.include_router(query.router, prefix="/api/v1", tags=["Query"])

# Global exception handler (fallback - middleware should catch most)
@app.exception_handler(Exception)
async def global_exception_handler(request, exc: Exception):
    logger.exception(
        "Global exception handler triggered",
        extra={
            "event": "global_exception",
            "path": request.url.path,
            "method": request.method,
        }
    )
    return JSONResponse(
        status_code=500,
        content=ErrorResponse(
            code="INTERNAL_ERROR",
            message="Internal server error",
            details={"error": str(exc)} if settings.DEBUG else None
        ).dict()
    )

# Startup event
@app.on_event("startup")
async def startup_event():
    logger.info("NeuroGraph API starting up...")
    logger.info(f"Environment: {settings.ENVIRONMENT}")
    logger.info(f"CORS origins: {settings.CORS_ORIGINS}")
    # TODO: Initialize neurograph runtime here

# Shutdown event
@app.on_event("shutdown")
async def shutdown_event():
    logger.info("NeuroGraph API shutting down...")
    # TODO: Cleanup neurograph runtime here

# Root endpoint
@app.get("/", include_in_schema=False)
async def root():
    return {
        "name": "NeuroGraph API",
        "version": "1.0.0 (v0.49.0)",
        "phase": "Phase 2 Complete - Storage + Token/Grid/CDNA",
        "storage_backend": settings.STORAGE_BACKEND,
        "features": {
            "tokens": settings.ENABLE_NEW_TOKEN_API,
            "grid": settings.ENABLE_NEW_GRID_API,
            "cdna": settings.ENABLE_NEW_CDNA_API
        },
        "docs": "/docs",
        "health": "/api/v1/health"
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        "src.api.main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
