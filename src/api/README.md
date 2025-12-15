# NeuroGraph REST API

**Version:** 1.0.0 (v0.48.0)
**Status:** Phase 2.1 Complete
**Framework:** FastAPI

REST API service for NeuroGraph semantic knowledge system.

## Quick Start

### Installation

```bash
cd src/api
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

# Install NeuroGraph Python library
pip install -e ../python
```

### Configuration

```bash
# Copy environment template
cp .env.example .env

# Edit .env file with your settings
nano .env
```

### Run Development Server

```bash
# From src/api directory
uvicorn main:app --reload --host 0.0.0.0 --port 8000

# Or using the main.py script
python -m main
```

### Access Documentation

- **Swagger UI**: http://localhost:8000/docs
- **ReDoc**: http://localhost:8000/redoc
- **OpenAPI JSON**: http://localhost:8000/openapi.json

## API Endpoints

### Health & Status

- `GET /api/v1/health` - Health check
- `GET /api/v1/health/ready` - Readiness check
- `GET /api/v1/status` - Full system status
- `GET /api/v1/stats` - System statistics

### Query

- `POST /api/v1/query` - Semantic query

**Example:**
```bash
curl -X POST http://localhost:8000/api/v1/query \
  -H "Content-Type: application/json" \
  -d '{"text": "cat", "limit": 5}'
```

### Modules

- `GET /api/v1/modules` - List all modules
- `GET /api/v1/modules/{name}` - Get module details

### Metrics

- `GET /api/v1/metrics` - Prometheus metrics (text format)
- `GET /api/v1/metrics/json` - Metrics in JSON format

## Response Format

All endpoints return standardized responses:

### Success Response

```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "550e8400-e29b-41d4-a716-446655440000",
    "processing_time_ms": 14.2,
    "version": "1.0.0",
    "timestamp": "2025-01-14T12:34:56Z"
  }
}
```

### Error Response

```json
{
  "success": false,
  "data": null,
  "meta": { ... },
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Query text is required",
    "details": {
      "field": "text",
      "constraint": "non_empty"
    }
  }
}
```

## Project Structure

```
src/api/
├── main.py                 # FastAPI application
├── config.py               # Configuration settings
├── dependencies.py         # Dependency injection
├── requirements.txt        # Python dependencies
├── .env.example            # Environment variables template
├── models/                 # Pydantic models
│   ├── __init__.py
│   ├── response.py         # Standard response wrappers
│   ├── query.py            # Query models
│   └── status.py           # Status models
├── routers/                # API route handlers
│   ├── __init__.py
│   ├── health.py           # Health endpoints
│   ├── query.py            # Query endpoints
│   ├── status.py           # Status endpoints
│   ├── modules.py          # Module endpoints
│   └── metrics.py          # Metrics endpoints
└── tests/                  # API tests
    └── test_api.py
```

## Development

### Running Tests

```bash
pytest tests/ -v --cov=. --cov-report=term-missing
```

### Code Quality

```bash
# Format code
black .
ruff check .

# Type checking
mypy .
```

## Next Steps (Phase 2.2-2.5)

- [ ] **Phase 2.2**: Integrate neurograph Python library
- [ ] **Phase 2.3**: WebSocket support for real-time updates
- [ ] **Phase 2.4**: JWT authentication and rate limiting
- [ ] **Phase 2.5**: Docker deployment and CI/CD

## Documentation

- [REST API Specification](../../docs/arch/REST_API_SPEC.md)
- [Implementation Roadmap](../../docs/IMPLEMENTATION_ROADMAP.md)
- [CHANGELOG v0.48.0](../../docs/changelogs/CHANGELOG_v0.48.0.md) (will be created)

## License

AGPL-3.0-or-later (dual licensing available)

---

Built with ❤️ by NeuroGraph Team
