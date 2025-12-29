/**
 * Retry utilities with exponential backoff.
 */

import { RateLimitError, ServerError, NetworkError } from '../errors';

export interface RetryConfig {
  maxRetries: number;
  initialDelay: number;
  maxDelay: number;
  exponentialBase: number;
  jitter: boolean;
}

const DEFAULT_RETRY_CONFIG: RetryConfig = {
  maxRetries: 3,
  initialDelay: 1000, // 1 second
  maxDelay: 60000, // 60 seconds
  exponentialBase: 2,
  jitter: true,
};

/**
 * Calculate delay for retry attempt with exponential backoff.
 */
export function calculateDelay(attempt: number, config: RetryConfig): number {
  const delay = Math.min(
    config.initialDelay * Math.pow(config.exponentialBase, attempt),
    config.maxDelay
  );

  if (config.jitter) {
    // Add random jitter (0-50% of delay)
    return delay + Math.random() * delay * 0.5;
  }

  return delay;
}

/**
 * Check if error should be retried.
 */
export function shouldRetry(error: any): boolean {
  // Retry on rate limit, server errors, and network errors
  return (
    error instanceof RateLimitError ||
    error instanceof ServerError ||
    error instanceof NetworkError
  );
}

/**
 * Sleep for specified milliseconds.
 */
export function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Retry a function with exponential backoff.
 */
export async function retryWithBackoff<T>(
  fn: () => Promise<T>,
  config: Partial<RetryConfig> = {}
): Promise<T> {
  const retryConfig = { ...DEFAULT_RETRY_CONFIG, ...config };
  let lastError: any;

  for (let attempt = 0; attempt <= retryConfig.maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error) {
      lastError = error;

      // Don't retry if this is the last attempt
      if (attempt === retryConfig.maxRetries) {
        break;
      }

      // Don't retry if error is not retryable
      if (!shouldRetry(error)) {
        throw error;
      }

      // Calculate delay
      let delay: number;
      if (error instanceof RateLimitError && error.retryAfter) {
        // Use retry-after from rate limit response
        delay = error.retryAfter * 1000;
      } else {
        delay = calculateDelay(attempt, retryConfig);
      }

      // Wait before retry
      await sleep(delay);
    }
  }

  throw lastError;
}

/**
 * Decorator for automatic retry with backoff.
 */
export function withRetry<T extends (...args: any[]) => Promise<any>>(
  config: Partial<RetryConfig> = {}
) {
  return function (
    target: any,
    propertyKey: string,
    descriptor: PropertyDescriptor
  ): PropertyDescriptor {
    const originalMethod = descriptor.value;

    descriptor.value = async function (...args: any[]) {
      return retryWithBackoff(() => originalMethod.apply(this, args), config);
    };

    return descriptor;
  };
}
