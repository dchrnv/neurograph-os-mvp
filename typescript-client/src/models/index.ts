/**
 * Type definitions for NeuroGraph API models.
 */

/**
 * Token model - represents a semantic token with embedding.
 */
export interface Token {
  id: number;
  text: string;
  embedding: number[];
  metadata: Record<string, any>;
  created_at: string;
  updated_at: string;
}

/**
 * Request to create a new token.
 */
export interface TokenCreate {
  text: string;
  metadata?: Record<string, any>;
}

/**
 * Request to update an existing token.
 */
export interface TokenUpdate {
  text?: string;
  metadata?: Record<string, any>;
}

/**
 * Query result with similarity score.
 */
export interface QueryResult {
  token: Token;
  similarity: number;
}

/**
 * API key model.
 */
export interface APIKey {
  key_id: string;
  name: string;
  scopes: string[];
  created_at: string;
  expires_at: string | null;
  last_used_at: string | null;
  is_active: boolean;
  api_key?: string; // Only returned on creation
}

/**
 * Request to create a new API key.
 */
export interface APIKeyCreate {
  name: string;
  scopes: string[];
  expires_in_days?: number;
}

/**
 * User model.
 */
export interface User {
  username: string;
  role: 'admin' | 'developer' | 'viewer';
  created_at: string;
}

/**
 * Health check response.
 */
export interface HealthCheck {
  status: 'healthy' | 'unhealthy';
  version: string;
  timestamp: string;
}

/**
 * System status response.
 */
export interface SystemStatus {
  status: 'healthy' | 'unhealthy';
  version: string;
  timestamp: string;
  uptime_seconds: number;
  tokens_count: number;
  api_keys_count: number;
}

/**
 * JWT token response.
 */
export interface JWTToken {
  access_token: string;
  token_type: string;
  expires_in: number;
}

/**
 * Error response from API.
 */
export interface ErrorResponse {
  error_code: string;
  message: string;
  details?: any;
}
