/**
 * Module types for NeuroGraph Dashboard
 */

export type ModuleStatus = 'active' | 'disabled' | 'error';

export interface ModuleMetrics {
  operations: number;
  ops_per_sec: number;
  avg_latency_us: number;
  p95_latency_us: number;
  errors: number;
  custom?: Record<string, number>;
}

export interface Module {
  id: string;
  name: string;
  description: string;
  version: string;
  status: ModuleStatus;
  enabled: boolean;
  can_disable: boolean;
  configurable: boolean;
  disable_warning?: string;
  metrics: ModuleMetrics;
  config?: Record<string, any>;
  logs?: string[];
}

export interface ModuleAction {
  moduleId: string;
  action: 'enable' | 'disable';
}
