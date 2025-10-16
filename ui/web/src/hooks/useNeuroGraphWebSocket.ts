/**
 * React Hook for NeuroGraph WebSocket
 */

import { useEffect, useRef, useState, useCallback } from 'react';
import { NeuroGraphWebSocket, WebSocketMessage, ConnectionOptions } from '../services/websocket/neurograph-websocket';

export interface UseWebSocketOptions extends Omit<ConnectionOptions, 'url'> {
  autoConnect?: boolean;
}

export interface UseWebSocketReturn {
  ws: NeuroGraphWebSocket | null;
  isConnected: boolean;
  connectionId: string | null;
  send: (type: string, payload?: Record<string, any>) => void;
  subscribe: (topic: string) => void;
  unsubscribe: (topic: string) => void;
  connect: () => Promise<void>;
  disconnect: () => void;
  
  // Token API
  createToken: (data: any) => void;
  getToken: (tokenId: string) => void;
  listTokens: (limit?: number, offset?: number) => void;
  
  // Graph API
  connectTokens: (sourceId: string, targetId: string, options?: any) => void;
  getNeighbors: (tokenId: string, direction?: 'incoming' | 'outgoing' | 'both') => void;
}

export function useNeuroGraphWebSocket(
  url: string,
  options: UseWebSocketOptions = {}
): UseWebSocketReturn {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionId, setConnectionId] = useState<string | null>(null);
  const wsRef = useRef<NeuroGraphWebSocket | null>(null);
  
  const { autoConnect = true, ...wsOptions } = options;

  // Initialize WebSocket
  useEffect(() => {
    const ws = new NeuroGraphWebSocket({
      url,
      ...wsOptions
    });

    wsRef.current = ws;

    // Register event handlers
    ws.onEvent('connected', () => {
      setIsConnected(true);
      setConnectionId(ws.getConnectionId());
    });

    ws.onEvent('disconnected', () => {
      setIsConnected(false);
      setConnectionId(null);
    });

    // Auto-connect if enabled
    if (autoConnect) {
      ws.connect().catch(console.error);
    }

    // Cleanup on unmount
    return () => {
      ws.disconnect();
    };
  }, [url, autoConnect]);

  // Memoized API methods
  const send = useCallback((type: string, payload: Record<string, any> = {}) => {
    wsRef.current?.send(type, payload);
  }, []);

  const subscribe = useCallback((topic: string) => {
    wsRef.current?.subscribe(topic);
  }, []);

  const unsubscribe = useCallback((topic: string) => {
    wsRef.current?.unsubscribe(topic);
  }, []);

  const connect = useCallback(async () => {
    if (wsRef.current) {
      await wsRef.current.connect();
    }
  }, []);

  const disconnect = useCallback(() => {
    wsRef.current?.disconnect();
  }, []);

  const createToken = useCallback((data: any) => {
    wsRef.current?.createToken(data);
  }, []);

  const getToken = useCallback((tokenId: string) => {
    wsRef.current?.getToken(tokenId);
  }, []);

  const listTokens = useCallback((limit?: number, offset?: number) => {
    wsRef.current?.listTokens(limit, offset);
  }, []);

  const connectTokens = useCallback((sourceId: string, targetId: string, options?: any) => {
    wsRef.current?.connectTokens(sourceId, targetId, options);
  }, []);

  const getNeighbors = useCallback((tokenId: string, direction?: 'incoming' | 'outgoing' | 'both') => {
    wsRef.current?.getNeighbors(tokenId, direction);
  }, []);

  return {
    ws: wsRef.current,
    isConnected,
    connectionId,
    send,
    subscribe,
    unsubscribe,
    connect,
    disconnect,
    createToken,
    getToken,
    listTokens,
    connectTokens,
    getNeighbors
  };
}

/**
 * Hook for listening to specific message types
 */
export function useWebSocketMessage(
  ws: NeuroGraphWebSocket | null,
  messageType: string,
  handler: (message: WebSocketMessage) => void
) {
  useEffect(() => {
    if (!ws) return;

    ws.on(messageType, handler);

    return () => {
      ws.off(messageType, handler);
    };
  }, [ws, messageType, handler]);
}

/**
 * Hook for subscribing to topics
 */
export function useWebSocketSubscription(
  ws: NeuroGraphWebSocket | null,
  topic: string,
  enabled: boolean = true
) {
  useEffect(() => {
    if (!ws || !enabled) return;

    ws.subscribe(topic);

    return () => {
      ws.unsubscribe(topic);
    };
  }, [ws, topic, enabled]);
}