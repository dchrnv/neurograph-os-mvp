/**
 * Health resource - health checks and system status.
 */

import type { AxiosInstance } from 'axios';
import type { HealthCheck, SystemStatus } from '../models';
import { handleApiError } from '../utils/errors';

export class HealthResource {
  constructor(private httpClient: AxiosInstance) {}

  /**
   * Check API health.
   */
  async check(): Promise<HealthCheck> {
    try {
      const response = await this.httpClient.get<HealthCheck>('/health');
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }

  /**
   * Get detailed system status.
   */
  async status(): Promise<SystemStatus> {
    try {
      const response = await this.httpClient.get<SystemStatus>('/health/status');
      return response.data;
    } catch (error) {
      throw handleApiError(error);
    }
  }
}
