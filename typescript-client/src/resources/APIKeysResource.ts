/**
 * API Keys resource - operations on API keys.
 */

import type { AxiosInstance } from 'axios';
import type { APIKey, APIKeyCreate } from '../models';
import { handleApiError } from '../utils/errors';

export class APIKeysResource {
  constructor(private httpClient: AxiosInstance) {}

  /**
   * Create a new API key.
   * Returns the full API key - save it immediately as it won't be shown again.
   */
  async create(data: APIKeyCreate): Promise<APIKey> {
    try {
      const response = await this.httpClient.post<APIKey>('/auth/api-keys', data);
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * List all API keys for the current user.
   */
  async list(): Promise<APIKey[]> {
    try {
      const response = await this.httpClient.get<APIKey[]>('/auth/api-keys');
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Get API key details by ID.
   */
  async get(keyId: string): Promise<APIKey> {
    try {
      const response = await this.httpClient.get<APIKey>(`/auth/api-keys/${keyId}`);
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Revoke an API key (makes it inactive).
   */
  async revoke(keyId: string): Promise<void> {
    try {
      await this.httpClient.post(`/auth/api-keys/${keyId}/revoke`);
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Delete an API key permanently.
   */
  async delete(keyId: string): Promise<void> {
    try {
      await this.httpClient.delete(`/auth/api-keys/${keyId}`);
    } catch (error) {
      throw handleApiError(error);
    }
  }
}
