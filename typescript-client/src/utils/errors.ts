/**
 * Error handling utilities.
 */

import axios from 'axios';
import { NeuroGraphError, NetworkError } from '../errors';
import type { ErrorResponse } from '../models';

/**
 * Handle API errors and convert to appropriate NeuroGraphError.
 */
export function handleApiError(error: unknown): never {
  if (axios.isAxiosError(error)) {
    if (error.response) {
      // Server responded with error
      const errorData = error.response.data as ErrorResponse;
      throw NeuroGraphError.fromResponse(errorData, error.response.status);
    } else if (error.request) {
      // Request made but no response received
      throw new NetworkError('No response from server', { originalError: error.message });
    } else {
      // Error setting up request
      throw new NetworkError('Request setup failed', { originalError: error.message });
    }
  }

  // Unknown error type
  throw error;
}
