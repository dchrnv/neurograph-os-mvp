"""
FastAPI application main entry point.
Includes WebSocket support and REST API endpoints.
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager

from src.infrastructure.websocket.server import connection_manager
from src.infrastructure.api.websocket_routes import router as ws_router
from src.core.utils.logger import get_logger

logger = get_logger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager."""
    # Startup
    logger.info("Starting NeuroGraph OS API...")
    await connection_manager.start()
    logger.info("WebSocket connection manager started")
    
    yield
    
    # Shutdown
    logger.info("Shutting down NeuroGraph OS API...")
    await connection_manager.stop()
    logger.info("WebSocket connection manager stopped")


# Create FastAPI application
app = FastAPI(
    title="NeuroGraph OS API",
    description="Token-based spatial computing system with real-time WebSocket support",
    version="0.3.0",
    lifespan=lifespan
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include WebSocket routes
app.include_router(ws_router, tags=["WebSocket"])


@app.get("/")
async def root():
    """Root endpoint."""
    return {
        "name": "NeuroGraph OS API",
        "version": "0.3.0",
        "status": "running",
        "websocket_url": "/ws",
        "docs_url": "/docs"
    }


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {
        "status": "healthy",
        "websocket_connections": connection_manager.get_connection_count()
    }


if __name__ == "__main__":
    import uvicorn
    
    uvicorn.run(
        "main:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )