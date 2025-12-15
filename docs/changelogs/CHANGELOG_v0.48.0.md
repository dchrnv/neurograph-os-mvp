# Changelog - v0.48.0 (REST API)

All notable changes for v0.48.x releases (Phase 2: REST API).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased][Unreleased]

### Added - REST API (Phase 2)

#### [0.48.1] - 2025-01-14

**Phase 2.1: FastAPI Project Setup**

Added:

- `src/api/` - Complete FastAPI project structure
  - `main.py` - FastAPI application with middleware and exception handling
  - `config.py` - Pydantic settings with environment variable support
  - `dependencies.py` - Dependency injection for runtime, auth, etc.
  - `requirements.txt` - Python dependencies (FastAPI, uvicorn, pydantic, etc.)
  - `.env.example` - Environment variables template
  - `README.md` - API documentation with quick start guide

- `src/api/models/` - Pydantic models for requests and responses
  - `response.py` - Standard API response wrappers (ApiResponse, ErrorResponse, MetaData)
  - `query.py` - Query models (QueryRequest, QueryResponse, TokenResult)
  - `status.py` - Status models (StatusResponse, HealthResponse, ComponentStatus)

- `src/api/routers/` - API route handlers
  - `health.py` - Health check endpoints (`/health`, `/health/ready`)
  - `query.py` - Query endpoint (`POST /query`) with mock data
  - `status.py` - Status endpoints (`/status`, `/stats`)
  - `modules.py` - Module management endpoints (`/modules`, `/modules/{name}`)
  - `metrics.py` - Prometheus metrics (`/metrics`, `/metrics/json`)

Features:

- **Standard Response Format**: All endpoints use consistent ApiResponse wrapper
- **Error Handling**: Global exception handler with structured error responses
- **CORS Support**: Configurable CORS middleware for web dashboard
- **Request Timing**: Middleware tracks processing time for all requests
- **OpenAPI Documentation**: Auto-generated Swagger UI at `/docs` and ReDoc at `/redoc`
- **Environment Configuration**: Pydantic settings with .env file support
- **Logging**: Structured logging with timestamps and severity levels

Endpoints Implemented (Phase 2.1):

```
GET  /api/v1/health          - Health check
GET  /api/v1/health/ready    - Readiness check
POST /api/v1/query           - Semantic query (mock data)
GET  /api/v1/status          - System status
GET  /api/v1/stats           - System statistics
GET  /api/v1/modules         - List modules
GET  /api/v1/modules/{name}  - Get module details
GET  /api/v1/metrics         - Prometheus metrics (text format)
GET  /api/v1/metrics/json    - Metrics in JSON format
```

Technical Details:

- **Framework**: FastAPI 0.109.0
- **Server**: Uvicorn with standard extras (uvloop, httptools)
- **Validation**: Pydantic 2.5.3 for request/response validation
- **Settings**: Pydantic-settings for environment management
- **Response Model**: Standardized success/error format with metadata

File Structure:

```
src/api/
├── main.py                 # FastAPI app (middleware, exception handlers)
├── config.py               # Settings (Pydantic BaseSettings)
├── dependencies.py         # DI (runtime, auth)
├── requirements.txt        # Dependencies
├── .env.example            # Environment template
├── README.md               # Documentation
├── models/
│   ├── response.py         # ApiResponse, ErrorResponse, MetaData
│   ├── query.py            # QueryRequest, QueryResponse, TokenResult
│   └── status.py           # StatusResponse, HealthResponse
└── routers/
    ├── health.py           # Health endpoints
    ├── query.py            # Query endpoint
    ├── status.py           # Status/stats endpoints
    ├── modules.py          # Module management
    └── metrics.py          # Prometheus metrics
```

## Version History

- **0.48.1** - FastAPI Project Setup (Phase 2.1)
- **0.47.x** - Python Library (Phase 1: Complete) ✅

[Unreleased]: https://github.com/dchrnv/neurograph-os/compare/v0.48.1...HEAD
[0.48.1]: https://github.com/dchrnv/neurograph-os/compare/v0.47.5...v0.48.1
