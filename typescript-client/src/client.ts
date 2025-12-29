/**
 * Main NeuroGraph client.
 */

import axios, { type AxiosInstance, type CreateAxiosDefaults } from 'axios';
import { AuthManager, type AuthConfig } from './auth/AuthManager';
import { TokensResource } from './resources/TokensResource';
import { APIKeysResource } from './resources/APIKeysResource';
import { HealthResource } from './resources/HealthResource';

export interface ClientConfig {
  baseUrl: string;
  username?: string;
  password?: string;
  apiKey?: string;
  timeout?: number;
  headers?: Record<string, string>;
}

/**
 * NeuroGraph API client.
 *
 * @example
 * ```typescript
 * // JWT authentication
 * const client = new NeuroGraphClient({
 *   baseUrl: 'http://localhost:8000',
 *   username: 'developer',
 *   password: 'developer123'
 * });
 *
 * // API key authentication
 * const client = new NeuroGraphClient({
 *   baseUrl: 'http://localhost:8000',
 *   apiKey: 'ng_your_api_key_here'
 * });
 *
 * // Create a token
 * const token = await client.tokens.create({ text: 'hello world' });
 *
 * // Query similar tokens
 * const results = await client.tokens.query({
 *   queryVector: token.embedding,
 *   topK: 10
 * });
 * ```
 */
export class NeuroGraphClient {
  private httpClient: AxiosInstance;
  private authManager: AuthManager;

  public readonly tokens: TokensResource;
  public readonly apiKeys: APIKeysResource;
  public readonly health: HealthResource;

  constructor(config: ClientConfig) {
    // Validate config
    if (!config.apiKey && (!config.username || !config.password)) {
      throw new Error('Either apiKey or username/password must be provided');
    }

    // Create HTTP client
    const axiosConfig: CreateAxiosDefaults = {
      baseURL: config.baseUrl,
      timeout: config.timeout ?? 30000,
      headers: {
        'Content-Type': 'application/json',
        ...config.headers,
      },
    };

    this.httpClient = axios.create(axiosConfig);

    // Create auth manager
    const authConfig: AuthConfig = {
      baseUrl: config.baseUrl,
      username: config.username,
      password: config.password,
      apiKey: config.apiKey,
    };
    this.authManager = new AuthManager(authConfig, this.httpClient);

    // Setup request interceptor for authentication
    this.httpClient.interceptors.request.use(async (config) => {
      const authHeader = await this.authManager.getAuthHeader();
      config.headers.Authorization = authHeader;
      return config;
    });

    // Setup response interceptor for auth errors
    this.httpClient.interceptors.response.use(
      (response) => response,
      async (error) => {
        // If we get 401 and using JWT, try to refresh token once
        if (
          error.response?.status === 401 &&
          !config.apiKey &&
          !error.config._retry
        ) {
          error.config._retry = true;
          this.authManager.invalidateToken();
          const authHeader = await this.authManager.getAuthHeader();
          error.config.headers.Authorization = authHeader;
          return this.httpClient.request(error.config);
        }
        return Promise.reject(error);
      }
    );

    // Initialize resource clients
    this.tokens = new TokensResource(this.httpClient);
    this.apiKeys = new APIKeysResource(this.httpClient);
    this.health = new HealthResource(this.httpClient);
  }

  /**
   * Get the underlying axios instance for advanced usage.
   */
  getHttpClient(): AxiosInstance {
    return this.httpClient;
  }
}
