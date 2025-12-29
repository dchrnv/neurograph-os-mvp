/**
 * Example: Error handling patterns.
 */

import {
  NeuroGraphClient,
  NotFoundError,
  AuthenticationError,
  ValidationError,
  RateLimitError,
  ServerError,
} from '../src';

async function main() {
  const client = new NeuroGraphClient({
    baseUrl: 'http://localhost:8000',
    username: 'developer',
    password: 'developer123',
  });

  try {
    // Pattern 1: Graceful degradation with NotFoundError
    console.log('=== Graceful Degradation ===');
    let token;
    try {
      token = await client.tokens.get(999999);
    } catch (error) {
      if (error instanceof NotFoundError) {
        console.log('Token not found, creating new one...');
        token = await client.tokens.create({ text: 'fallback token' });
      } else {
        throw error;
      }
    }
    console.log(`Token: ${token.id}`);

    // Pattern 2: Validation error handling
    console.log('\n=== Validation Error Handling ===');
    try {
      await client.tokens.create({ text: '' }); // Empty text - should fail
    } catch (error) {
      if (error instanceof ValidationError) {
        console.log('Validation failed:', error.message);
        console.log('Details:', error.details);
      }
    }

    // Pattern 3: Rate limit handling
    console.log('\n=== Rate Limit Handling ===');
    try {
      // Simulate rate limiting by making many requests
      for (let i = 0; i < 100; i++) {
        await client.tokens.create({ text: `spam ${i}` });
      }
    } catch (error) {
      if (error instanceof RateLimitError) {
        console.log('Rate limited!');
        console.log(`Retry after: ${error.retryAfter}s`);
        console.log('Backing off...');
      }
    }

    // Pattern 4: Server error handling
    console.log('\n=== Server Error Handling ===');
    try {
      // This might fail if server is overloaded
      await client.tokens.list({ limit: 10000 });
    } catch (error) {
      if (error instanceof ServerError) {
        console.log('Server error:', error.message);
        console.log('Status code:', error.statusCode);
        console.log('Retrying with smaller limit...');
        const tokens = await client.tokens.list({ limit: 10 });
        console.log(`Success: got ${tokens.length} tokens`);
      }
    }

    // Pattern 5: Comprehensive error handling
    console.log('\n=== Comprehensive Error Handling ===');
    try {
      const results = await client.tokens.query({
        queryVector: token.embedding,
        topK: 5,
      });
      console.log(`Found ${results.length} results`);
    } catch (error) {
      if (error instanceof AuthenticationError) {
        console.error('Authentication failed - check credentials');
      } else if (error instanceof ValidationError) {
        console.error('Invalid input:', error.details);
      } else if (error instanceof RateLimitError) {
        console.error('Rate limited - wait and retry');
      } else if (error instanceof ServerError) {
        console.error('Server error - try again later');
      } else {
        console.error('Unknown error:', error);
      }
    }

    // Cleanup
    console.log('\n=== Cleanup ===');
    await client.tokens.delete(token.id);
    console.log('Cleanup complete');
  } catch (error) {
    console.error('Fatal error:', error);
  }
}

main();
