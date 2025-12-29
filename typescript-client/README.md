# NeuroGraph TypeScript/JavaScript Client

Official TypeScript/JavaScript client library for [NeuroGraph](https://github.com/dchrnv/neurograph-os) - semantic knowledge system based on token embeddings.

[![npm version](https://img.shields.io/npm/v/@neurograph/client)](https://www.npmjs.com/package/@neurograph/client)
[![License](https://img.shields.io/badge/license-AGPLv3-blue.svg)](LICENSE)

## Features

- ✅ **Full TypeScript** support with comprehensive type definitions
- ✅ **Works everywhere**: Node.js, Browser, Deno, Bun
- ✅ **JWT and API Key** authentication
- ✅ **Automatic token refresh** for JWT
- ✅ **Comprehensive error handling** with 8 exception types
- ✅ **Retry mechanism** with exponential backoff
- ✅ **Promise-based** async/await API
- ✅ **Tree-shakeable** ESM and CJS builds
- ✅ **Zero dependencies** (only axios)

## Installation

```bash
npm install @neurograph/client
```

Or with other package managers:

```bash
yarn add @neurograph/client
pnpm add @neurograph/client
bun add @neurograph/client
```

## Quick Start

### Basic Usage

```typescript
import { NeuroGraphClient } from '@neurograph/client';

// Initialize with JWT authentication
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
```

### With API Key

```typescript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  apiKey: 'ng_your_api_key_here'
});

const token = await client.tokens.create({ text: 'hello world' });
```

## Authentication

### JWT Authentication (Username/Password)

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

### API Key Authentication

```typescript
// First, create an API key using JWT
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

console.log(`Save this key: ${apiKey.api_key}`);

// Then use the API key
const apiClient = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  apiKey: apiKey.api_key
});
```

## API Reference

### Tokens

```typescript
// Create token
const token = await client.tokens.create({
  text: 'hello world',
  metadata: { category: 'greeting' }
});

// Get token by ID
const token = await client.tokens.get(123);

// List tokens
const tokens = await client.tokens.list({ limit: 100, offset: 0 });

// Update token
const updated = await client.tokens.update(123, {
  text: 'new text',
  metadata: { updated: true }
});

// Delete token
await client.tokens.delete(123);

// Query similar tokens
const results = await client.tokens.query({
  queryVector: [0.1, 0.2, ...],
  topK: 10,
  threshold: 0.8
});

// Query by text (convenience method)
const results = await client.tokens.queryByText({
  text: 'hello',
  topK: 10
});
```

### API Keys

```typescript
// Create API key (returns full key - save it!)
const apiKey = await client.apiKeys.create({
  name: 'Integration Key',
  scopes: ['tokens:read', 'tokens:write'],
  expires_in_days: 30
});

// List API keys
const keys = await client.apiKeys.list();

// Get API key details
const key = await client.apiKeys.get('key_123');

// Revoke API key
await client.apiKeys.revoke('key_123');

// Delete API key
await client.apiKeys.delete('key_123');
```

### Health Checks

```typescript
// Check API health
const health = await client.health.check();
console.log(health.status, health.version);

// Get system status
const status = await client.health.status();
console.log(`Tokens: ${status.tokens_count}`);
console.log(`Uptime: ${status.uptime_seconds}s`);
```

## Error Handling

```typescript
import {
  NeuroGraphError,
  AuthenticationError,
  AuthorizationError,
  NotFoundError,
  ValidationError,
  RateLimitError,
  ServerError,
  NetworkError,
} from '@neurograph/client';

try {
  const token = await client.tokens.get(999999);
} catch (error) {
  if (error instanceof NotFoundError) {
    console.log('Token not found');
  } else if (error instanceof AuthenticationError) {
    console.log('Auth failed');
  } else if (error instanceof RateLimitError) {
    console.log(`Rate limited. Retry after ${error.retryAfter}s`);
  } else if (error instanceof NeuroGraphError) {
    console.log(`Error [${error.errorCode}]: ${error.message}`);
  }
}
```

## Retry Mechanism

```typescript
import { retryWithBackoff } from '@neurograph/client';

// Automatic retry with exponential backoff
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

// Batch operations with retry
const promises = Array.from({ length: 10 }, (_, i) =>
  retryWithBackoff(() => client.tokens.create({ text: `token ${i}` }))
);
const tokens = await Promise.all(promises);
```

## Configuration

```typescript
const client = new NeuroGraphClient({
  baseUrl: 'http://localhost:8000',
  username: 'developer',
  password: 'developer123',
  timeout: 30000,              // Request timeout in ms
  headers: {                   // Custom headers
    'X-Custom-Header': 'value'
  }
});
```

## Examples

See the [examples/](examples/) directory for more examples:

- [basic-usage.ts](examples/basic-usage.ts) - Comprehensive usage examples
- [with-retry.ts](examples/with-retry.ts) - Retry mechanism patterns
- [error-handling.ts](examples/error-handling.ts) - Error handling patterns

## Browser Usage

The client works in browsers via bundlers (webpack, vite, etc.):

```typescript
import { NeuroGraphClient } from '@neurograph/client';

const client = new NeuroGraphClient({
  baseUrl: 'https://api.neurograph.dev',
  apiKey: 'ng_your_api_key_here'
});

const token = await client.tokens.create({ text: 'hello from browser' });
```

## Node.js Usage

Works with both CommonJS and ESM:

```javascript
// ESM
import { NeuroGraphClient } from '@neurograph/client';

// CommonJS
const { NeuroGraphClient } = require('@neurograph/client');
```

## TypeScript Support

Full TypeScript support with comprehensive type definitions:

```typescript
import type { Token, TokenCreate, QueryResult } from '@neurograph/client';

const data: TokenCreate = {
  text: 'hello world',
  metadata: { foo: 'bar' }
};

const token: Token = await client.tokens.create(data);
const results: QueryResult[] = await client.tokens.query({
  queryVector: token.embedding,
  topK: 5
});
```

## Requirements

- Node.js 18+ (for Node.js usage)
- Modern browser with ES2020 support (for browser usage)

## Development

```bash
# Clone repository
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os/typescript-client

# Install dependencies
npm install

# Build
npm run build

# Run tests
npm test

# Run tests in watch mode
npm run test:watch

# Lint
npm run lint

# Format
npm run format
```

## API Compatibility

This client is compatible with NeuroGraph API **v0.58.0+**

## License

AGPLv3 - See [LICENSE](../LICENSE) for details

## Links

- **Documentation**: https://neurograph.dev/docs
- **GitHub**: https://github.com/dchrnv/neurograph-os
- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **npm**: https://www.npmjs.com/package/@neurograph/client

## Support

- GitHub Issues: https://github.com/dchrnv/neurograph-os/issues
- GitHub Discussions: https://github.com/dchrnv/neurograph-os/discussions

---

**Built with ❤️ by the NeuroGraph team**
