/**
 * NeuroGraph OS WebSocket Client
 * TypeScript client library for real-time communication
 */

export interface WebSocketMessage {
  id: string;
  type: string;
  payload: Record<string, any>;
  timestamp: number;
  sender_id?: string;
}

export interface ConnectionOptions {
  url: string;
  clientId?: string;
  token?: string;
  reconnect?: boolean;
  reconnectInterval?: number;
  heartbeatInterval?: number;
  debug?: boolean;
}

export type MessageHandler = (message: WebSocketMessage) => void;
export type EventHandler = () => void;

export class NeuroGraphWebSocket {
  private ws: WebSocket | null = null;
  private options: Required<ConnectionOptions>;
  private messageHandlers: Map<string, Set<MessageHandler>> = new Map();
  private eventHandlers: Map<string, Set<EventHandler>> = new Map();
  private reconnectTimeout: number | null = null;
  private heartbeatInterval: number | null = null;
  private messageQueue: WebSocketMessage[] = [];
  private connectionId: string | null = null;
  
  constructor(options: ConnectionOptions) {
    this.options = {
      url: options.url,
      clientId: options.clientId || this.generateClientId(),
      token: options.token || '',
      reconnect: options.reconnect !== false,
      reconnectInterval: options.reconnectInterval || 5000,
      heartbeatInterval: options.heartbeatInterval || 30000,
      debug: options.debug || false
    };
  }

  /**
   * Connect to WebSocket server
   */
  public connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        const url = new URL(this.options.url);
        url.searchParams.set('client_id', this.options.clientId);
        
        this.log('Connecting to', url.toString());
        
        this.ws = new WebSocket(url.toString());
        
        this.ws.onopen = () => {
          this.log('Connected');
          this.emit('connected');
          this.startHeartbeat();
          this.flushMessageQueue();
          resolve();
        };
        
        this.ws.onmessage = (event) => {
          this.handleMessage(event.data);
        };
        
        this.ws.onerror = (error) => {
          this.log('Error', error);
          this.emit('error', error);
          reject(error);
        };
        
        this.ws.onclose = () => {
          this.log('Disconnected');
          this.emit('disconnected');
          this.stopHeartbeat();
          
          if (this.options.reconnect) {
            this.scheduleReconnect();
          }
        };
        
      } catch (error) {
        this.log('Connection error', error);
        reject(error);
      }
    });
  }

  /**
   * Disconnect from WebSocket server
   */
  public disconnect(): void {
    this.options.reconnect = false;
    
    if (this.reconnectTimeout) {
      clearTimeout(this.reconnectTimeout);
      this.reconnectTimeout = null;
    }
    
    this.stopHeartbeat();
    
    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  /**
   * Send a message
   */
  public send(type: string, payload: Record<string, any> = {}): void {
    const message: WebSocketMessage = {
      id: this.generateMessageId(),
      type,
      payload,
      timestamp: Date.now()
    };
    
    if (this.isConnected()) {
      this.sendMessage(message);
    } else {
      this.log('Not connected, queueing message', message);
      this.messageQueue.push(message);
    }
  }

  /**
   * Subscribe to a topic
   */
  public subscribe(topic: string): void {
    this.send('subscribe', { topic });
  }

  /**
   * Unsubscribe from a topic
   */
  public unsubscribe(topic: string): void {
    this.send('unsubscribe', { topic });
  }

  /**
   * Register message handler for specific message type
   */
  public on(messageType: string, handler: MessageHandler): void {
    if (!this.messageHandlers.has(messageType)) {
      this.messageHandlers.set(messageType, new Set());
    }
    this.messageHandlers.get(messageType)!.add(handler);
  }

  /**
   * Unregister message handler
   */
  public off(messageType: string, handler: MessageHandler): void {
    const handlers = this.messageHandlers.get(messageType);
    if (handlers) {
      handlers.delete(handler);
    }
  }

  /**
   * Register event handler (connected, disconnected, error)
   */
  public onEvent(event: string, handler: EventHandler): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  /**
   * Check if connected
   */
  public isConnected(): boolean {
    return this.ws !== null && this.ws.readyState === WebSocket.OPEN;
  }

  /**
   * Get connection ID
   */
  public getConnectionId(): string | null {
    return this.connectionId;
  }

  // Token API methods

  /**
   * Create a token
   */
  public createToken(data: {
    type?: string;
    coord_x?: number[];
    coord_y?: number[];
    coord_z?: number[];
    weight?: number;
    flags?: number;
    metadata?: Record<string, any>;
  }): void {
    this.send('token.create', data);
  }

  /**
   * Get token by ID
   */
  public getToken(tokenId: string): void {
    this.send('token.get', { token_id: tokenId });
  }

  /**
   * List tokens
   */
  public listTokens(limit: number = 10, offset: number = 0): void {
    this.send('token.list', { limit, offset });
  }

  /**
   * Create graph connection
   */
  public connectTokens(sourceId: string, targetId: string, options: {
    type?: string;
    weight?: number;
    bidirectional?: boolean;
  } = {}): void {
    this.send('graph.connect', {
      source_id: sourceId,
      target_id: targetId,
      ...options
    });
  }

  /**
   * Get token neighbors
   */
  public getNeighbors(tokenId: string, direction: 'incoming' | 'outgoing' | 'both' = 'both'): void {
    this.send('graph.neighbors', { token_id: tokenId, direction });
  }

  // Private methods

  private handleMessage(data: string): void {
    try {
      const message: WebSocketMessage = JSON.parse(data);
      this.log('Received', message.type, message);
      
      // Handle connection confirmation
      if (message.type === 'connection.established') {
        this.connectionId = message.payload.connection_id;
        this.log('Connection ID:', this.connectionId);
      }
      
      // Call registered handlers
      const handlers = this.messageHandlers.get(message.type);
      if (handlers) {
        handlers.forEach(handler => {
          try {
            handler(message);
          } catch (error) {
            this.log('Handler error', error);
          }
        });
      }
      
      // Also call wildcard handlers
      const wildcardHandlers = this.messageHandlers.get('*');
      if (wildcardHandlers) {
        wildcardHandlers.forEach(handler => handler(message));
      }
      
    } catch (error) {
      this.log('Failed to parse message', error);
    }
  }

  private sendMessage(message: WebSocketMessage): void {
    if (this.ws && this.isConnected()) {
      this.ws.send(JSON.stringify(message));
      this.log('Sent', message.type, message);
    }
  }

  private startHeartbeat(): void {
    this.stopHeartbeat();
    
    this.heartbeatInterval = window.setInterval(() => {
      if (this.isConnected()) {
        this.send('ping', {});
      }
    }, this.options.heartbeatInterval);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatInterval !== null) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimeout) {
      return;
    }
    
    this.log('Scheduling reconnect in', this.options.reconnectInterval, 'ms');
    
    this.reconnectTimeout = window.setTimeout(() => {
      this.reconnectTimeout = null;
      this.log('Attempting reconnect...');
      this.connect().catch((error) => {
        this.log('Reconnect failed', error);
      });
    }, this.options.reconnectInterval);
  }

  private flushMessageQueue(): void {
    if (this.messageQueue.length > 0) {
      this.log('Flushing message queue', this.messageQueue.length, 'messages');
      
      this.messageQueue.forEach(message => {
        this.sendMessage(message);
      });
      
      this.messageQueue = [];
    }
  }

  private emit(event: string, data?: any): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.forEach(handler => {
        try {
          handler();
        } catch (error) {
          this.log('Event handler error', error);
        }
      });
    }
  }

  private generateClientId(): string {
    return `client_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private generateMessageId(): string {
    return `msg_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
  }

  private log(...args: any[]): void {
    if (this.options.debug) {
      console.log('[NeuroGraphWS]', ...args);
    }
  }
}

// Export convenience function
export function createWebSocket(options: ConnectionOptions): NeuroGraphWebSocket {
  return new NeuroGraphWebSocket(options);
}