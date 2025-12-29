/**
 * Tests for mock NeuroGraph client.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { MockNeuroGraphClient, mockToken, mockAPIKey } from '../src/testing';
import { NotFoundError } from '../src';

describe('MockNeuroGraphClient', () => {
  let client: MockNeuroGraphClient;

  beforeEach(() => {
    client = new MockNeuroGraphClient();
  });

  describe('Tokens', () => {
    it('should create token', async () => {
      const token = await client.tokens.create({ text: 'test token' });

      expect(token.id).toBe(1);
      expect(token.text).toBe('test token');
      expect(token.embedding).toHaveLength(768);
      expect(token.metadata).toEqual({});
    });

    it('should get token', async () => {
      const created = await client.tokens.create({ text: 'test' });
      const retrieved = await client.tokens.get(created.id);

      expect(retrieved.id).toBe(created.id);
      expect(retrieved.text).toBe(created.text);
    });

    it('should throw NotFoundError for non-existent token', async () => {
      await expect(client.tokens.get(999)).rejects.toThrow(NotFoundError);
    });

    it('should list tokens', async () => {
      // Create some tokens
      for (let i = 0; i < 5; i++) {
        await client.tokens.create({ text: `token ${i}` });
      }

      const tokens = await client.tokens.list();
      expect(tokens).toHaveLength(5);
    });

    it('should update token', async () => {
      const token = await client.tokens.create({ text: 'original' });
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

    it('should query tokens', async () => {
      // Create some tokens
      for (let i = 0; i < 10; i++) {
        await client.tokens.create({ text: `token ${i}` });
      }

      const results = await client.tokens.query({
        queryVector: Array(768).fill(0.1),
        topK: 5,
      });

      expect(results.length).toBeLessThanOrEqual(5);
      results.forEach((r) => {
        expect(r.token).toBeDefined();
        expect(r.similarity).toBeGreaterThanOrEqual(0);
        expect(r.similarity).toBeLessThanOrEqual(1);
      });
    });

    it('should query by text', async () => {
      // Create some tokens
      for (let i = 0; i < 5; i++) {
        await client.tokens.create({ text: `token ${i}` });
      }

      const results = await client.tokens.queryByText({
        text: 'search query',
        topK: 3,
      });

      expect(results.length).toBeLessThanOrEqual(3);
    });
  });

  describe('API Keys', () => {
    it('should create API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'test key',
        scopes: ['tokens:read'],
      });

      expect(apiKey.key_id).toMatch(/^key_/);
      expect(apiKey.api_key).toMatch(/^ng_mock_/);
      expect(apiKey.name).toBe('test key');
      expect(apiKey.scopes).toEqual(['tokens:read']);
      expect(apiKey.is_active).toBe(true);
    });

    it('should list API keys', async () => {
      // Create some keys
      for (let i = 0; i < 3; i++) {
        await client.apiKeys.create({
          name: `key ${i}`,
          scopes: ['tokens:read'],
        });
      }

      const keys = await client.apiKeys.list();

      expect(keys).toHaveLength(3);
      // api_key field should not be included in list
      keys.forEach((k) => {
        expect(k.api_key).toBeUndefined();
      });
    });

    it('should revoke API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'test',
        scopes: ['tokens:read'],
      });

      await client.apiKeys.revoke(apiKey.key_id);

      const retrieved = await client.apiKeys.get(apiKey.key_id);
      expect(retrieved.is_active).toBe(false);
    });

    it('should delete API key', async () => {
      const apiKey = await client.apiKeys.create({
        name: 'test',
        scopes: ['tokens:read'],
      });

      await client.apiKeys.delete(apiKey.key_id);

      await expect(client.apiKeys.get(apiKey.key_id)).rejects.toThrow(
        NotFoundError
      );
    });
  });

  describe('Health', () => {
    it('should check health', async () => {
      const health = await client.health.check();

      expect(health.status).toBe('healthy');
      expect(health.version).toBe('0.59.0-mock');
    });

    it('should get system status', async () => {
      // Create some data
      await client.tokens.create({ text: 'test' });
      await client.apiKeys.create({ name: 'test', scopes: ['tokens:read'] });

      const status = await client.health.status();

      expect(status.status).toBe('healthy');
      expect(status.tokens_count).toBe(1);
      expect(status.api_keys_count).toBe(1);
    });
  });

  describe('Reset', () => {
    it('should reset client state', async () => {
      // Create some data
      await client.tokens.create({ text: 'test' });
      await client.apiKeys.create({ name: 'test', scopes: ['tokens:read'] });

      // Reset
      client.reset();

      // Verify state is cleared
      const tokens = await client.tokens.list();
      const keys = await client.apiKeys.list();

      expect(tokens).toHaveLength(0);
      expect(keys).toHaveLength(0);
    });
  });
});

describe('mockToken', () => {
  it('should create mock token with defaults', () => {
    const token = mockToken();

    expect(token.id).toBe(1);
    expect(token.text).toBe('mock token text');
    expect(token.embedding).toHaveLength(768);
  });

  it('should create mock token with overrides', () => {
    const token = mockToken({
      id: 123,
      text: 'custom text',
    });

    expect(token.id).toBe(123);
    expect(token.text).toBe('custom text');
  });
});

describe('mockAPIKey', () => {
  it('should create mock API key with defaults', () => {
    const apiKey = mockAPIKey();

    expect(apiKey.key_id).toBe('key_00000001');
    expect(apiKey.name).toBe('mock api key');
    expect(apiKey.api_key).toMatch(/^ng_mock_/);
  });

  it('should create mock API key with overrides', () => {
    const apiKey = mockAPIKey({
      name: 'custom key',
      scopes: ['tokens:write'],
    });

    expect(apiKey.name).toBe('custom key');
    expect(apiKey.scopes).toEqual(['tokens:write']);
  });
});
