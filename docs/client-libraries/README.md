# NeuroGraph Client Libraries

Official client libraries for NeuroGraph API in multiple programming languages.

## Available Libraries

### Python Client

**Package:** `neurograph-python`
**Version:** 0.59.1
**Python:** 3.10+
**Status:** ✅ Production Ready

[📖 Python Client Documentation](./python-client.md) | [GitHub](../../python-client/)

```bash
pip install neurograph-python
```

**Features:**
- ✅ Synchronous and Asynchronous clients
- ✅ Full type hints with Pydantic
- ✅ JWT and API Key authentication
- ✅ Retry mechanism with exponential backoff
- ✅ CLI utility (`neurograph-cli`)
- ✅ Comprehensive error handling
- ✅ Logging support

### TypeScript/JavaScript Client

**Package:** `@neurograph/client`
**Version:** 0.59.2
**Node.js:** 18+
**Status:** ✅ Production Ready

[📖 TypeScript Client Documentation](./typescript-client.md) | [GitHub](../../typescript-client/)

```bash
npm install @neurograph/client
```

**Features:**
- ✅ Full TypeScript support
- ✅ Works in Node.js, Browser, Deno, Bun
- ✅ JWT and API Key authentication
- ✅ Retry mechanism with exponential backoff
- ✅ Promise-based async/await API
- ✅ ESM and CJS builds
- ✅ Comprehensive error handling

---

## Quick Comparison

| Feature | Python | TypeScript |
|---------|--------|------------|
| Sync API | ✅ | ✅ (Promise-based) |
| Async API | ✅ | ✅ (Native) |
| Type Safety | ✅ Pydantic | ✅ TypeScript |
| Authentication | JWT + API Key | JWT + API Key |
| Retry Logic | ✅ | ✅ |
| Error Handling | 8 exception types | 8 exception types |
| CLI Tool | ✅ `neurograph-cli` | ❌ |
| Browser Support | ❌ | ✅ |
| Node.js | ✅ | ✅ |

---

## Quick Start Examples

### Python (Sync)

```python
from neurograph import NeuroGraphClient

client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

# Create token
token = client.tokens.create(text="hello world")

# Query similar
results = client.tokens.query(
    query_vector=token.embedding,
    top_k=10
)

client.close()
```

### Python (Async)

```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def main():
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        api_key="ng_your_key"
    ) as client:
        token = await client.tokens.create(text="hello world")
        results = await client.tokens.query(
            query_vector=token.embedding,
            top_k=10
        )

asyncio.run(main())
```

### TypeScript/JavaScript

```typescript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

// Create token
const token = await client.tokens.create({ text: 'hello world' });

// Query similar
const results = await client.tokens.query({
  queryVector: token.embedding,
  topK: 10
});
```

---

## Authentication

Both libraries support two authentication methods:

### 1. JWT Authentication (Username/Password)

**Python:**
```python
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)
```

**TypeScript:**
```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});
```

**Default Users:**
- `admin` / `admin123` - Full access
- `developer` / `developer123` - Read + Write
- `viewer` / `viewer123` - Read only

### 2. API Key Authentication

**Step 1:** Create API key using JWT auth

**Python:**
```python
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
print(f"Save this: {api_key.api_key}")
```

**TypeScript:**
```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

const apiKey = await client.apiKeys.create({
  name: 'My Integration',
  scopes: ['tokens:read', 'tokens:write'],
  expires_in_days: 30
});
console.log(`Save this: ${apiKey.api_key}`);
```

**Step 2:** Use API key

**Python:**
```python
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    api_key="ng_your_api_key_here"
)
```

**TypeScript:**
```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  apiKey: 'ng_your_api_key_here'
});
```

---

## Common Operations

### Create Token

**Python:**
```python
token = client.tokens.create(
    text="hello world",
    metadata={"category": "greeting"}
)
```

**TypeScript:**
```typescript
const token = await client.tokens.create({
  text: 'hello world',
  metadata: { category: 'greeting' }
});
```

### Query Similar Tokens

**Python:**
```python
results = client.tokens.query(
    query_vector=token.embedding,
    top_k=10,
    threshold=0.8
)

for result in results:
    print(f"{result.token.text}: {result.similarity:.4f}")
```

**TypeScript:**
```typescript
const results = await client.tokens.query({
  queryVector: token.embedding,
  topK: 10,
  threshold: 0.8
});

for (const result of results) {
  console.log(`${result.token.text}: ${result.similarity.toFixed(4)}`);
}
```

### List Tokens

**Python:**
```python
tokens = client.tokens.list(limit=100, offset=0)
```

**TypeScript:**
```typescript
const tokens = await client.tokens.list({ limit: 100, offset: 0 });
```

### Update Token

**Python:**
```python
updated = client.tokens.update(
    token_id=123,
    metadata={"updated": True}
)
```

**TypeScript:**
```typescript
const updated = await client.tokens.update(123, {
  metadata: { updated: true }
});
```

### Delete Token

**Python:**
```python
client.tokens.delete(token_id=123)
```

**TypeScript:**
```typescript
await client.tokens.delete(123);
```

---

## Error Handling

Both libraries provide the same error types:

| Error Type | Status | Description |
|------------|--------|-------------|
| `NeuroGraphError` | - | Base error class |
| `AuthenticationError` | 401 | Invalid credentials |
| `AuthorizationError` | 403 | Insufficient permissions |
| `NotFoundError` | 404 | Resource not found |
| `ValidationError` | 422 | Invalid input |
| `RateLimitError` | 429 | Rate limit exceeded |
| `ConflictError` | 409 | Resource conflict |
| `ServerError` | 500+ | Server error |
| `NetworkError` | - | Connection issues |

**Python:**
```python
from neurograph import (
    NotFoundError,
    RateLimitError,
    ValidationError
)

try:
    token = client.tokens.get(token_id=999999)
except NotFoundError:
    print("Token not found")
except RateLimitError as e:
    print(f"Rate limited. Retry after {e.retry_after}s")
except ValidationError as e:
    print(f"Validation error: {e.details}")
```

**TypeScript:**
```typescript
import {
  NotFoundError,
  RateLimitError,
  ValidationError
} from '@neurograph/client';

try {
  const token = await client.tokens.get(999999);
} catch (error) {
  if (error instanceof NotFoundError) {
    console.log('Token not found');
  } else if (error instanceof RateLimitError) {
    console.log(`Rate limited. Retry after ${error.retryAfter}s`);
  } else if (error instanceof ValidationError) {
    console.log(`Validation error: ${error.details}`);
  }
}
```

---

## Retry Mechanism

Both libraries include retry logic with exponential backoff.

**Python:**
```python
from neurograph import retry_with_backoff, RetryConfig

@retry_with_backoff(config=RetryConfig(
    max_retries=5,
    initial_delay=1.0,
    max_delay=60.0,
    exponential_base=2.0,
    jitter=True
))
def create_token_resilient(text):
    return client.tokens.create(text=text)

token = create_token_resilient("resilient token")
```

**TypeScript:**
```typescript
import { retryWithBackoff } from '@neurograph/client';

const token = await retryWithBackoff(
  () => client.tokens.create({ text: 'resilient token' }),
  {
    maxRetries: 5,
    initialDelay: 1000,
    maxDelay: 60000,
    exponentialBase: 2,
    jitter: true
  }
);
```

---

## Logging

### Python Logging

```python
from neurograph import setup_logging, enable_debug_logging
import logging

# Custom logging setup
setup_logging(level=logging.DEBUG)

# Or simple debug mode
enable_debug_logging()

# Use logger
from neurograph import get_logger
logger = get_logger()
logger.info("Creating token...")
```

### TypeScript Logging

TypeScript client uses standard console logging. Enable debug mode:

```typescript
// Set environment variable
process.env.DEBUG = 'neurograph:*';

// Or use custom logger with axios interceptors
const client = new NeuroGraphClient({ ... });
const httpClient = client.getHttpClient();

httpClient.interceptors.request.use((config) => {
  console.log('Request:', config.method, config.url);
  return config;
});
```

---

## CLI Usage (Python Only)

The Python client includes a CLI tool:

```bash
# Health check
neurograph-cli health

# Create token
neurograph-cli token create "hello world" --username developer --password developer123

# List tokens
neurograph-cli token list --limit 10

# Query tokens
neurograph-cli token query --text "hello" --top-k 5

# Create API key
neurograph-cli apikey create --name "My Key" --expires 30

# Enable debug logging
neurograph-cli --debug token list

# Use API key for auth
neurograph-cli --api-key "ng_your_key" token list
```

---

## Best Practices

### 1. Use Context Managers (Python)

```python
# Sync
with NeuroGraphClient(...) as client:
    token = client.tokens.create(text="hello")
    # Auto-closed on exit

# Async
async with AsyncNeuroGraphClient(...) as client:
    token = await client.tokens.create(text="hello")
    # Auto-closed on exit
```

### 2. Use API Keys for Production

- Create API keys with minimum required scopes
- Set expiration dates
- Rotate keys regularly
- Never commit keys to git

### 3. Handle Rate Limits

```python
from neurograph import RateLimitError
import time

try:
    token = client.tokens.create(text="test")
except RateLimitError as e:
    time.sleep(e.retry_after)
    token = client.tokens.create(text="test")
```

### 4. Use Retry for Resilience

```python
# Python
from neurograph import retry_with_backoff

@retry_with_backoff
def create_token(text):
    return client.tokens.create(text=text)
```

```typescript
// TypeScript
import { retryWithBackoff } from '@neurograph/client';

const token = await retryWithBackoff(() =>
  client.tokens.create({ text: 'test' })
);
```

### 5. Batch Operations Efficiently

**Python:**
```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def batch_create(texts):
    async with AsyncNeuroGraphClient(...) as client:
        tasks = [client.tokens.create(text=t) for t in texts]
        return await asyncio.gather(*tasks)

tokens = asyncio.run(batch_create(["text1", "text2", "text3"]))
```

**TypeScript:**
```typescript
const texts = ['text1', 'text2', 'text3'];
const promises = texts.map(text =>
  client.tokens.create({ text })
);
const tokens = await Promise.all(promises);
```

---

## Troubleshooting

### Connection Errors

**Problem:** Cannot connect to API

**Solution:**
- Check `base_url` / `baseUrl` is correct
- Verify API server is running
- Check firewall/network settings

### Authentication Errors

**Problem:** 401 Unauthorized

**Solution:**
- Verify credentials are correct
- Check API key is active and not expired
- Ensure API key has required scopes

### Rate Limit Errors

**Problem:** 429 Too Many Requests

**Solution:**
- Implement retry with backoff
- Reduce request rate
- Use batch operations
- Respect `retry_after` header

### SSL/TLS Errors

**Python:**
```python
client = NeuroGraphClient(
    base_url="https://api.neurograph.dev",
    verify_ssl=False  # Only for development!
)
```

**TypeScript:**
```typescript
const client = new NeuroGraphClient({
  baseUrl: 'https://api.neurograph.dev',
  // For Node.js with custom certs:
  // Use axios config via getHttpClient()
});
```

---

## Migration Guide

### From v0.58 to v0.59

**Changes:**
- New client libraries available
- API remains backwards compatible
- JWT token expiration now handled automatically

**Migration:**

Before (direct API calls):
```python
import requests

response = requests.post(
    "http://localhost:8000/tokens",
    json={"text": "hello"},
    headers={"Authorization": f"Bearer {token}"}
)
```

After (using client):
```python
from neurograph import NeuroGraphClient

client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)
token = client.tokens.create(text="hello")
```

---

## Support

- **Documentation**: https://neurograph.dev/docs
- **GitHub Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Discussions**: https://github.com/dchrnv/neurograph-os/discussions

---

## License

Both client libraries are licensed under AGPLv3 - See [LICENSE](../../LICENSE) for details.
