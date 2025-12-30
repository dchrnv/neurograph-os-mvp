/**
 * Module types for NeuroGraph Dashboard
 */

export type ModuleStatus = 'running' | 'starting' | 'stopped' | 'error' | 'restarting';

export interface Module {
  id: string;
  name: string;
  version: string;
  status: ModuleStatus;
  metrics?: Record<string, number | string>;
  config?: Record<string, any>;
  logs?: string[];
  restarts?: number;
}

export interface ModuleAction {
  moduleId: string;
  action: 'start' | 'stop' | 'restart';
}
