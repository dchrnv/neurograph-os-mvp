"""
NeuroGraph REST API - FastAPI Application

Version: 1.0.0 (v0.48.0)
Base URL: http://localhost:8000/api/v1
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse
import time
from datetime import datetime
import logging

from .config import settings
from .routers import health, query, status, modules, metrics
from .models.response import ErrorResponse

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Create FastAPI app
app = FastAPI(
    title="NeuroGraph API",
    description="REST API for NeuroGraph semantic knowledge system",
    version="1.0.0",
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json",
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.CORS_ORIGINS,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Request ID and timing middleware
@app.middleware("http")
async def add_process_time_header(request, call_next):
    start_time = time.time()
    response = await call_next(request)
    process_time = (time.time() - start_time) * 1000  # Convert to ms
    response.headers["X-Process-Time"] = f"{process_time:.2f}ms"
    return response

# Include routers
app.include_router(health.router, prefix="/api/v1", tags=["Health"])
app.include_router(query.router, prefix="/api/v1", tags=["Query"])
app.include_router(status.router, prefix="/api/v1", tags=["Status"])
app.include_router(modules.router, prefix="/api/v1", tags=["Modules"])
app.include_router(metrics.router, prefix="/api/v1", tags=["Metrics"])

# Global exception handler
@app.exception_handler(Exception)
async def global_exception_handler(request, exc: Exception):
    logger.error(f"Global exception: {exc}", exc_info=True)
    return JSONResponse(
        status_code=500,
        content=ErrorResponse(
            code="INTERNAL_ERROR",
            message="Internal server error",
            details={"error": str(exc)}
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
        "version": "1.0.0",
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
