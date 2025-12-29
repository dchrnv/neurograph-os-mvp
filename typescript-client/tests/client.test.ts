/**
 * Tests for NeuroGraph client.
 *
 * Note: These tests require a running API server.
 * Start with: uvicorn src.api.main:app
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { NeuroGraphClient, NotFoundError, AuthenticationError } from '../src';
import type { Token } from '../src';

describe('NeuroGraphClient', () => {
  let client: NeuroGraphClient;
  let createdTokens: Token[] = [];

  beforeAll(() => {
    client = new NeuroGraphClient({
      baseUrl: 'http://localhost:8000',
      username: 'developer',
      password: 'developer123',
    });
  });

  afterAll(async () => {
    // Cleanup all created tokens
    for (const token of createdTokens) {
      try {
        await client.tokens.delete(token.id);
      } catch {
        // Ignore errors during cleanup
      }
    }
  });

  describe('Health', () => {
    it('should check API health', async () => {
      const health = await client.health.check();
      expect(health.status).toBe('healthy');
      expect(health.version).toBeDefined();
    });

    it('should get system status', async () => {
      const status = await client.health.status();
      expect(status.status).toBe('healthy');
      expect(status.tokens_count).toBeGreaterThanOrEqual(0);
      expect(status.uptime_seconds).toBeGreaterThan(0);
    });
  });

  describe('Tokens', () => {
    it('should create a token', async () => {
      const token = await client.tokens.create({
        text: 'test token',
        metadata: { test: true },
      });

      expect(token.id).toBeDefined();
      expect(token.text).toBe('test token');
      expect(token.embedding).toBeInstanceOf(Array);
      expect(token.embedding.length).toBeGreaterThan(0);
      expect(token.metadata).toEqual({ test: true });

      createdTokens.push(token);
    });

    it('should get token by ID', async () => {
      const created = await client.tokens.create({ text: 'get test' });
      createdTokens.push(created);

      const retrieved = await client.tokens.get(created.id);
      expect(retrieved.id).toBe(created.id);
      expect(retrieved.text).toBe(created.text);
    });

    it('should throw NotFoundError for non-existent token', async () => {
      await expect(client.tokens.get(999999)).rejects.toThrow(NotFoundError);
    });

    it('should list tokens', async () => {
      // Create some tokens
      const tokens = await Promise.all([
        client.tokens.create({ text: 'list test 1' }),
        client.tokens.create({ text: 'list test 2' }),
        client.tokens.create({ text: 'list test 3' }),
      ]);
      createdTokens.push(...tokens);

      const listed = await client.tokens.list({ limit: 10 });
      expect(listed.length).toBeGreaterThanOrEqual(3);
    });

    it('should update token', async () => {
      const token = await client.tokens.create({ text: 'original' });
      createdTokens.push(token);

      const updated = await client.tokens.update(token.id, {
        metadata: { updated: true },
      });

      expect(updated.id).toBe(token.id);
      expect(updated.metadata).toEqual({ updated: true });
    });

    it('should delete token', async () => {
      const token = await client.tokens.create({ text: 'to delete' });

      await client.tokens.delete(token.id);

      await expect(client.tokens.get(token.id)).rejects.toThrow(NotFoundError);
    });

    it('should query similar tokens', async () => {
      const token = await client.tokens.create({ text: 'query test' });
      createdTokens.push(token);

      const results = await client.tokens.query({
        queryVector: token.embedding,
        topK: 5,
      });

      expect(results.length).toBeGreaterThan(0);
      expect(results[0].token.id).toBe(token.id);
      expect(results[0].similarity).toBeGreaterThan(0.99);
    });

    it('should query by text', async () => {
      const results = await client.tokens.queryByText({
        text: 'hello world',
        topK: 5,
      });

      expect(results).toBeInstanceOf(Array);
    });
  });

  describe('API Keys', () => {
    it('should create API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'Test Key',
        scopes: ['tokens:read'],
      });

      expect(apiKey.key_id).toBeDefined();
      expect(apiKey.api_key).toBeDefined();
      expect(apiKey.name).toBe('Test Key');

      // Cleanup
      await client.apiKeys.delete(apiKey.key_id);
    });

    it('should list API keys', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'List Test',
        scopes: ['tokens:read'],
      });

      const keys = await client.apiKeys.list();
      expect(keys.length).toBeGreaterThan(0);

      // Cleanup
      await client.apiKeys.delete(apiKey.key_id);
    });

    it('should get API key', async () => {
      const created = await client.apiKeys.create({
        name: 'Get Test',
        scopes: ['tokens:read'],
      });

      const retrieved = await client.apiKeys.get(created.key_id);
      expect(retrieved.key_id).toBe(created.key_id);
      expect(retrieved.name).toBe('Get Test');

      // Cleanup
      await client.apiKeys.delete(created.key_id);
    });

    it('should revoke API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'Revoke Test',
        scopes: ['tokens:read'],
      });

      await client.apiKeys.revoke(apiKey.key_id);

      const retrieved = await client.apiKeys.get(apiKey.key_id);
      expect(retrieved.is_active).toBe(false);

      // Cleanup
      await client.apiKeys.delete(apiKey.key_id);
    });

    it('should delete API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'Delete Test',
        scopes: ['tokens:read'],
      });

      await client.apiKeys.delete(apiKey.key_id);

      await expect(client.apiKeys.get(apiKey.key_id)).rejects.toThrow(
        NotFoundError
      );
    });
  });

  describe('Authentication', () => {
    it('should throw AuthenticationError with wrong credentials', async () => {
      const badClient = new NeuroGraphClient({
        baseUrl: 'http://localhost:8000',
        username: 'wrong',
        password: 'wrong',
      });

      await expect(badClient.tokens.list()).rejects.toThrow(AuthenticationError);
    });

    it('should work with API key authentication', async () => {
      // Create API key with main client
      const apiKey = await client.apiKeys.create({
        name: 'Auth Test',
        scopes: ['tokens:read', 'tokens:write'],
      });

      // Create new client with API key
      const apiClient = new NeuroGraphClient({
        baseUrl: 'http://localhost:8000',
        apiKey: apiKey.api_key!,
      });

      const health = await apiClient.health.check();
      expect(health.status).toBe('healthy');

      // Cleanup
      await client.apiKeys.delete(apiKey.key_id);
    });
  });
});
