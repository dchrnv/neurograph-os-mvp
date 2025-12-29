# TypeScript/JavaScript Client Library - Complete Guide

Official TypeScript/JavaScript client for NeuroGraph API with full type safety.

**Package:** `@neurograph/client`
**Version:** 0.59.2
**Node.js:** 18+
**License:** AGPLv3

---

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Authentication](#authentication)
- [API Reference](#api-reference)
- [Error Handling](#error-handling)
- [Retry Mechanism](#retry-mechanism)
- [TypeScript Support](#typescript-support)
- [Browser Usage](#browser-usage)
- [Advanced Usage](#advanced-usage)
- [Examples](#examples)

---

## Installation

### npm

```bash
npm install @neurograph/client
```

### yarn

```bash
yarn add @neurograph/client
```

### pnpm

```bash
pnpm add @neurograph/client
```

### bun

```bash
bun add @neurograph/client
```

### From source

```bash
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os/typescript-client
npm install
npm run build
npm link
```

---

## Quick Start

### Basic Usage (Node.js/TypeScript)

```typescript
import { NeuroGraphClient } from '@neurograph/client';

// Create client
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

// Create a token
const token = await client.tokens.create({ text: 'hello world' });
console.log(`Token ID: ${token.id}`);
console.log(`Embedding: ${token.embedding.slice(0, 5)}...`);

// Query similar tokens
const results = await client.tokens.query({
  queryVector: token.embedding,
  topK: 10
});

for (const result of results) {
  console.log(`${result.token.text}: ${result.similarity.toFixed(4)}`);
}

// Cleanup
await client.tokens.delete(token.id);
```

### JavaScript (CommonJS)

```javascript
const { NeuroGraphClient } = require('@neurograph/client');

async function main() {
  const client = new NeuroGraphClient({
    baseUrl: 'http://localhost:8000',
    username: 'developer',
    password: 'developer123'
  });

  const token = await client.tokens.create({ text: 'hello world' });
  console.log(token.id);
}

main();
```

### JavaScript (ESM)

```javascript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

const token = await client.tokens.create({ text: 'hello world' });
console.log(token.id);
```

---

## Authentication

### JWT Authentication

```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});
```

**Default users:**
- `admin` / `admin123` - Full access
- `developer` / `developer123` - Read + Write
- `viewer` / `viewer123` - Read only

**Features:**
- Automatic JWT token refresh
- Token caching for performance
- Automatic retry on 401 errors

### API Key Authentication

```typescript
// Step 1: Create API key with JWT
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

const apiKey = await client.apiKeys.create({
  name: 'Production Key',
  scopes: ['tokens:read', 'tokens:write'],
  expires_in_days: 90
});

// IMPORTANT: Save this key - it's only shown once!
console.log(`API Key: ${apiKey.api_key}`);

// Step 2: Use API key
const apiClient = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  apiKey: apiKey.api_key!
});

// Now use apiClient for all operations
const token = await apiClient.tokens.create({ text: 'hello' });
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

```typescript
const token = await client.tokens.create({
  text: 'hello world',
  metadata: { category: 'greeting', language: 'en' }
});

// Returns Token object
console.log(token.id);           // number
console.log(token.text);         // string
console.log(token.embedding);    // number[]
console.log(token.metadata);     // Record<string, any>
console.log(token.created_at);   // string (ISO 8601)
console.log(token.updated_at);   // string (ISO 8601)
```

#### Get Token

```typescript
const token = await client.tokens.get(123);

// Throws NotFoundError if token doesn't exist
```

#### List Tokens

```typescript
// List with pagination
const tokens = await client.tokens.list({ limit: 100, offset: 0 });

// Get all tokens (use with caution)
const allTokens = await client.tokens.list({ limit: 10000 });

// Default limit is 100
const tokens = await client.tokens.list();
```

#### Update Token

```typescript
// Update text and/or metadata
const updated = await client.tokens.update(123, {
  text: 'updated text',           // Optional
  metadata: { updated: true }      // Optional
});

// Update only metadata
const updated = await client.tokens.update(123, {
  metadata: { status: 'processed' }
});
```

#### Delete Token

```typescript
// Delete returns void on success
await client.tokens.delete(123);

// Throws NotFoundError if token doesn't exist
```

#### Query Similar Tokens

```typescript
// Query by embedding vector
const results = await client.tokens.query({
  queryVector: [0.1, 0.2, 0.3, ...],  // number[]
  topK: 10,                             // Optional, default=10
  threshold: 0.8                        // Optional, minimum similarity
});

// Each result has:
for (const result of results) {
  console.log(result.token);       // Token object
  console.log(result.similarity);  // number (0.0 to 1.0)
}

// Common pattern: query using existing token
const token = await client.tokens.get(123);
const similar = await client.tokens.query({
  queryVector: token.embedding,
  topK: 5
});
```

#### Query by Text (Convenience Method)

```typescript
// Creates temporary token, queries, and cleans up
const results = await client.tokens.queryByText({
  text: 'hello world',
  topK: 10,
  threshold: 0.8
});

// This is equivalent to:
// 1. Create token with text
// 2. Query using its embedding
// 3. Delete temporary token
```

### API Keys

#### Create API Key

```typescript
const apiKey = await client.apiKeys.create({
  name: 'My Application',
  scopes: ['tokens:read', 'tokens:write'],
  expires_in_days: 30  // Optional, default=undefined (never expires)
});

// SAVE THIS - only shown once!
console.log(apiKey.api_key);  // Full key: ng_xxx...
console.log(apiKey.key_id);   // Key ID for management
```

#### List API Keys

```typescript
const keys = await client.apiKeys.list();

for (const key of keys) {
  console.log(`${key.name}: ${key.key_id}`);
  console.log(`  Scopes: ${key.scopes}`);
  console.log(`  Active: ${key.is_active}`);
  console.log(`  Expires: ${key.expires_at}`);
}
```

#### Get API Key

```typescript
const key = await client.apiKeys.get('key_abc123');

// Returns metadata only, NOT the actual key
console.log(key.name);
console.log(key.scopes);
console.log(key.is_active);
```

#### Revoke API Key

```typescript
// Makes key inactive but keeps in database
await client.apiKeys.revoke('key_abc123');

// Can still view key details but can't use for auth
```

#### Delete API Key

```typescript
// Permanently deletes key
await client.apiKeys.delete('key_abc123');

// Throws NotFoundError if key doesn't exist
```

### Health

#### Health Check

```typescript
const health = await client.health.check();

console.log(health.status);     // "healthy" or "unhealthy"
console.log(health.version);    // API version
console.log(health.timestamp);  // Current server time
```

#### System Status

```typescript
const status = await client.health.status();

console.log(status.status);          // "healthy" or "unhealthy"
console.log(status.version);         // API version
console.log(status.uptime_seconds);  // Server uptime
console.log(status.tokens_count);    // Total tokens in system
console.log(status.api_keys_count);  // Total API keys
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
├── ServerError (500+)
└── NetworkError (connection issues)
```

### Basic Error Handling

```typescript
import {
  NeuroGraphError,
  NotFoundError,
  AuthenticationError,
  ValidationError,
  RateLimitError
} from '@neurograph/client';

try {
  const token = await client.tokens.get(999999);
} catch (error) {
  if (error instanceof NotFoundError) {
    console.log(`Token not found: ${error.message}`);
    console.log(`Error code: ${error.errorCode}`);
  } else if (error instanceof AuthenticationError) {
    console.log(`Auth failed: ${error.message}`);
  } else if (error instanceof NeuroGraphError) {
    console.log(`API error: ${error.message}`);
    console.log(`Details: ${error.details}`);
  }
}
```

### Graceful Degradation

```typescript
// Pattern: Fallback to default if not found
let token;
try {
  token = await client.tokens.get(123);
} catch (error) {
  if (error instanceof NotFoundError) {
    // Create if doesn't exist
    token = await client.tokens.create({ text: 'default token' });
  } else {
    throw error;
  }
}
```

### Rate Limit Handling

```typescript
import { RateLimitError } from '@neurograph/client';

async function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

try {
  const token = await client.tokens.create({ text: 'test' });
} catch (error) {
  if (error instanceof RateLimitError) {
    // Wait for the specified time
    console.log(`Rate limited. Waiting ${error.retryAfter}s...`);
    await sleep(error.retryAfter! * 1000);

    // Retry
    const token = await client.tokens.create({ text: 'test' });
  }
}
```

### Validation Errors

```typescript
import { ValidationError } from '@neurograph/client';

try {
  const token = await client.tokens.create({ text: '' });  // Empty text
} catch (error) {
  if (error instanceof ValidationError) {
    console.log(`Validation failed: ${error.message}`);
    console.log(`Details:`, error.details);
    // Details contain field-specific errors
  }
}
```

---

## Retry Mechanism

### Using retryWithBackoff

```typescript
import { retryWithBackoff } from '@neurograph/client';

// Simple retry with defaults
const token = await retryWithBackoff(
  () => client.tokens.create({ text: 'resilient token' })
);

// Custom retry configuration
const token = await retryWithBackoff(
  () => client.tokens.create({ text: 'very resilient' }),
  {
    maxRetries: 5,
    initialDelay: 500,       // 500ms
    maxDelay: 30000,         // 30 seconds max
    exponentialBase: 2,      // Double each time
    jitter: true             // Add randomness
  }
);
```

### Retry for Batch Operations

```typescript
import { retryWithBackoff } from '@neurograph/client';

const texts = ['token1', 'token2', 'token3'];

const promises = texts.map(text =>
  retryWithBackoff(() => client.tokens.create({ text }))
);

const tokens = await Promise.all(promises);
console.log(`Created ${tokens.length} tokens`);
```

### What Gets Retried

**Automatically retried:**
- `RateLimitError` (429)
- `ServerError` (500, 502, 503, 504)
- `NetworkError` (connection failures)

**NOT retried:**
- `AuthenticationError` (401)
- `AuthorizationError` (403)
- `NotFoundError` (404)
- `ValidationError` (422)
- `ConflictError` (409)

---

## TypeScript Support

### Full Type Definitions

```typescript
import type {
  Token,
  TokenCreate,
  TokenUpdate,
  QueryResult,
  APIKey,
  APIKeyCreate,
  HealthCheck,
  SystemStatus
} from '@neurograph/client';

// Type-safe token creation
const data: TokenCreate = {
  text: 'hello world',
  metadata: { foo: 'bar' }
};

const token: Token = await client.tokens.create(data);

// Type-safe query results
const results: QueryResult[] = await client.tokens.query({
  queryVector: token.embedding,
  topK: 5
});
```

### Client Configuration Types

```typescript
import { NeuroGraphClient, type ClientConfig } from '@neurograph/client';

const config: ClientConfig = {
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123',
  timeout: 30000,
  headers: {
    'X-Custom-Header': 'value'
  }
};

const client = new NeuroGraphClient(config);
```

### Generic Error Handling

```typescript
import { NeuroGraphError } from '@neurograph/client';

async function handleOperation<T>(
  operation: () => Promise<T>
): Promise<T | null> {
  try {
    return await operation();
  } catch (error) {
    if (error instanceof NeuroGraphError) {
      console.error(`API error: ${error.errorCode}`);
      return null;
    }
    throw error;
  }
}

const token = await handleOperation(() =>
  client.tokens.create({ text: 'test' })
);
```

---

## Browser Usage

### With Module Bundler (Webpack, Vite, etc.)

```typescript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'https://api.neurograph.dev',
  apiKey: 'ng_your_api_key_here'
});

// Use in React component
function TokenCreator() {
  const [token, setToken] = useState(null);

  const createToken = async () => {
    const newToken = await client.tokens.create({ text: 'hello' });
    setToken(newToken);
  };

  return <button onClick={createToken}>Create Token</button>;
}
```

### CORS Considerations

```typescript
// Ensure your API server allows CORS
// In Python FastAPI:
// from fastapi.middleware.cors import CORSMiddleware
// app.add_middleware(
//     CORSMiddleware,
//     allow_origins=["http://localhost:3000"],
//     allow_credentials=True,
//     allow_methods=["*"],
//     allow_headers=["*"],
// )

const client = new NeuroGraphClient({
  baseUrl: 'https://api.neurograph.dev',
  apiKey: 'ng_your_api_key_here'
});
```

### Browser Limitations

- ⚠️ Never expose API credentials in browser code
- ✅ Use API keys with limited scopes
- ✅ Implement server-side proxy if needed
- ✅ Use HTTPS in production

---

## Advanced Usage

### Concurrent Operations

```typescript
// Create multiple tokens concurrently
const texts = ['token1', 'token2', 'token3', 'token4', 'token5'];

const promises = texts.map(text =>
  client.tokens.create({ text })
);

const tokens = await Promise.all(promises);
console.log(`Created ${tokens.length} tokens`);
```

### Custom HTTP Client Configuration

```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123',
  timeout: 60000,  // 60 seconds
  headers: {
    'X-Custom-Header': 'value',
    'User-Agent': 'MyApp/1.0'
  }
});

// Access underlying axios instance for advanced usage
const httpClient = client.getHttpClient();

// Add custom interceptor
httpClient.interceptors.request.use((config) => {
  console.log('Request:', config.method, config.url);
  return config;
});
```

### Pagination Helper

```typescript
async function* getAllTokens(client: NeuroGraphClient) {
  let offset = 0;
  const limit = 100;

  while (true) {
    const batch = await client.tokens.list({ limit, offset });
    if (batch.length === 0) break;

    for (const token of batch) {
      yield token;
    }

    offset += limit;
  }
}

// Usage
for await (const token of getAllTokens(client)) {
  console.log(token.text);
}
```

### Connection Pooling

```typescript
// Client automatically uses connection pooling via axios
// Reuse client instance for best performance

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

// Good: Reuse client
for (let i = 0; i < 100; i++) {
  await client.tokens.create({ text: `token ${i}` });
}

// Bad: New client per request
for (let i = 0; i < 100; i++) {
  const newClient = new NeuroGraphClient({...});
  await newClient.tokens.create({ text: `token ${i}` });
}
```

---

## Examples

### Example 1: Semantic Search

```typescript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

// Index documents
const documents = [
  'Python is a programming language',
  'JavaScript is used for web development',
  'Machine learning uses neural networks',
  'Deep learning is a subset of ML'
];

const tokens = await Promise.all(
  documents.map(doc =>
    client.tokens.create({
      text: doc,
      metadata: { type: 'document' }
    })
  )
);

// Search
const query = await client.tokens.create({ text: 'programming languages' });
const results = await client.tokens.query({
  queryVector: query.embedding,
  topK: 3
});

console.log('Search results:');
for (const result of results) {
  console.log(`  ${result.token.text} (${result.similarity.toFixed(4)})`);
}

// Cleanup
await client.tokens.delete(query.id);
for (const token of tokens) {
  await client.tokens.delete(token.id);
}
```

### Example 2: Batch Processing with Retry

```typescript
import { NeuroGraphClient, retryWithBackoff } from '@neurograph/client';

async function processBatch(texts: string[]) {
  const client = new NeuroGraphClient({
    baseUrl: 'http://localhost:8000',
    username: 'developer',
    password: 'developer123'
  });

  // Process with retry
  const promises = texts.map(text =>
    retryWithBackoff(
      () => client.tokens.create({ text }),
      { maxRetries: 5, jitter: true }
    )
  );

  const tokens = await Promise.all(promises);
  console.log(`Created ${tokens.length} tokens`);
  return tokens;
}

// Process large batch
const texts = Array.from({ length: 100 }, (_, i) => `document ${i}`);
const tokens = await processBatch(texts);
```

### Example 3: Error Recovery

```typescript
import {
  NeuroGraphClient,
  NotFoundError,
  RateLimitError,
  ValidationError
} from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123'
});

async function createOrUpdate(tokenId: number, text: string) {
  /**
   * Create new token or update existing.
   */
  try {
    // Try to get existing
    const token = await client.tokens.get(tokenId);
    // Update if exists
    return await client.tokens.update(tokenId, { text });
  } catch (error) {
    if (error instanceof NotFoundError) {
      // Create if doesn't exist
      return await client.tokens.create({ text });
    }
    throw error;
  }
}

async function resilientCreate(
  text: string,
  maxAttempts: number = 3
) {
  /**
   * Create token with retry on rate limit.
   */
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      return await client.tokens.create({ text });
    } catch (error) {
      if (error instanceof RateLimitError) {
        if (attempt === maxAttempts - 1) throw error;

        const delay = error.retryAfter || Math.pow(2, attempt);
        console.log(`Rate limited, waiting ${delay}s...`);
        await new Promise(resolve => setTimeout(resolve, delay * 1000));
      } else if (error instanceof ValidationError) {
        // Don't retry validation errors
        console.error(`Invalid input: ${error.details}`);
        throw error;
      } else {
        throw error;
      }
    }
  }
}

const token = await resilientCreate('resilient token');
```

### Example 4: React Hook

```typescript
import { useState, useEffect } from 'react';
import { NeuroGraphClient, type Token } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'https://api.neurograph.dev',
  apiKey: process.env.REACT_APP_NEUROGRAPH_API_KEY!
});

function useTokens(limit: number = 10) {
  const [tokens, setTokens] = useState<Token[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    async function fetchTokens() {
      try {
        setLoading(true);
        const data = await client.tokens.list({ limit });
        setTokens(data);
      } catch (err) {
        setError(err as Error);
      } finally {
        setLoading(false);
      }
    }

    fetchTokens();
  }, [limit]);

  return { tokens, loading, error };
}

// Usage in component
function TokenList() {
  const { tokens, loading, error } = useTokens(20);

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <ul>
      {tokens.map(token => (
        <li key={token.id}>{token.text}</li>
      ))}
    </ul>
  );
}
```

---

## Performance Tips

1. **Reuse client instances**
   ```typescript
   // Good: One client for all operations
   const client = new NeuroGraphClient({...});
   for (const text of texts) {
     await client.tokens.create({ text });
   }

   // Bad: New client per operation
   for (const text of texts) {
     const client = new NeuroGraphClient({...});
     await client.tokens.create({ text });
   }
   ```

2. **Use Promise.all for concurrent operations**
   ```typescript
   // Much faster for independent operations
   const promises = texts.map(text => client.tokens.create({ text }));
   const tokens = await Promise.all(promises);
   ```

3. **Use API keys in production**
   ```typescript
   // Faster auth, no token refresh overhead
   const client = new NeuroGraphClient({ apiKey: 'ng_xxx' });
   ```

4. **Enable HTTP keep-alive**
   ```typescript
   // Axios uses keep-alive by default
   // Connection pooling is automatic
   ```

5. **Batch deletions**
   ```typescript
   const tokenIds = [1, 2, 3, 4, 5];
   await Promise.all(tokenIds.map(id => client.tokens.delete(id)));
   ```

---

## Troubleshooting

### Module Resolution Errors

```bash
# Make sure package is installed
npm install @neurograph/client

# Clear cache if needed
rm -rf node_modules package-lock.json
npm install
```

### TypeScript Errors

```typescript
// Ensure tsconfig.json has correct settings
{
  "compilerOptions": {
    "moduleResolution": "node",
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true
  }
}
```

### Connection Errors

```typescript
// Check server is running
// curl http://localhost:8000/health

// Verify baseUrl is correct
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',  // Not https, no trailing slash
  ...
});
```

### Authentication Issues

```typescript
// Verify credentials
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',  // Correct username
  password: 'developer123'  // Correct password
});

// Check API key is active
const keys = await client.apiKeys.list();
for (const key of keys) {
  console.log(`${key.key_id}: active=${key.is_active}`);
}
```

### Timeout Errors

```typescript
// Increase timeout
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123',
  timeout: 60000  // Increase from default 30s
});
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
- **npm**: https://www.npmjs.com/package/@neurograph/client
