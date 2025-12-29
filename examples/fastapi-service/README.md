# NeuroGraph FastAPI Service Example

Example FastAPI service demonstrating async NeuroGraph Python client integration.

## Features

- Async REST API for document management
- Semantic search endpoint
- Batch document creation
- Automatic retry with decorators
- Comprehensive error handling
- Python + FastAPI + NeuroGraph async client
- CORS enabled
- OpenAPI/Swagger docs

## API Endpoints

- `GET /health` - Health check
- `POST /documents` - Create document
- `GET /documents` - List documents (with pagination)
- `GET /documents/{id}` - Get document by ID
- `PUT /documents/{id}` - Update document
- `DELETE /documents/{id}` - Delete document
- `POST /documents/search` - Semantic search
- `POST /documents/batch` - Batch create documents

## Setup

1. Install dependencies:
```bash
pip install -r requirements.txt
```

2. Create `.env`:
```bash
NEUROGRAPH_API_URL=http://localhost:8000
NEUROGRAPH_API_KEY=your_api_key_here
# OR use username/password
NEUROGRAPH_USERNAME=developer
NEUROGRAPH_PASSWORD=developer123
```

3. Start development server:
```bash
uvicorn main:app --reload --port 8001
```

4. API available at http://localhost:8001
5. Swagger docs at http://localhost:8001/docs

## Usage Examples

### Create Document
```bash
curl -X POST http://localhost:8001/documents \
  -H "Content-Type: application/json" \
  -d '{"text": "Machine learning is awesome", "metadata": {"category": "tech"}}'
```

### Search Documents
```bash
curl -X POST http://localhost:8001/documents/search \
  -H "Content-Type: application/json" \
  -d '{"query": "artificial intelligence", "top_k": 5, "threshold": 0.7}'
```

### List Documents
```bash
curl http://localhost:8001/documents?limit=10&offset=0
```

### Batch Create
```bash
curl -X POST http://localhost:8001/documents/batch \
  -H "Content-Type: application/json" \
  -d '{
    "documents": [
      {"text": "Document 1", "metadata": {"type": "test"}},
      {"text": "Document 2", "metadata": {"type": "test"}}
    ]
  }'
```

## Production Deployment

```bash
# With gunicorn
pip install gunicorn
gunicorn main:app -w 4 -k uvicorn.workers.UvicornWorker --bind 0.0.0.0:8001

# With uvicorn
uvicorn main:app --host 0.0.0.0 --port 8001 --workers 4
```

## Docker

```bash
docker build -t neurograph-fastapi-service .
docker run -p 8001:8001 --env-file .env neurograph-fastapi-service
```

## Features Demonstrated

- Async/await with AsyncNeuroGraphClient
- Context manager for resource cleanup
- Retry decorator for resilience
- Pydantic models for validation
- OpenAPI documentation
- Error handling
- CORS middleware
- Health checks
