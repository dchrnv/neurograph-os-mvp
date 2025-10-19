"""
NeuroGraph OS - MVP API
Simplified version without complex dependencies
"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import RedirectResponse, JSONResponse

# Import only Token v2.0 routes (no WebSocket, no old routes)
from src.infrastructure.api.routes.tokens_v2 import router as tokens_v2_router

# Create FastAPI application
app = FastAPI(
    title="NeuroGraph OS - MVP",
    description="Token v2.0 based spatial computing system (MVP version)",
    version="0.10.0",
    docs_url="/docs",
    redoc_url="/redoc",
    openapi_url="/openapi.json"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # For MVP - allow all origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include only Token v2.0 router
app.include_router(tokens_v2_router)


@app.get("/")
async def root():
    """Root endpoint - redirects to docs."""
    return RedirectResponse(url="/docs")


@app.get("/api")
async def api_info():
    """API information endpoint."""
    return {
        "name": "NeuroGraph OS - MVP",
        "version": "0.10.0",
        "status": "running",
        "token_version": "2.0",
        "features": [
            "Token v2.0 with 8 semantic coordinate spaces",
            "In-memory token storage",
            "RESTful API",
            "OpenAPI documentation"
        ],
        "endpoints": {
            "docs": "/docs",
            "openapi": "/openapi.json",
            "tokens": "/api/v1/tokens",
            "health": "/health"
        }
    }


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    from src.infrastructure.api.routes.tokens_v2 import TOKEN_STORAGE

    return {
        "status": "healthy",
        "tokens_in_memory": len(TOKEN_STORAGE)
    }


@app.exception_handler(Exception)
async def global_exception_handler(request, exc):
    """Global exception handler for better error messages"""
    return JSONResponse(
        status_code=500,
        content={
            "error": "Internal server error",
            "detail": str(exc),
            "path": str(request.url)
        }
    )


if __name__ == "__main__":
    import uvicorn

    print("=" * 60)
    print("🚀 NeuroGraph OS MVP API Starting...")
    print("=" * 60)
    print("📖 Documentation: http://localhost:8000/docs")
    print("💚 Health check:  http://localhost:8000/health")
    print("🎯 API info:      http://localhost:8000/api")
    print("=" * 60)

    uvicorn.run(
        "main_mvp:app",
        host="0.0.0.0",
        port=8000,
        reload=True,
        log_level="info"
    )
