/**
 * NeuroGraph WebSocket Client
 *
 * TypeScript/JavaScript client for NeuroGraph WebSocket API
 * Version: v0.60.0
 *
 * @license AGPLv3
 * @copyright 2024-2025 Chernov Denys
 */

export interface NeurographWSClientOptions {
  /** WebSocket URL (default: ws://localhost:8000/ws) */
  url?: string;
  /** JWT authentication token */
  token?: string;
  /** Auto-reconnect on disconnect (default: true) */
  autoReconnect?: boolean;
  /** Reconnect delay in milliseconds (default: 3000) */
  reconnectDelay?: number;
  /** Maximum reconnection attempts (default: 10) */
  maxReconnectAttempts?: number;
  /** Enable debug logging (default: false) */
  debug?: boolean;
}

export interface ChannelEvent {
  channel: string;
  timestamp: string;
  event_type: string;
  data: Record<string, any>;
}

export type EventHandler = (data: any) => void;

export interface ConnectionInfo {
  client_id: string;
  user_id: string | null;
  timestamp: string;
}

/**
 * WebSocket client for NeuroGraph real-time events
 *
 * @example
 * ```typescript
 * const client = new NeurographWSClient({
 *   url: "ws://localhost:8000/ws",
 *   token: "your-jwt-token",
 *   autoReconnect: true
 * });
 *
 * await client.connect();
 *
 * client.subscribe("metrics", (data) => {
 *   console.log("Metrics:", data);
 * });
 *
 * client.on("connected", (info) => {
 *   console.log("Connected:", info.client_id);
 * });
 * ```
 */
export class NeurographWSClient {
  private ws: WebSocket | null = null;
  private options: Required<NeurographWSClientOptions>;
  private subscriptions: Map<string, Set<EventHandler>> = new Map();
  private eventHandlers: Map<string, Set<EventHandler>> = new Map();
  private reconnectAttempts = 0;
  private reconnectTimer: NodeJS.Timeout | null = null;
  private connectionInfo: ConnectionInfo | null = null;
  private connected = false;

  constructor(options: NeurographWSClientOptions = {}) {
    this.options = {
      url: options.url || "ws://localhost:8000/ws",
      token: options.token || "",
      autoReconnect: options.autoReconnect !== false,
      reconnectDelay: options.reconnectDelay || 3000,
      maxReconnectAttempts: options.maxReconnectAttempts || 10,
      debug: options.debug || false,
    };
  }

  /**
   * Connect to WebSocket server
   */
  async connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      try {
        // Build URL with token if provided
        let url = this.options.url;
        if (this.options.token) {
          url += `?token=${encodeURIComponent(this.options.token)}`;
        }

        this.log("Connecting to:", url);

        this.ws = new WebSocket(url);

        this.ws.onopen = () => {
          this.log("WebSocket connection opened");
        };

        this.ws.onmessage = (event) => {
          this.handleMessage(event.data);
        };

        this.ws.onerror = (error) => {
          this.log("WebSocket error:", error);
          this.emit("error", error);
          reject(error);
        };

        this.ws.onclose = (event) => {
          this.log("WebSocket connection closed:", event.code, event.reason);
          this.connected = false;
          this.connectionInfo = null;
          this.emit("disconnected", { code: event.code, reason: event.reason });

          if (this.options.autoReconnect && this.reconnectAttempts < this.options.maxReconnectAttempts) {
            this.scheduleReconnect();
          }
        };

        // Wait for connection confirmation
        const messageHandler = (event: MessageEvent) => {
          const message = JSON.parse(event.data);
          if (message.type === "connected") {
            this.connected = true;
            this.connectionInfo = message;
            this.reconnectAttempts = 0;
            this.log("Connected with client_id:", message.client_id);
            this.emit("connected", message);
            this.ws!.removeEventListener("message", messageHandler);
            resolve();
          }
        };

        this.ws.addEventListener("message", messageHandler);

      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Disconnect from WebSocket server
   */
  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }

    this.connected = false;
    this.connectionInfo = null;
  }

  /**
   * Subscribe to a channel
   *
   * @param channel - Channel name (e.g., "metrics", "signals")
   * @param handler - Event handler function
   */
  subscribe(channel: string, handler: EventHandler): void {
    // Add handler to local subscriptions
    if (!this.subscriptions.has(channel)) {
      this.subscriptions.set(channel, new Set());
    }
    this.subscriptions.get(channel)!.add(handler);

    // Send subscription request to server
    this.send({
      type: "subscribe",
      channels: [channel],
    });

    this.log("Subscribed to channel:", channel);
  }

  /**
   * Subscribe to multiple channels at once
   *
   * @param channels - Array of channel names
   * @param handler - Event handler function (receives events from all channels)
   */
  subscribeMultiple(channels: string[], handler: EventHandler): void {
    for (const channel of channels) {
      if (!this.subscriptions.has(channel)) {
        this.subscriptions.set(channel, new Set());
      }
      this.subscriptions.get(channel)!.add(handler);
    }

    // Send subscription request to server
    this.send({
      type: "subscribe",
      channels,
    });

    this.log("Subscribed to channels:", channels);
  }

  /**
   * Unsubscribe from a channel
   *
   * @param channel - Channel name
   * @param handler - Optional specific handler to remove
   */
  unsubscribe(channel: string, handler?: EventHandler): void {
    if (handler) {
      // Remove specific handler
      const handlers = this.subscriptions.get(channel);
      if (handlers) {
        handlers.delete(handler);
        if (handlers.size === 0) {
          this.subscriptions.delete(channel);
        }
      }
    } else {
      // Remove all handlers for channel
      this.subscriptions.delete(channel);
    }

    // Send unsubscription request to server
    this.send({
      type: "unsubscribe",
      channels: [channel],
    });

    this.log("Unsubscribed from channel:", channel);
  }

  /**
   * Register an event handler for internal events
   *
   * Events:
   * - "connected" - Connection established
   * - "disconnected" - Connection closed
   * - "error" - Error occurred
   * - "pong" - Pong response received
   *
   * @param event - Event name
   * @param handler - Event handler function
   */
  on(event: string, handler: EventHandler): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  /**
   * Remove an event handler
   *
   * @param event - Event name
   * @param handler - Event handler function
   */
  off(event: string, handler: EventHandler): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      handlers.delete(handler);
    }
  }

  /**
   * Send a ping to the server
   */
  ping(): void {
    this.send({ type: "ping" });
  }

  /**
   * Get connection information
   */
  getConnectionInfo(): ConnectionInfo | null {
    return this.connectionInfo;
  }

  /**
   * Check if client is connected
   */
  isConnected(): boolean {
    return this.connected;
  }

  /**
   * Get current subscriptions
   */
  async getSubscriptions(): Promise<string[]> {
    return new Promise((resolve) => {
      // Send request
      this.send({ type: "get_subscriptions" });

      // Wait for response
      const handler = (event: MessageEvent) => {
        const message = JSON.parse(event.data);
        if (message.type === "subscriptions") {
          this.ws!.removeEventListener("message", handler);
          resolve(message.channels);
        }
      };

      this.ws!.addEventListener("message", handler);
    });
  }

  // Private methods

  private send(data: any): void {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      this.log("Cannot send message: WebSocket not connected");
      return;
    }

    this.ws.send(JSON.stringify(data));
  }

  private handleMessage(data: string): void {
    try {
      const message = JSON.parse(data);

      // Handle system messages
      if (message.type === "pong") {
        this.emit("pong", message);
        return;
      }

      if (message.type === "error") {
        this.log("Server error:", message.message);
        this.emit("error", message);
        return;
      }

      if (message.type === "subscribed") {
        this.log("Subscription confirmed:", message.channels);
        return;
      }

      if (message.type === "unsubscribed") {
        this.log("Unsubscription confirmed:", message.channels);
        return;
      }

      // Handle channel events
      if (message.channel) {
        const handlers = this.subscriptions.get(message.channel);
        if (handlers) {
          for (const handler of handlers) {
            try {
              handler(message);
            } catch (error) {
              this.log("Error in event handler:", error);
            }
          }
        }
      }
    } catch (error) {
      this.log("Error parsing message:", error);
    }
  }

  private emit(event: string, data: any): void {
    const handlers = this.eventHandlers.get(event);
    if (handlers) {
      for (const handler of handlers) {
        try {
          handler(data);
        } catch (error) {
          this.log("Error in event handler:", error);
        }
      }
    }
  }

  private scheduleReconnect(): void {
    this.reconnectAttempts++;
    this.log(`Reconnecting in ${this.options.reconnectDelay}ms (attempt ${this.reconnectAttempts}/${this.options.maxReconnectAttempts})`);

    this.reconnectTimer = setTimeout(() => {
      this.log("Attempting to reconnect...");
      this.connect().catch((error) => {
        this.log("Reconnection failed:", error);
      });
    }, this.options.reconnectDelay);
  }

  private log(...args: any[]): void {
    if (this.options.debug) {
      console.log("[NeurographWS]", ...args);
    }
  }
}

// Export types
export { Channel } from "./types";

// Default export
export default NeurographWSClient;
