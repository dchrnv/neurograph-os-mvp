/**
 * API service using axios
 */

import axios, { AxiosInstance } from 'axios';
import { API_BASE_URL } from '../utils/constants';
import type { SystemStatus, SystemMetrics, ApiResponse } from '../types/api';
import type { Module } from '../types/modules';

class ApiService {
  private client: AxiosInstance;

  constructor() {
    this.client = axios.create({
      baseURL: API_BASE_URL,
      timeout: 10000,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  // Health & Metrics
  async getHealth(): Promise<SystemStatus> {
    const { data } = await this.client.get<SystemStatus>('/health');
    return data;
  }

  async getMetrics(): Promise<SystemMetrics> {
    const { data } = await this.client.get<SystemMetrics>('/metrics');
    return data;
  }

  // Modules
  async getModules(): Promise<Module[]> {
    const { data } = await this.client.get<ApiResponse<Module[]>>('/modules');
    return data.data || [];
  }

  async getModule(id: string): Promise<Module> {
    const { data } = await this.client.get<ApiResponse<Module>>(`/modules/${id}`);
    return data.data!;
  }

  async startModule(id: string): Promise<void> {
    await this.client.post(`/modules/${id}/start`);
  }

  async stopModule(id: string): Promise<void> {
    await this.client.post(`/modules/${id}/stop`);
  }

  async restartModule(id: string): Promise<void> {
    await this.client.post(`/modules/${id}/restart`);
  }

  // Config
  async getConfig(): Promise<Record<string, any>> {
    const { data } = await this.client.get('/config');
    return data;
  }

  async updateConfig(section: string, config: Record<string, any>): Promise<void> {
    await this.client.put(`/config/${section}`, config);
  }

  // Chat
  async sendChatMessage(message: string): Promise<any> {
    const { data } = await this.client.post('/chat/message', { message });
    return data;
  }

  // CDNA
  async getCDNAConfig(): Promise<any> {
    const { data} = await this.client.get('/cdna/config');
    return data;
  }

  async updateCDNAScales(scales: Record<string, number>): Promise<void> {
    await this.client.put('/cdna/scales', { scales });
  }

  // Admin operations
  async exportData(): Promise<any> {
    const { data } = await this.client.post('/admin/export');
    return data;
  }

  async importData(importData: any): Promise<void> {
    await this.client.post('/admin/import', importData);
  }

  async createBackup(): Promise<any> {
    const { data } = await this.client.post('/admin/backup');
    return data;
  }

  async clearTokens(): Promise<void> {
    await this.client.delete('/admin/tokens');
  }

  async resetSystem(): Promise<void> {
    await this.client.post('/admin/reset');
  }
}

export const api = new ApiService();
