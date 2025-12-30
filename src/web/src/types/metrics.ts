/**
 * Metrics types for charts and monitoring
 */

export interface MetricPoint {
  timestamp: number;
  value: number;
}

export interface ChartData {
  name: string;
  data: MetricPoint[];
}

export interface CDNAConfig {
  profile: 'balanced' | 'explorer' | 'focused' | 'creative';
  scales: {
    physical: number;
    sensory: number;
    motor: number;
    emotional: number;
    cognitive: number;
    social: number;
    temporal: number;
    abstract: number;
  };
}
