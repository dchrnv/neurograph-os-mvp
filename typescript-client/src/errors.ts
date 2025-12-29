/**
 * Error classes for NeuroGraph client.
 */

import type { ErrorResponse } from './models';

/**
 * Base error class for all NeuroGraph errors.
 */
export class NeuroGraphError extends Error {
  public readonly errorCode: string;
  public readonly details?: any;
  public readonly statusCode?: number;

  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message);
    this.name = 'NeuroGraphError';
    this.errorCode = errorCode;
    this.details = details;
    this.statusCode = statusCode;
    Object.setPrototypeOf(this, NeuroGraphError.prototype);
  }

  static fromResponse(response: ErrorResponse, statusCode?: number): NeuroGraphError {
    const { error_code, message, details } = response;

    // Map to specific error types based on error code
    switch (error_code) {
      case 'AUTHENTICATION_FAILED':
      case 'INVALID_TOKEN':
      case 'TOKEN_EXPIRED':
        return new AuthenticationError(message, error_code, details, statusCode);

      case 'INSUFFICIENT_PERMISSIONS':
      case 'FORBIDDEN':
        return new AuthorizationError(message, error_code, details, statusCode);

      case 'NOT_FOUND':
      case 'TOKEN_NOT_FOUND':
      case 'API_KEY_NOT_FOUND':
        return new NotFoundError(message, error_code, details, statusCode);

      case 'VALIDATION_ERROR':
      case 'INVALID_INPUT':
        return new ValidationError(message, error_code, details, statusCode);

      case 'RATE_LIMIT_EXCEEDED':
        return new RateLimitError(message, error_code, details, statusCode);

      case 'CONFLICT':
      case 'ALREADY_EXISTS':
        return new ConflictError(message, error_code, details, statusCode);

      case 'INTERNAL_SERVER_ERROR':
      case 'SERVICE_UNAVAILABLE':
        return new ServerError(message, error_code, details, statusCode);

      default:
        return new NeuroGraphError(message, error_code, details, statusCode);
    }
  }
}

/**
 * Authentication error (401).
 */
export class AuthenticationError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 401);
    this.name = 'AuthenticationError';
    Object.setPrototypeOf(this, AuthenticationError.prototype);
  }
}

/**
 * Authorization error (403).
 */
export class AuthorizationError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 403);
    this.name = 'AuthorizationError';
    Object.setPrototypeOf(this, AuthorizationError.prototype);
  }
}

/**
 * Not found error (404).
 */
export class NotFoundError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 404);
    this.name = 'NotFoundError';
    Object.setPrototypeOf(this, NotFoundError.prototype);
  }
}

/**
 * Validation error (422).
 */
export class ValidationError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 422);
    this.name = 'ValidationError';
    Object.setPrototypeOf(this, ValidationError.prototype);
  }
}

/**
 * Rate limit error (429).
 */
export class RateLimitError extends NeuroGraphError {
  public readonly retryAfter?: number;

  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 429);
    this.name = 'RateLimitError';
    this.retryAfter = details?.retry_after;
    Object.setPrototypeOf(this, RateLimitError.prototype);
  }
}

/**
 * Conflict error (409).
 */
export class ConflictError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 409);
    this.name = 'ConflictError';
    Object.setPrototypeOf(this, ConflictError.prototype);
  }
}

/**
 * Server error (500, 502, 503, 504).
 */
export class ServerError extends NeuroGraphError {
  constructor(message: string, errorCode: string, details?: any, statusCode?: number) {
    super(message, errorCode, details, statusCode || 500);
    this.name = 'ServerError';
    Object.setPrototypeOf(this, ServerError.prototype);
  }
}

/**
 * Network error (connection failures, timeouts).
 */
export class NetworkError extends NeuroGraphError {
  constructor(message: string, details?: any) {
    super(message, 'NETWORK_ERROR', details);
    this.name = 'NetworkError';
    Object.setPrototypeOf(this, NetworkError.prototype);
  }
}
