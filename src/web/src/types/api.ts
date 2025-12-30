/**
 * API types for NeuroGraph Dashboard
 */

export interface SystemStatus {
  status: 'running' | 'stopped' | 'error';
  uptime: number;
  version: string;
}

export interface SystemMetrics {
  tokens: number;
  connections: number;
  queries_per_hour: number;
  events_per_sec: number;
  avg_latency_us: number;
  fast_path_percent: number;
  cache_hit_percent: number;
  cpu_percent: number;
  memory_percent: number;
  disk_percent: number;
}

export interface ActivityEvent {
  time: string;
  event: 'query' | 'feedback' | 'module' | 'system';
  details: string;
  duration_ms?: number;
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
