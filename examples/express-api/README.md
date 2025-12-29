# NeuroGraph Express.js API Example

Example Express.js REST API that integrates with NeuroGraph for semantic document management.

## Features

- REST API for document management (CRUD)
- Semantic search endpoint
- Batch document creation
- Retry logic with exponential backoff
- Comprehensive error handling
- TypeScript + Express.js
- CORS enabled

## API Endpoints

- `GET /health` - Health check
- `POST /documents` - Create document
- `GET /documents` - List documents (with pagination)
- `GET /documents/:id` - Get document by ID
- `PUT /documents/:id` - Update document
- `DELETE /documents/:id` - Delete document
- `POST /documents/search` - Semantic search
- `POST /documents/batch` - Batch create documents

## Setup

1. Install dependencies:
```bash
npm install
```

2. Create `.env`:
```bash
PORT=3001
NEUROGRAPH_API_URL=http://localhost:8000
NEUROGRAPH_API_KEY=your_api_key_here
# OR use username/password
NEUROGRAPH_USERNAME=developer
NEUROGRAPH_PASSWORD=developer123
```

3. Start development server:
```bash
npm run dev
```

4. API available at http://localhost:3001

## Usage Examples

### Create Document
```bash
curl -X POST http://localhost:3001/documents \
  -H "Content-Type: application/json" \
  -d '{"text": "Machine learning is awesome", "metadata": {"category": "tech"}}'
```

### Search Documents
```bash
curl -X POST http://localhost:3001/documents/search \
  -H "Content-Type: application/json" \
  -d '{"query": "artificial intelligence", "top_k": 5}'
```

### List Documents
```bash
curl http://localhost:3001/documents?limit=10&offset=0
```

### Batch Create
```bash
curl -X POST http://localhost:3001/documents/batch \
  -H "Content-Type: application/json" \
  -d '{
    "documents": [
      {"text": "Document 1", "metadata": {"type": "test"}},
      {"text": "Document 2", "metadata": {"type": "test"}}
    ]
  }'
```

## Production Build

```bash
npm run build
npm start
```

## Docker

```bash
docker build -t neurograph-express-api .
docker run -p 3001:3001 --env-file .env neurograph-express-api
```
