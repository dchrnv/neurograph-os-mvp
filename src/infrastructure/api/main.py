"""
FastAPI application main entry point.
Includes WebSocket support and REST API endpoints.
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import RedirectResponse
from contextlib import asynccontextmanager

from src.infrastructure.websocket.server import connection_manager
from src.infrastructure.api.websocket_routes import router as ws_router
from src.infrastructure.api.routes.tokens import router as tokens_router
from src.infrastructure.api.routes.graph import router as graph_router
from src.infrastructure.api.routes.system import system_router, experience_router
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
    lifespan=lifespan,
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(ws_router, tags=["WebSocket"])
app.include_router(tokens_router, prefix="/api/v1")
app.include_router(graph_router, prefix="/api/v1")
app.include_router(system_router, prefix="/api/v1")
app.include_router(experience_router, prefix="/api/v1")


@app.get("/")
async def root():
    """Root endpoint - redirects to docs."""
    return RedirectResponse(url="/docs")


@app.get("/api")
async def api_info():
    """API information endpoint."""
    return {
        "name": "NeuroGraph OS API",
        "version": "0.3.0",
        "status": "running",
        "endpoints": {
            "docs": "/docs",
            "openapi": "/openapi.json",
            "websocket": "/ws",
            "tokens": "/api/v1/tokens",
            "graph": "/api/v1/graph",
            "experience": "/api/v1/experience",
            "system": "/api/v1/system"
        }
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