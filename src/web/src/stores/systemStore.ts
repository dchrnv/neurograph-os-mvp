/**
 * System store - metrics, status, activity
 */

import { create } from 'zustand';
import type { SystemStatus, SystemMetrics, ActivityEvent } from '../types/api';

interface SystemStore {
  // Status
  status: SystemStatus | null;
  setStatus: (status: SystemStatus) => void;

  // Metrics
  metrics: SystemMetrics | null;
  metricsHistory: SystemMetrics[];
  setMetrics: (metrics: SystemMetrics) => void;
  addMetricsHistory: (metrics: SystemMetrics) => void;

  // Activity
  activities: ActivityEvent[];
  addActivity: (activity: ActivityEvent) => void;

  // Loading states
  loading: boolean;
  setLoading: (loading: boolean) => void;

  // Error
  error: string | null;
  setError: (error: string | null) => void;
}

export const useSystemStore = create<SystemStore>((set) => ({
  // Status
  status: null,
  setStatus: (status) => set({ status }),

  // Metrics
  metrics: null,
  metricsHistory: [],
  setMetrics: (metrics) => set({ metrics }),
  addMetricsHistory: (metrics) =>
    set((state) => ({
      metricsHistory: [...state.metricsHistory.slice(-99), metrics],
    })),

  // Activity
  activities: [],
  addActivity: (activity) =>
    set((state) => ({
      activities: [activity, ...state.activities.slice(0, 49)],
    })),

  // Loading
  loading: false,
  setLoading: (loading) => set({ loading }),

  // Error
  error: null,
  setError: (error) => set({ error }),
}));
