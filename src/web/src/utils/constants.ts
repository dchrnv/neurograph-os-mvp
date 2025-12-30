/**
 * Application constants
 */

export const API_BASE_URL = import.meta.env.VITE_API_URL || '/api/v1';
export const WS_URL = import.meta.env.VITE_WS_URL || 'ws://localhost:8000/ws';

export const ROUTES = {
  DASHBOARD: '/',
  MODULES: '/modules',
  CONFIG: '/config',
  BOOTSTRAP: '/bootstrap',
  CHAT: '/chat',
  TERMINAL: '/terminal',
  ADMIN: '/admin',
} as const;

export const WS_CHANNELS = {
  METRICS: 'metrics',
  ACTIVITY: 'activity',
  MODULES: 'modules',
  CHAT: 'chat',
  TERMINAL: 'terminal',
} as const;

export const MODULE_STATUS_COLORS = {
  running: '#52c41a',
  starting: '#faad14',
  stopped: '#d9d9d9',
  error: '#ff4d4f',
  restarting: '#1890ff',
} as const;

export const THEME_STORAGE_KEY = 'neurograph-theme';
export const LANGUAGE_STORAGE_KEY = 'neurograph-language';
