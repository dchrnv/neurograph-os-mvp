# Python Client Library - Complete Guide

Official Python client for NeuroGraph API with sync and async support.

**Package:** `neurograph-python`
**Version:** 0.59.1
**Python:** 3.10+
**License:** AGPLv3

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [API Reference](#api-reference)
- [Error Handling](#error-handling)
- [Retry Mechanism](#retry-mechanism)
- [Logging](#logging)
- [CLI Tool](#cli-tool)
- [Advanced Usage](#advanced-usage)
- [Examples](#examples)

---

## Installation

### Via pip (when published)

```bash
pip install neurograph-python
```

### From source

```bash
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os/python-client
pip install -e .
```

### With development dependencies

```bash
pip install -e ".[dev]"
```

---

## Quick Start

### Synchronous Client

```python
from neurograph import NeuroGraphClient

# Create client
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

# Cleanup
client.tokens.delete(token.id)
client.close()
```

### Asynchronous Client

```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def main():
    # Use context manager for auto-cleanup
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123"
    ) as client:
        # Create token
        token = await client.tokens.create(text="hello async")

        # Query
        results = await client.tokens.query(
            query_vector=token.embedding,
            top_k=10
        )

        # Process results
        for result in results:
            print(f"{result.token.text}: {result.similarity:.4f}")

        # Cleanup
        await client.tokens.delete(token.id)

# Run async code
asyncio.run(main())
```

### Using Context Manager

```python
# Sync
with NeuroGraphClient(...) as client:
    token = client.tokens.create(text="hello")
    # Client automatically closed on exit

# Async
async with AsyncNeuroGraphClient(...) as client:
    token = await client.tokens.create(text="hello")
    # Client automatically closed on exit
```

---

## Authentication

### JWT Authentication

```python
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)
```

**Default users:**
- `admin` / `admin123` - Full access (admin role)
- `developer` / `developer123` - Read + Write (developer role)
- `viewer` / `viewer123` - Read only (viewer role)

**Features:**
- Automatic JWT token refresh
- Token caching for performance
- Automatic retry on 401 errors

### API Key Authentication

```python
# Step 1: Create API key with JWT
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

api_key = client.api_keys.create(
    name="Production Key",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=90
)

# IMPORTANT: Save this key - it's only shown once!
print(f"API Key: {api_key.api_key}")

# Step 2: Use API key
api_client = NeuroGraphClient(
    base_url="http://localhost:8000",
    api_key=api_key.api_key
)

# Now use api_client for all operations
token = api_client.tokens.create(text="hello")
```

**Available Scopes:**
- `tokens:read` - Read tokens
- `tokens:write` - Create, update, delete tokens
- `api_keys:read` - List and view API keys
- `api_keys:write` - Create, revoke, delete API keys

---

## API Reference

### Tokens

#### Create Token

```python
token = client.tokens.create(
    text="hello world",
    metadata={"category": "greeting", "language": "en"}
)

# Returns: Token object with id, text, embedding, metadata, timestamps
print(token.id)           # int
print(token.text)         # str
print(token.embedding)    # List[float]
print(token.metadata)     # dict
print(token.created_at)   # datetime
print(token.updated_at)   # datetime
```

#### Get Token

```python
token = client.tokens.get(token_id=123)

# Raises NotFoundError if token doesn't exist
```

#### List Tokens

```python
# List with pagination
tokens = client.tokens.list(limit=100, offset=0)

# Get all tokens (use with caution)
all_tokens = client.tokens.list(limit=10000)
```

#### Update Token

```python
# Update text and/or metadata
updated = client.tokens.update(
    token_id=123,
    text="updated text",           # Optional
    metadata={"updated": True}      # Optional
)

# Update only metadata
updated = client.tokens.update(
    token_id=123,
    metadata={"status": "processed"}
)
```

#### Delete Token

```python
# Delete returns True on success
success = client.tokens.delete(token_id=123)

# Raises NotFoundError if token doesn't exist
```

#### Query Similar Tokens

```python
# Query by embedding vector
results = client.tokens.query(
    query_vector=[0.1, 0.2, 0.3, ...],  # List of floats
    top_k=10,                             # Optional, default=10
    threshold=0.8                         # Optional, minimum similarity
)

# Each result has:
for result in results:
    print(result.token)       # Token object
    print(result.similarity)  # float (0.0 to 1.0)

# Common pattern: query using existing token
token = client.tokens.get(token_id=123)
similar = client.tokens.query(
    query_vector=token.embedding,
    top_k=5
)
```

### API Keys

#### Create API Key

```python
api_key = client.api_keys.create(
    name="My Application",
    scopes=["tokens:read", "tokens:write"],
    expires_in_days=30  # Optional, default=None (never expires)
)

# SAVE THIS - only shown once!
print(api_key.api_key)  # Full key: ng_xxx...
print(api_key.key_id)   # Key ID for management
```

#### List API Keys

```python
keys = client.api_keys.list()

for key in keys:
    print(f"{key.name}: {key.key_id}")
    print(f"  Scopes: {key.scopes}")
    print(f"  Active: {key.is_active}")
    print(f"  Expires: {key.expires_at}")
```

#### Get API Key

```python
key = client.api_keys.get(key_id="key_abc123")

# Returns metadata only, NOT the actual key
print(key.name)
print(key.scopes)
print(key.is_active)
```

#### Revoke API Key

```python
# Makes key inactive but keeps in database
client.api_keys.revoke(key_id="key_abc123")

# Can still view key details but can't use for auth
```

#### Delete API Key

```python
# Permanently deletes key
client.api_keys.delete(key_id="key_abc123")

# Raises NotFoundError if key doesn't exist
```

### Health

#### Health Check

```python
health = client.health.check()

print(health.status)     # "healthy" or "unhealthy"
print(health.version)    # API version
print(health.timestamp)  # Current server time
```

#### System Status

```python
status = client.health.status()

print(status.status)          # "healthy" or "unhealthy"
print(status.version)         # API version
print(status.uptime_seconds)  # Server uptime
print(status.tokens_count)    # Total tokens in system
print(status.api_keys_count)  # Total API keys
```

---

## Error Handling

### Exception Hierarchy

```
NeuroGraphError (base)
├── AuthenticationError (401)
├── AuthorizationError (403)
├── NotFoundError (404)
├── ValidationError (422)
├── RateLimitError (429)
├── ConflictError (409)
└── ServerError (500+)
```

### Basic Error Handling

```python
from neurograph import (
    NeuroGraphError,
    NotFoundError,
    AuthenticationError,
    ValidationError,
    RateLimitError
)

try:
    token = client.tokens.get(token_id=999999)
except NotFoundError as e:
    print(f"Token not found: {e.message}")
    print(f"Error code: {e.error_code}")
except AuthenticationError as e:
    print(f"Auth failed: {e.message}")
except NeuroGraphError as e:
    print(f"API error: {e.message}")
    print(f"Details: {e.details}")
```

### Graceful Degradation

```python
# Pattern: Fallback to default if not found
try:
    token = client.tokens.get(token_id=123)
except NotFoundError:
    # Create if doesn't exist
    token = client.tokens.create(text="default token")
```

### Rate Limit Handling

```python
import time

try:
    token = client.tokens.create(text="test")
except RateLimitError as e:
    # Wait for the specified time
    print(f"Rate limited. Waiting {e.retry_after}s...")
    time.sleep(e.retry_after)

    # Retry
    token = client.tokens.create(text="test")
```

### Validation Errors

```python
try:
    token = client.tokens.create(text="")  # Empty text
except ValidationError as e:
    print(f"Validation failed: {e.message}")
    print(f"Details: {e.details}")
    # Details contain field-specific errors
```

---

## Retry Mechanism

### Using Decorator

```python
from neurograph import retry_with_backoff, RetryConfig

# Simple retry with defaults
@retry_with_backoff
def create_token_resilient(text):
    return client.tokens.create(text=text)

token = create_token_resilient("resilient token")

# Custom retry configuration
@retry_with_backoff(config=RetryConfig(
    max_retries=5,
    initial_delay=0.5,      # 500ms
    max_delay=30.0,         # 30 seconds max
    exponential_base=2.0,   # Double each time
    jitter=True             # Add randomness
))
def robust_create(text):
    return client.tokens.create(text=text)

token = robust_create("very resilient")
```

### Async Retry

```python
from neurograph import async_retry_with_backoff

@async_retry_with_backoff(config=RetryConfig(max_retries=5))
async def create_async(text):
    return await client.tokens.create(text=text)

token = await create_async("async resilient")
```

### Manual Retry Logic

```python
from neurograph import RetryConfig, RateLimitError, ServerError
import time

config = RetryConfig(max_retries=3)

for attempt in range(config.max_retries + 1):
    try:
        token = client.tokens.create(text="manual retry")
        break  # Success
    except (RateLimitError, ServerError) as e:
        if attempt == config.max_retries:
            raise  # Give up after max retries

        # Calculate delay
        if isinstance(e, RateLimitError) and e.retry_after:
            delay = e.retry_after
        else:
            delay = config.get_delay(attempt)

        print(f"Attempt {attempt + 1} failed. Retrying in {delay}s...")
        time.sleep(delay)
```

### What Gets Retried

**Automatically retried:**
- `RateLimitError` (429)
- `ServerError` (500, 502, 503, 504)
- Connection errors

**NOT retried:**
- `AuthenticationError` (401)
- `AuthorizationError` (403)
- `NotFoundError` (404)
- `ValidationError` (422)
- `ConflictError` (409)

---

## Logging

### Setup Logging

```python
from neurograph import setup_logging
import logging

# Basic setup
setup_logging(level=logging.INFO)

# Debug mode
setup_logging(level=logging.DEBUG)

# Custom format
setup_logging(
    level=logging.DEBUG,
    format_string="%(asctime)s - %(name)s - %(levelname)s - %(message)s"
)
```

### Quick Debug Mode

```python
from neurograph import enable_debug_logging

# Enable debug logging with default format
enable_debug_logging()

# Now all operations are logged
client = NeuroGraphClient(...)
token = client.tokens.create(text="logged operation")
```

### Using Logger

```python
from neurograph import get_logger

logger = get_logger()

logger.info("Starting token creation...")
token = client.tokens.create(text="test")
logger.info(f"Created token: {token.id}")

try:
    result = client.tokens.get(999999)
except Exception as e:
    logger.error(f"Error: {e}", exc_info=True)
```

### Disable Logging

```python
from neurograph import disable_logging

disable_logging()
# All logging suppressed
```

---

## CLI Tool

The Python client includes `neurograph-cli` command-line tool.

### Installation

```bash
pip install neurograph-python
# neurograph-cli is now available
```

### Health Commands

```bash
# Check health
neurograph-cli health

# Get system status
neurograph-cli status
```

### Token Commands

```bash
# Create token
neurograph-cli token create "hello world" \
  --username developer \
  --password developer123

# Create with metadata
neurograph-cli token create "test" \
  --metadata '{"key": "value"}' \
  --username developer \
  --password developer123

# Get token
neurograph-cli token get 123 \
  --username developer \
  --password developer123

# List tokens
neurograph-cli token list --limit 10 --offset 0

# Update token
neurograph-cli token update 123 \
  --text "new text" \
  --metadata '{"updated": true}'

# Delete token
neurograph-cli token delete 123

# Query tokens
neurograph-cli token query \
  --text "hello" \
  --top-k 5 \
  --threshold 0.8
```

### API Key Commands

```bash
# Create API key
neurograph-cli apikey create \
  --name "My Key" \
  --scopes tokens:read tokens:write \
  --expires 30 \
  --username developer \
  --password developer123

# List API keys
neurograph-cli apikey list

# Revoke API key
neurograph-cli apikey revoke key_abc123

# Delete API key
neurograph-cli apikey delete key_abc123
```

### Global Options

```bash
# Custom base URL
neurograph-cli --base-url https://api.neurograph.dev token list

# Use API key instead of username/password
neurograph-cli --api-key ng_your_key token list

# Enable debug logging
neurograph-cli --debug token create "test"

# JSON output
neurograph-cli --json token list
```

### Environment Variables

```bash
# Set defaults via environment
export NEUROGRAPH_BASE_URL=http://localhost:8000
export NEUROGRAPH_USERNAME=developer
export NEUROGRAPH_PASSWORD=developer123

# Now no need to specify them
neurograph-cli token list
```

---

## Advanced Usage

### Concurrent Operations (Async)

```python
import asyncio
from neurograph import AsyncNeuroGraphClient

async def batch_create(texts):
    async with AsyncNeuroGraphClient(...) as client:
        # Create all tokens concurrently
        tasks = [
            client.tokens.create(text=text)
            for text in texts
        ]
        tokens = await asyncio.gather(*tasks)
        return tokens

texts = ["token1", "token2", "token3", "token4", "token5"]
tokens = asyncio.run(batch_create(texts))
print(f"Created {len(tokens)} tokens")
```

### Connection Pooling

```python
# Sync client uses connection pooling by default
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123",
    timeout=30.0  # Request timeout
)

# Async client also uses pooling
async_client = AsyncNeuroGraphClient(...)
```

### Custom Timeouts

```python
# Per-client timeout
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123",
    timeout=60.0  # 60 seconds
)
```

### SSL/TLS Configuration

```python
# Disable SSL verification (development only!)
client = NeuroGraphClient(
    base_url="https://localhost:8443",
    username="developer",
    password="developer123",
    verify_ssl=False
)
```

### Streaming Large Results

```python
# For large datasets, use pagination
offset = 0
limit = 100

while True:
    batch = client.tokens.list(limit=limit, offset=offset)
    if not batch:
        break

    # Process batch
    for token in batch:
        print(token.text)

    offset += limit
```

---

## Examples

### Example 1: Semantic Search

```python
from neurograph import NeuroGraphClient

client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123"
)

# Index documents
documents = [
    "Python is a programming language",
    "JavaScript is used for web development",
    "Machine learning uses neural networks",
    "Deep learning is a subset of ML"
]

tokens = []
for doc in documents:
    token = client.tokens.create(
        text=doc,
        metadata={"type": "document"}
    )
    tokens.append(token)

# Search
query = client.tokens.create(text="programming languages")
results = client.tokens.query(
    query_vector=query.embedding,
    top_k=3
)

print("Search results:")
for result in results:
    print(f"  {result.token.text} ({result.similarity:.4f})")

# Cleanup
client.tokens.delete(query.id)
for token in tokens:
    client.tokens.delete(token.id)

client.close()
```

### Example 2: Batch Processing

```python
import asyncio
from neurograph import AsyncNeuroGraphClient, async_retry_with_backoff

async def process_batch(texts):
    async with AsyncNeuroGraphClient(...) as client:
        # Create with retry
        @async_retry_with_backoff
        async def create_with_retry(text):
            return await client.tokens.create(text=text)

        # Process concurrently
        tasks = [create_with_retry(text) for text in texts]
        tokens = await asyncio.gather(*tasks)

        print(f"Created {len(tokens)} tokens")
        return tokens

# Process large batch
texts = [f"document {i}" for i in range(100)]
tokens = asyncio.run(process_batch(texts))
```

### Example 3: Error Recovery

```python
from neurograph import (
    NeuroGraphClient,
    NotFoundError,
    RateLimitError,
    ValidationError
)
import time

client = NeuroGraphClient(...)

def create_or_update(token_id, text):
    """Create new token or update existing."""
    try:
        # Try to get existing
        token = client.tokens.get(token_id)
        # Update if exists
        return client.tokens.update(token_id, text=text)
    except NotFoundError:
        # Create if doesn't exist
        return client.tokens.create(text=text)

def resilient_create(text, max_attempts=3):
    """Create token with retry on rate limit."""
    for attempt in range(max_attempts):
        try:
            return client.tokens.create(text=text)
        except RateLimitError as e:
            if attempt == max_attempts - 1:
                raise
            print(f"Rate limited, waiting {e.retry_after}s...")
            time.sleep(e.retry_after)
        except ValidationError as e:
            # Don't retry validation errors
            print(f"Invalid input: {e.details}")
            raise

token = resilient_create("resilient token")
```

---

## Performance Tips

1. **Use async client for I/O-bound workloads**
   ```python
   # Much faster for multiple concurrent operations
   async with AsyncNeuroGraphClient(...) as client:
       tasks = [client.tokens.create(text=t) for t in texts]
       tokens = await asyncio.gather(*tasks)
   ```

2. **Reuse client instances**
   ```python
   # Good: One client for all operations
   client = NeuroGraphClient(...)
   for text in texts:
       client.tokens.create(text=text)
   client.close()

   # Bad: New client per operation
   for text in texts:
       client = NeuroGraphClient(...)
       client.tokens.create(text=text)
       client.close()
   ```

3. **Use context managers**
   ```python
   # Auto-cleanup, no resource leaks
   with NeuroGraphClient(...) as client:
       # operations here
   ```

4. **Batch similar operations**
   ```python
   # Better: batch deletes
   token_ids = [1, 2, 3, 4, 5]
   for tid in token_ids:
       client.tokens.delete(tid)
   ```

5. **Use API keys in production**
   ```python
   # Faster auth, no token refresh overhead
   client = NeuroGraphClient(api_key="ng_xxx")
   ```

---

## Troubleshooting

### Import Errors

```bash
# Make sure package is installed
pip install neurograph-python

# Or install from source
pip install -e /path/to/neurograph-os/python-client
```

### Connection Errors

```python
# Check server is running
curl http://localhost:8000/health

# Verify base_url is correct
client = NeuroGraphClient(
    base_url="http://localhost:8000",  # Not https, no trailing slash
    ...
)
```

### Authentication Issues

```python
# Verify credentials
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",  # Correct username
    password="developer123"  # Correct password
)

# Check API key is active
keys = client.api_keys.list()
for key in keys:
    print(f"{key.key_id}: active={key.is_active}")
```

### Timeout Errors

```python
# Increase timeout
client = NeuroGraphClient(
    base_url="http://localhost:8000",
    username="developer",
    password="developer123",
    timeout=60.0  # Increase from default 30s
)
```

---

## API Compatibility

- Compatible with NeuroGraph API **v0.58.0+**
- Automatically handles API version differences
- JWT token refresh supported

---

## Support

- **Documentation**: https://neurograph.dev/docs
- **GitHub**: https://github.com/dchrnv/neurograph-os
- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Discussions**: https://github.com/dchrnv/neurograph-os/discussions
