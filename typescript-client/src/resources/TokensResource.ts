/**
 * Tokens resource - operations on semantic tokens.
 */

import type { AxiosInstance } from 'axios';
import type { Token, TokenCreate, TokenUpdate, QueryResult } from '../models';
import { handleApiError } from '../utils/errors';

export class TokensResource {
  constructor(private httpClient: AxiosInstance) {}

  /**
   * Create a new token.
   */
  async create(data: TokenCreate): Promise<Token> {
    try {
      const response = await this.httpClient.post<Token>('/tokens', data);
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Get token by ID.
   */
  async get(tokenId: number): Promise<Token> {
    try {
      const response = await this.httpClient.get<Token>(`/tokens/${tokenId}`);
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * List tokens with pagination.
   */
  async list(options: { limit?: number; offset?: number } = {}): Promise<Token[]> {
    try {
      const params = {
        limit: options.limit ?? 100,
        offset: options.offset ?? 0,
      };
      const response = await this.httpClient.get<Token[]>('/tokens', { params });
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Update token.
   */
  async update(tokenId: number, data: TokenUpdate): Promise<Token> {
    try {
      const response = await this.httpClient.put<Token>(`/tokens/${tokenId}`, data);
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Delete token.
   */
  async delete(tokenId: number): Promise<void> {
    try {
      await this.httpClient.delete(`/tokens/${tokenId}`);
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Query similar tokens by vector.
   */
  async query(options: {
    queryVector: number[];
    topK?: number;
    threshold?: number;
  }): Promise<QueryResult[]> {
    try {
      const response = await this.httpClient.post<QueryResult[]>('/tokens/query', {
        query_vector: options.queryVector,
        top_k: options.topK ?? 10,
        threshold: options.threshold,
      });
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Query similar tokens by text (convenience method).
   */
  async queryByText(options: {
    text: string;
    topK?: number;
    threshold?: number;
  }): Promise<QueryResult[]> {
    try {
      // First create a temporary token to get embedding
      const token = await this.create({ text: options.text });

      // Query using the embedding
      const results = await this.query({
        queryVector: token.embedding,
        topK: options.topK,
        threshold: options.threshold,
      });

      // Clean up temporary token
      await this.delete(token.id);

      return results;
    } catch (error) {
      throw handleApiError(error);
    }
  }
}
