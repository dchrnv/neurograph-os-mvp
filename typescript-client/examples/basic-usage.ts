/**
 * Basic usage examples for NeuroGraph TypeScript client.
 */

import { NeuroGraphClient } from '../src';

async function main() {
  // Initialize client with JWT authentication
  const client = new NeuroGraphClient({
    baseUrl: 'http://localhost:8000',
    username: 'developer',
    password: 'developer123',
  });

  try {
    // Health check
    console.log('=== Health Check ===');
    const health = await client.health.check();
    console.log(`Status: ${health.status}, Version: ${health.version}`);

    // Create a token
    console.log('\n=== Create Token ===');
    const token = await client.tokens.create({
      text: 'hello world',
      metadata: { category: 'greeting' },
    });
    console.log(`Created token: ${token.id}`);
    console.log(`Text: ${token.text}`);
    console.log(`Embedding length: ${token.embedding.length}`);

    // Get token
    console.log('\n=== Get Token ===');
    const retrieved = await client.tokens.get(token.id);
    console.log(`Retrieved: ${retrieved.text}`);

    // List tokens
    console.log('\n=== List Tokens ===');
    const tokens = await client.tokens.list({ limit: 5 });
    console.log(`Found ${tokens.length} tokens`);
    tokens.forEach((t) => console.log(`  - ${t.id}: ${t.text}`));

    // Query similar tokens
    console.log('\n=== Query Tokens ===');
    const results = await client.tokens.query({
      queryVector: token.embedding,
      topK: 5,
    });
    console.log(`Found ${results.length} similar tokens:`);
    results.forEach((r) => {
      console.log(`  - ${r.token.text}: ${r.similarity.toFixed(4)}`);
    });

    // Query by text (convenience method)
    console.log('\n=== Query by Text ===');
    const textResults = await client.tokens.queryByText({
      text: 'hello',
      topK: 5,
    });
    console.log(`Found ${textResults.length} results for "hello"`);

    // Update token
    console.log('\n=== Update Token ===');
    const updated = await client.tokens.update(token.id, {
      metadata: { category: 'greeting', updated: true },
    });
    console.log(`Updated metadata: ${JSON.stringify(updated.metadata)}`);

    // Create API key
    console.log('\n=== Create API Key ===');
    const apiKey = await client.apiKeys.create({
      name: 'Test Key',
      scopes: ['tokens:read', 'tokens:write'],
      expires_in_days: 30,
    });
    console.log(`API Key created: ${apiKey.key_id}`);
    console.log(`Key (save this!): ${apiKey.api_key}`);

    // List API keys
    console.log('\n=== List API Keys ===');
    const keys = await client.apiKeys.list();
    console.log(`Found ${keys.length} API keys`);

    // System status
    console.log('\n=== System Status ===');
    const status = await client.health.status();
    console.log(`Tokens: ${status.tokens_count}`);
    console.log(`API Keys: ${status.api_keys_count}`);
    console.log(`Uptime: ${status.uptime_seconds}s`);

    // Cleanup
    console.log('\n=== Cleanup ===');
    await client.tokens.delete(token.id);
    await client.apiKeys.delete(apiKey.key_id);
    console.log('Cleanup complete');
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
