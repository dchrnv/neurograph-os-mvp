/**
 * NeuroGraph TypeScript/JavaScript Client
 *
 * Official client library for NeuroGraph - semantic knowledge system.
 *
 * @packageDocumentation
 */

export { NeuroGraphClient, type ClientConfig } from './client';

// Models
export type {
  Token,
  TokenCreate,
  TokenUpdate,
  QueryResult,
  APIKey,
  APIKeyCreate,
  User,
  HealthCheck,
  SystemStatus,
  JWTToken,
  ErrorResponse,
} from './models';

// Errors
export {
  NeuroGraphError,
  AuthenticationError,
  AuthorizationError,
  NotFoundError,
  ValidationError,
  RateLimitError,
  ConflictError,
  ServerError,
  NetworkError,
} from './errors';

// Retry utilities
export {
  retryWithBackoff,
  withRetry,
  calculateDelay,
  shouldRetry,
  type RetryConfig,
} from './utils/retry';

// Version
export const VERSION = '0.59.2';
