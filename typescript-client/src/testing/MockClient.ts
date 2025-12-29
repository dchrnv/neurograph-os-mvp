/**
 * Mock NeuroGraph client for testing.
 *
 * Provides a mock implementation of NeuroGraphClient that simulates
 * API responses without requiring a live server.
 */

import type {
  Token,
  TokenCreate,
  TokenUpdate,
  QueryResult,
  APIKey,
  APIKeyCreate,
  HealthCheck,
  SystemStatus,
} from '../models';
import { NotFoundError } from '../errors';

/**
 * Mock NeuroGraph client for unit testing.
 *
 * @example
 * ```typescript
 * import { MockNeuroGraphClient } from '@neurograph/client/testing';
 *
 * describe('MyApp', () => {
 *   it('should create token', async () => {
 *     const client = new MockNeuroGraphClient();
 *     const token = await client.tokens.create({ text: 'test' });
 *     expect(token.text).toBe('test');
 *   });
 * });
 * ```
 */
export class MockNeuroGraphClient {
  private tokensStore: Map<number, Token> = new Map();
  private apiKeysStore: Map<string, APIKey> = new Map();
  private nextTokenId = 1;
  private nextKeyId = 1;

  public readonly tokens: MockTokensResource;
  public readonly apiKeys: MockAPIKeysResource;
  public readonly health: MockHealthResource;

  constructor() {
    this.tokens = new MockTokensResource(this);
    this.apiKeys = new MockAPIKeysResource(this);
    this.health = new MockHealthResource(this);
  }

  /**
   * Reset mock client state.
   */
  reset(): void {
    this.tokensStore.clear();
    this.apiKeysStore.clear();
    this.nextTokenId = 1;
    this.nextKeyId = 1;
  }
}

class MockTokensResource {
  constructor(private client: MockNeuroGraphClient) {}

  async create(data: TokenCreate): Promise<Token> {
    const id = (this.client as any).nextTokenId++;

    // Generate fake embedding
    const embedding = Array.from({ length: 768 }, () => Math.random());

    const token: Token = {
      id,
      text: data.text,
      embedding,
      metadata: data.metadata || {},
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    (this.client as any).tokensStore.set(id, token);
    return token;
  }

  async get(tokenId: number): Promise<Token> {
    const token = (this.client as any).tokensStore.get(tokenId);

    if (!token) {
      throw new NotFoundError(
        `Token ${tokenId} not found`,
        'TOKEN_NOT_FOUND'
      );
    }

    return token;
  }

  async list(options: { limit?: number; offset?: number } = {}): Promise<Token[]> {
    const { limit = 100, offset = 0 } = options;
    const tokens = Array.from((this.client as any).tokensStore.values());
    return tokens.slice(offset, offset + limit);
  }

  async update(tokenId: number, data: TokenUpdate): Promise<Token> {
    const token = await this.get(tokenId);

    if (data.text !== undefined) {
      token.text = data.text;
      // Re-generate embedding
      token.embedding = Array.from({ length: 768 }, () => Math.random());
    }

    if (data.metadata !== undefined) {
      token.metadata = data.metadata;
    }

    token.updated_at = new Date().toISOString();

    (this.client as any).tokensStore.set(tokenId, token);
    return token;
  }

  async delete(tokenId: number): Promise<void> {
    const exists = (this.client as any).tokensStore.has(tokenId);

    if (!exists) {
      throw new NotFoundError(
        `Token ${tokenId} not found`,
        'TOKEN_NOT_FOUND'
      );
    }

    (this.client as any).tokensStore.delete(tokenId);
  }

  async query(options: {
    queryVector: number[];
    topK?: number;
    threshold?: number;
  }): Promise<QueryResult[]> {
    const { topK = 10, threshold } = options;
    const tokens = Array.from((this.client as any).tokensStore.values());

    // Generate mock results with random similarities
    const results: QueryResult[] = tokens
      .slice(0, topK)
      .map((token) => ({
        token,
        similarity: 0.7 + Math.random() * 0.3, // 0.7 to 1.0
      }));

    // Filter by threshold if provided
    const filtered = threshold
      ? results.filter((r) => r.similarity >= threshold)
      : results;

    // Sort by similarity descending
    return filtered.sort((a, b) => b.similarity - a.similarity);
  }

  async queryByText(options: {
    text: string;
    topK?: number;
    threshold?: number;
  }): Promise<QueryResult[]> {
    // Create temporary token
    const tempToken = await this.create({ text: options.text });

    // Query using embedding
    const results = await this.query({
      queryVector: tempToken.embedding,
      topK: options.topK,
      threshold: options.threshold,
    });

    // Delete temporary token
    await this.delete(tempToken.id);

    return results;
  }
}

class MockAPIKeysResource {
  constructor(private client: MockNeuroGraphClient) {}

  async create(data: APIKeyCreate): Promise<APIKey> {
    const keyId = `key_${String((this.client as any).nextKeyId++).padStart(8, '0')}`;
    const apiKeyStr = `ng_mock_${keyId}_${Math.floor(Math.random() * 90000) + 10000}`;

    let expiresAt: string | null = null;
    if (data.expires_in_days) {
      const expiresDate = new Date();
      expiresDate.setDate(expiresDate.getDate() + data.expires_in_days);
      expiresAt = expiresDate.toISOString();
    }

    const apiKey: APIKey = {
      key_id: keyId,
      name: data.name,
      scopes: data.scopes,
      created_at: new Date().toISOString(),
      expires_at: expiresAt,
      last_used_at: null,
      is_active: true,
      api_key: apiKeyStr, // Only returned on creation
    };

    (this.client as any).apiKeysStore.set(keyId, apiKey);
    return apiKey;
  }

  async list(): Promise<APIKey[]> {
    const keys = Array.from((this.client as any).apiKeysStore.values());

    // Return without api_key field
    return keys.map((k) => ({
      ...k,
      api_key: undefined,
    }));
  }

  async get(keyId: string): Promise<APIKey> {
    const apiKey = (this.client as any).apiKeysStore.get(keyId);

    if (!apiKey) {
      throw new NotFoundError(
        `API key ${keyId} not found`,
        'API_KEY_NOT_FOUND'
      );
    }

    // Return without api_key field
    return {
      ...apiKey,
      api_key: undefined,
    };
  }

  async revoke(keyId: string): Promise<void> {
    const apiKey = (this.client as any).apiKeysStore.get(keyId);

    if (!apiKey) {
      throw new NotFoundError(
        `API key ${keyId} not found`,
        'API_KEY_NOT_FOUND'
      );
    }

    apiKey.is_active = false;
  }

  async delete(keyId: string): Promise<void> {
    const exists = (this.client as any).apiKeysStore.has(keyId);

    if (!exists) {
      throw new NotFoundError(
        `API key ${keyId} not found`,
        'API_KEY_NOT_FOUND'
      );
    }

    (this.client as any).apiKeysStore.delete(keyId);
  }
}

class MockHealthResource {
  constructor(private client: MockNeuroGraphClient) {}

  async check(): Promise<HealthCheck> {
    return {
      status: 'healthy',
      version: '0.59.0-mock',
      timestamp: new Date().toISOString(),
    };
  }

  async status(): Promise<SystemStatus> {
    return {
      status: 'healthy',
      version: '0.59.0-mock',
      timestamp: new Date().toISOString(),
      uptime_seconds: 12345,
      tokens_count: (this.client as any).tokensStore.size,
      api_keys_count: (this.client as any).apiKeysStore.size,
    };
  }
}

/**
 * Create a mock token for testing.
 */
export function mockToken(overrides: Partial<Token> = {}): Token {
  const defaults: Token = {
    id: 1,
    text: 'mock token text',
    embedding: Array.from({ length: 768 }, () => Math.random()),
    metadata: {},
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  };

  return { ...defaults, ...overrides };
}

/**
 * Create a mock API key for testing.
 */
export function mockAPIKey(overrides: Partial<APIKey> = {}): APIKey {
  const defaults: APIKey = {
    key_id: 'key_00000001',
    name: 'mock api key',
    scopes: ['tokens:read', 'tokens:write'],
    created_at: new Date().toISOString(),
    expires_at: null,
    last_used_at: null,
    is_active: true,
    api_key: 'ng_mock_key_12345',
  };

  return { ...defaults, ...overrides };
}
