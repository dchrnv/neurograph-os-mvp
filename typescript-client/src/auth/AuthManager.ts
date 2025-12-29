/**
 * Authentication manager for JWT and API key authentication.
 */

import axios, { type AxiosInstance } from 'axios';
import type { JWTToken } from '../models';
import { AuthenticationError } from '../errors';

export interface AuthConfig {
  baseUrl: string;
  username?: string;
  password?: string;
  apiKey?: string;
}

/**
 * Manages authentication for the NeuroGraph client.
 * Handles JWT token refresh and API key authentication.
 */
export class AuthManager {
  private baseUrl: string;
  private username?: string;
  private password?: string;
  private apiKey?: string;
  private accessToken?: string;
  private tokenExpiresAt?: Date;
  private httpClient: AxiosInstance;

  constructor(config: AuthConfig, httpClient: AxiosInstance) {
    this.baseUrl = config.baseUrl;
    this.username = config.username;
    this.password = config.password;
    this.apiKey = config.apiKey;
    this.httpClient = httpClient;

    if (!this.apiKey && (!this.username || !this.password)) {
      throw new Error('Either API key or username/password must be provided');
    }
  }

  /**
   * Get authorization header value.
   */
  async getAuthHeader(): Promise<string> {
    if (this.apiKey) {
      return `Bearer ${this.apiKey}`;
    }

    // JWT authentication - check if token needs refresh
    if (!this.accessToken || this.isTokenExpired()) {
      await this.refreshToken();
    }

    return `Bearer ${this.accessToken}`;
  }

  /**
   * Check if current JWT token is expired or about to expire.
   */
  private isTokenExpired(): boolean {
    if (!this.tokenExpiresAt) {
      return true;
    }

    // Refresh if token expires in less than 60 seconds
    const expiresIn = this.tokenExpiresAt.getTime() - Date.now();
    return expiresIn < 60000;
  }

  /**
   * Refresh JWT access token.
   */
  private async refreshToken(): Promise<void> {
    if (!this.username || !this.password) {
      throw new AuthenticationError(
        'Cannot refresh token: username/password not provided',
        'MISSING_CREDENTIALS'
      );
    }

    try {
      const formData = new URLSearchParams();
      formData.append('username', this.username);
      formData.append('password', this.password);

      const response = await this.httpClient.post<JWTToken>(
        `${this.baseUrl}/auth/token`,
        formData,
        {
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
          },
        }
      );

      const { access_token, expires_in } = response.data;
      this.accessToken = access_token;
      this.tokenExpiresAt = new Date(Date.now() + expires_in * 1000);
    } catch (error) {
      if (axios.isAxiosError(error) && error.response?.status === 401) {
        throw new AuthenticationError(
          'Invalid username or password',
          'AUTHENTICATION_FAILED',
          error.response.data
        );
      }
      throw error;
    }
  }

  /**
   * Force token refresh on next request.
   */
  invalidateToken(): void {
    this.accessToken = undefined;
    this.tokenExpiresAt = undefined;
  }
}
