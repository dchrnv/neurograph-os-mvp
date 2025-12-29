# NeuroGraph v0.59.0 - Python Client Library

**Release Date:** 2025-12-28
**Type:** Major Feature Release
**Status:** Production Ready ✅

---

## 🎯 Overview

Official Python client library for NeuroGraph REST API. Provides both synchronous and asynchronous clients with full type hints, comprehensive error handling, and automatic authentication management.

**Package:** `neurograph-python`
**PyPI:** https://pypi.org/project/neurograph-python/ (coming soon)
**Python:** 3.10+

---

## 📦 What's New

### Python Client Library

**Features:**
- ✅ Synchronous client (`NeuroGraphClient`)
- ✅ Asynchronous client (`AsyncNeuroGraphClient`)
- ✅ Full type hints with Pydantic v2 models
- ✅ JWT and API Key authentication
- ✅ Automatic JWT token refresh
- ✅ Comprehensive error handling (8 exception types)
- ✅ Context manager support (`with`/`async with`)
- ✅ Configurable timeout and SSL verification
- ✅ Complete API coverage

**API Coverage:**
- **Tokens:** create, get, list, update, delete, query
- **API Keys:** create, list, get, revoke, delete
- **Health:** health check, system status

---

## 🚀 Installation

```bash
pip install neurograph-python
```

**Development:**
```bash
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os/python-client
pip install -e ".[dev]"
```

---

## 📖 Quick Start

### Synchronous Client

```python
from neurograph import NeuroGraphClient

# Initialize with JWT
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

# Create a token
token = client.tokens.create(text="hello world")
print(f"Token ID: {token.id}")
print(f"Embedding: {token.embedding[:5]}...")

# Query similar tokens
results = client.tokens.query(
    query_vector=token.embedding,
    top_k=10
)
for result in results:
    print(f"{result.token.text}: {result.similarity:.4f}")

client.close()
```

### Asynchronous Client

```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def main():
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        api_key="ng_your_api_key_here"
    ) as client:
        # Create tokens concurrently
        tasks = [
            client.tokens.create(text=f"token {i}")
            for i in range(10)
        ]
        tokens = await asyncio.gather(*tasks)
        print(f"Created {len(tokens)} tokens")

asyncio.run(main())
```

### Context Manager

```python
# Automatic cleanup with context manager
with NeuroGraphClient(...) as client:
    token = client.tokens.create(text="test")
    # Client automatically closed

# Async version
async with AsyncNeuroGraphClient(...) as client:
    token = await client.tokens.create(text="test")
    # Client automatically closed
```

---

## 🔐 Authentication

### JWT Authentication

```python
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)
# Automatic token refresh every 15 minutes
```

**Default users:**
- `admin` / `admin123` - Full access
- `developer` / `developer123` - Read + Write
- `viewer` / `viewer123` - Read only

### API Key Authentication

```python
# Create API key using JWT
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

api_key = client.api_keys.create(
    name="My Integration",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=30
)
print(f"⚠️  Save this key: {api_key.api_key}")

# Use API key for authentication
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    api_key=api_key.api_key
)
```

---

## 📚 API Reference

### Tokens API

```python
# Create token
token = client.tokens.create(
    text="hello world",
    metadata={"category": "greeting"}
)

# Get token by ID
token = client.tokens.get(token_id=123)

# List tokens with pagination
tokens = client.tokens.list(limit=100, offset=0)

# Update token
updated = client.tokens.update(
    token_id=123,
    text="new text",
    metadata={"updated": True}
)

# Delete token
client.tokens.delete(token_id=123)

# Query similar tokens
results = client.tokens.query(
    query_vector=[0.1, 0.2, ...],
    top_k=10,
    threshold=0.8  # Optional similarity threshold
)

# Access results
for result in results:
    print(f"Token {result.token.id}: {result.similarity:.4f}")
```

### API Keys API

```python
# Create API key (full key returned once!)
api_key = client.api_keys.create(
    name="Integration Key",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=30  # Optional expiration
)
print(f"Key: {api_key.api_key}")  # Save this!

# List all API keys
keys = client.api_keys.list()
for key in keys:
    print(f"{key.name}: {key.key_prefix}...")

# Get API key details
key = client.api_keys.get(key_id="key_123")

# Revoke API key (disable but keep in database)
client.api_keys.revoke(key_id="key_123")

# Delete API key (permanent deletion)
client.api_keys.delete(key_id="key_123")
```

### Health API

```python
# Check API health
health = client.health.check()
print(f"Status: {health.status}")
print(f"Version: {health.version}")

# Get system status with metrics
status = client.health.status()
print(f"API Version: {status.api_version}")
print(f"Tokens Count: {status.tokens_count}")
print(f"Uptime: {status.uptime_seconds}s")
print(f"Memory: {status.memory_usage_mb}MB")
```

---

## ⚠️ Error Handling

### Exception Hierarchy

```python
from neurograph import (
    NeuroGraphError,           # Base exception
    AuthenticationError,        # 401 - Auth failed
    AuthorizationError,         # 403 - Permission denied
    NotFoundError,             # 404 - Resource not found
    ValidationError,           # 422 - Validation failed
    RateLimitError,            # 429 - Rate limit exceeded
    ServerError,               # 500+ - Server error
    ConnectionError,           # Network connection failed
    TimeoutError,              # Request timeout
)
```

### Error Handling Examples

```python
try:
    token = client.tokens.get(token_id=999999)
except NotFoundError as e:
    print(f"Token not found: {e.message}")
    print(f"Error code: {e.error_code}")
    print(f"Details: {e.details}")

except AuthenticationError as e:
    print(f"Authentication failed: {e.message}")
    # Re-authenticate or check credentials

except AuthorizationError as e:
    print(f"Permission denied: {e.message}")
    # Check user permissions

except ValidationError as e:
    print(f"Validation error: {e.message}")
    print(f"Field errors: {e.details['errors']}")

except RateLimitError as e:
    print(f"Rate limit exceeded: {e.message}")
    if e.retry_after:
        print(f"Retry after {e.retry_after} seconds")
        time.sleep(e.retry_after)

except ServerError as e:
    print(f"Server error: {e.message}")
    # Retry or report error

except NeuroGraphError as e:
    # Catch all NeuroGraph errors
    print(f"Error [{e.error_code}]: {e.message}")
```

---

## 📐 Data Models

### Token

```python
class Token(BaseModel):
    id: int
    text: Optional[str]
    embedding: List[float]
    metadata: Optional[Dict[str, Any]]
    created_at: Optional[datetime]
```

### APIKey

```python
class APIKey(BaseModel):
    key_id: str
    name: str
    key_prefix: str
    scopes: List[str]
    created_at: datetime
    expires_at: Optional[datetime]
    last_used_at: Optional[datetime]
    is_active: bool
```

### User

```python
class User(BaseModel):
    user_id: str
    username: str
    email: Optional[str]
    full_name: Optional[str]
    role: str
    scopes: List[str]
    disabled: bool
    created_at: Optional[datetime]
```

---

## 🔧 Configuration

```python
client = NeuroGraphClient(
    base_url="http://localhost:8000",     # API base URL
    username="developer",                  # Username for JWT
    password="developer123",               # Password for JWT
    api_key=None,                         # Or use API key instead
    timeout=30.0,                         # Request timeout (seconds)
    verify_ssl=True,                      # Verify SSL certificates
)
```

**Environment Variables:**

```bash
export NEUROGRAPH_BASE_URL="http://localhost:8000"
export NEUROGRAPH_USERNAME="developer"
export NEUROGRAPH_PASSWORD="developer123"
# Or
export NEUROGRAPH_API_KEY="ng_1234567890abcdef"
```

---

## 📁 Package Structure

```
python-client/
├── neurograph/
│   ├── __init__.py           # Package entry point
│   ├── client.py             # Synchronous client (450+ lines)
│   ├── async_client.py       # Async client (350+ lines)
│   ├── auth.py               # Authentication manager
│   ├── models.py             # Pydantic data models (20+ models)
│   └── exceptions.py         # Exception hierarchy (8 exceptions)
├── examples/
│   ├── basic_usage.py        # Comprehensive sync examples
│   └── async_usage.py        # Async patterns
├── tests/
│   └── test_client.py        # Unit tests
├── pyproject.toml            # Modern Python packaging
├── setup.py                  # Setup script
├── README.md                 # Documentation
└── LICENSE                   # AGPLv3
```

---

## 📊 Statistics

**Code Metrics:**
- **Files:** 14 files
- **Lines of Code:** ~2,000+ lines
- **Models:** 20+ Pydantic models
- **Exceptions:** 8 custom exception types
- **API Methods:** 25+ client methods

**Test Coverage:**
- Unit tests for all major operations
- Integration tests (requires running API)
- Error handling tests

**Documentation:**
- Comprehensive README
- Inline docstrings for all public methods
- Usage examples for common patterns
- Type hints for IDE autocompletion

---

## 🔗 Dependencies

```toml
dependencies = [
    "httpx>=0.25.0",           # Modern HTTP client
    "pydantic>=2.0.0",         # Data validation
    "python-dateutil>=2.8.0",  # Date handling
]

[project.optional-dependencies]
dev = [
    "pytest>=7.4.0",
    "pytest-asyncio>=0.21.0",
    "pytest-cov>=4.1.0",
    "black>=23.0.0",
    "ruff>=0.1.0",
    "mypy>=1.5.0",
]
```

---

## 🧪 Testing

```bash
# Install dev dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run tests with coverage
pytest --cov=neurograph --cov-report=html

# Run specific test
pytest tests/test_client.py::test_create_token -v

# Run async tests
pytest tests/test_async_client.py
```

---

## 📝 Examples

### Example 1: Basic Token Management

```python
from neurograph import NeuroGraphClient

client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

# Create tokens
tokens = []
for text in ["hello", "world", "python", "neurograph"]:
    token = client.tokens.create(text=text)
    tokens.append(token)
    print(f"Created: {token.text}")

# Query similar to "hello"
results = client.tokens.query(
    query_vector=tokens[0].embedding,
    top_k=3
)

print("\nSimilar to 'hello':")
for result in results:
    print(f"  {result.token.text}: {result.similarity:.4f}")

# Cleanup
for token in tokens:
    client.tokens.delete(token.id)

client.close()
```

### Example 2: Async Batch Processing

```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def batch_create(client, texts):
    """Create tokens concurrently."""
    tasks = [client.tokens.create(text=text) for text in texts]
    return await asyncio.gather(*tasks)

async def main():
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        api_key="ng_your_key_here"
    ) as client:
        # Create 100 tokens concurrently
        texts = [f"token_{i}" for i in range(100)]
        tokens = await batch_create(client, texts)
        print(f"Created {len(tokens)} tokens")

        # Query in parallel
        query_tasks = [
            client.tokens.query(
                query_vector=token.embedding,
                top_k=5
            )
            for token in tokens[:10]
        ]
        results = await asyncio.gather(*query_tasks)
        print(f"Executed {len(results)} queries")

asyncio.run(main())
```

### Example 3: Error Recovery

```python
from neurograph import (
    NeuroGraphClient,
    RateLimitError,
    ServerError,
)
import time

client = NeuroGraphClient(...)

def create_token_with_retry(text, max_retries=3):
    """Create token with automatic retry on rate limit."""
    for attempt in range(max_retries):
        try:
            return client.tokens.create(text=text)
        except RateLimitError as e:
            if attempt == max_retries - 1:
                raise
            wait_time = e.retry_after or (2 ** attempt)
            print(f"Rate limited. Retry in {wait_time}s...")
            time.sleep(wait_time)
        except ServerError as e:
            if attempt == max_retries - 1:
                raise
            print(f"Server error. Retrying...")
            time.sleep(2 ** attempt)

# Use it
token = create_token_with_retry("resilient token")
```

---

## 🔄 API Compatibility

**Compatible with NeuroGraph API:**
- v0.58.0+ (JWT auth, API keys, RBAC)
- v0.58.1+ (Enhanced error handling, security)

**Breaking changes:** None (v0.59.0 is backward compatible)

---

## 🚀 Roadmap

### v0.59.1 (Planned)
- [ ] Grid API support
- [ ] CDNA API support
- [ ] Streaming responses
- [ ] Batch operations
- [ ] Connection pooling

### v0.59.2 (Planned)
- [ ] Retry strategies (exponential backoff)
- [ ] Circuit breaker pattern
- [ ] Request/response middleware
- [ ] Metrics collection

### v0.60.0 (Planned)
- [ ] WebSocket support
- [ ] Real-time event streaming
- [ ] Server-Sent Events (SSE)

---

## 📞 Support

**Issues:** https://github.com/dchrnv/neurograph-os/issues
**Discussions:** https://github.com/dchrnv/neurograph-os/discussions
**Documentation:** https://neurograph.dev/docs
**License:** AGPLv3

---

## 📖 Related Documentation

- [NeuroGraph API Documentation](../README.md)
- [REST API Reference](../docs/API.md)
- [Authentication Guide](../docs/AUTH.md)
- [CHANGELOG v0.58.0](CHANGELOG_v0.58.0.md)
- [Master Plan](docs/MASTER_PLAN_v3.0.md)

---

**Release Tag:** `v0.59.0`
**Commit:** `ccf9f51`
**Date:** 2025-12-28

🎉 **Happy coding with NeuroGraph Python Client!**
