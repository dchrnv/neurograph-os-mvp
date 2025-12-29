/**
 * Example: Using retry mechanism with exponential backoff.
 */

import { NeuroGraphClient, retryWithBackoff } from '../src';

async function main() {
  const client = new NeuroGraphClient({
    baseUrl: 'http://localhost:8000',
    username: 'developer',
    password: 'developer123',
  });

  try {
    // Example 1: Retry with default config (3 retries, exponential backoff)
    console.log('=== Retry with Default Config ===');
    const token = await retryWithBackoff(
      () => client.tokens.create({ text: 'resilient token' }),
      {
        maxRetries: 3,
        initialDelay: 1000,
        exponentialBase: 2,
        jitter: true,
      }
    );
    console.log(`Created token: ${token.id}`);

    // Example 2: Custom retry config with more retries
    console.log('\n=== Retry with Custom Config ===');
    const results = await retryWithBackoff(
      () =>
        client.tokens.query({
          queryVector: token.embedding,
          topK: 5,
        }),
      {
        maxRetries: 5,
        initialDelay: 500,
        maxDelay: 10000,
        exponentialBase: 2,
        jitter: true,
      }
    );
    console.log(`Found ${results.length} results`);

    // Example 3: Batch operations with retry
    console.log('\n=== Batch Create with Retry ===');
    const createPromises = Array.from({ length: 10 }, (_, i) =>
      retryWithBackoff(() =>
        client.tokens.create({ text: `batch token ${i}` })
      )
    );
    const batchTokens = await Promise.all(createPromises);
    console.log(`Created ${batchTokens.length} tokens in batch`);

    // Cleanup
    console.log('\n=== Cleanup ===');
    await client.tokens.delete(token.id);
    for (const t of batchTokens) {
      await client.tokens.delete(t.id);
    }
    console.log('Cleanup complete');
  } catch (error) {
    console.error('Error:', error);
  }
}

main();
