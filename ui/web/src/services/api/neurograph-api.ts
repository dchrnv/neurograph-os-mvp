import { ApiResponse, Token, GraphNode, Connection } from './types';

export class NeuroGraphApi {
  private baseUrl: string;
  private token: string | null = null;

  constructor(baseUrl: string = import.meta.env.VITE_API_URL || 'http://localhost:8000') {
    this.baseUrl = baseUrl;
  }

  setAuthToken(token: string): void {
    this.token = token;
  }

  private async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
    const url = `${this.baseUrl}/api/v1${endpoint}`;
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (this.token) {
      headers['Authorization'] = `Bearer ${this.token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  // Token methods
  async createToken(tokenData: Partial<Token>): Promise<ApiResponse<Token>> {
    return this.request<Token>('/tokens', {
      method: 'POST',
      body: JSON.stringify(tokenData),
    });
  }

  async getToken(id: string): Promise<ApiResponse<Token>> {
    return this.request<Token>(`/tokens/${id}`);
  }

  async searchTokens(query: string): Promise<ApiResponse<Token[]>> {
    return this.request<Token[]>(`/tokens/search?q=${encodeURIComponent(query)}`);
  }

  // Graph methods
  async getGraph(): Promise<ApiResponse<GraphNode[]>> {
    return this.request<GraphNode[]>('/graph');
  }

  async createConnection(from: string, to: string, weight: number): Promise<ApiResponse<Connection>> {
    return this.request<Connection>('/connections', {
      method: 'POST',
      body: JSON.stringify({ from, to, weight }),
    });
  }

  // System methods
  async healthCheck(): Promise<ApiResponse<{ status: string }>> {
    return this.request<{ status: string }>('/health');
  }
}

export const neurographApi = new NeuroGraphApi();