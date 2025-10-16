/**
 * Example React component using NeuroGraph WebSocket
 */

import React, { useState, useCallback } from 'react';
import { useNeuroGraphWebSocket, useWebSocketMessage } from '../hooks/useNeuroGraphWebSocket';
import { WebSocketMessage } from '../services/websocket/neurograph-websocket';

export const WebSocketExample: React.FC = () => {
  const [messages, setMessages] = useState<WebSocketMessage[]>([]);
  const [tokenId, setTokenId] = useState('');
  const [tokens, setTokens] = useState<any[]>([]);

  // Connect to WebSocket
  const {
    isConnected,
    connectionId,
    send,
    subscribe,
    createToken,
    getToken,
    listTokens,
    connectTokens,
    getNeighbors,
    ws
  } = useNeuroGraphWebSocket('ws://localhost:8000/ws', {
    debug: true,
    reconnect: true
  });

  // Listen for token created events
  useWebSocketMessage(ws, 'token.created', useCallback((message: WebSocketMessage) => {
    console.log('Token created:', message.payload);
    setMessages(prev => [...prev, message]);
    
    // Refresh token list
    listTokens();
  }, [listTokens]));

  // Listen for token data
  useWebSocketMessage(ws, 'token.data', useCallback((message: WebSocketMessage) => {
    console.log('Token data:', message.payload);
    setMessages(prev => [...prev, message]);
  }, []));

  // Listen for token list
  useWebSocketMessage(ws, 'token.list', useCallback((message: WebSocketMessage) => {
    console.log('Token list:', message.payload);
    setTokens(message.payload.tokens || []);
  }, []));

  // Listen for graph events
  useWebSocketMessage(ws, 'graph.connected', useCallback((message: WebSocketMessage) => {
    console.log('Graph connected:', message.payload);
    setMessages(prev => [...prev, message]);
  }, []));

  // Listen for neighbors
  useWebSocketMessage(ws, 'graph.neighbors', useCallback((message: WebSocketMessage) => {
    console.log('Neighbors:', message.payload);
    setMessages(prev => [...prev, message]);
  }, []));

  // Listen for errors
  useWebSocketMessage(ws, 'error', useCallback((message: WebSocketMessage) => {
    console.error('Error:', message.payload);
    setMessages(prev => [...prev, message]);
  }, []));

  const handleCreateToken = () => {
    createToken({
      type: 'demo',
      coord_x: [Math.random() * 10, 0, 0, 0, 0, 0, 0, 0],
      coord_y: [Math.random() * 10, 0, 0, 0, 0, 0, 0, 0],
      coord_z: [0, 0, 0, 0, 0, 0, 0, 0],
      weight: 1.0
    });
  };

  const handleGetToken = () => {
    if (tokenId) {
      getToken(tokenId);
    }
  };

  const handleSubscribeTokens = () => {
    subscribe('tokens');
  };

  const handleSubscribeGraph = () => {
    subscribe('graph');
  };

  return (
    <div style={{ padding: '20px', fontFamily: 'monospace' }}>
      <h1>NeuroGraph WebSocket Demo</h1>

      {/* Connection Status */}
      <div style={{ 
        padding: '10px', 
        marginBottom: '20px',
        backgroundColor: isConnected ? '#d4edda' : '#f8d7da',
        border: `1px solid ${isConnected ? '#c3e6cb' : '#f5c6cb'}`,
        borderRadius: '4px'
      }}>
        <strong>Status:</strong> {isConnected ? '🟢 Connected' : '🔴 Disconnected'}
        {connectionId && <span> | Connection ID: {connectionId}</span>}
      </div>

      {/* Controls */}
      <div style={{ marginBottom: '20px' }}>
        <h3>Token Operations</h3>
        
        <button 
          onClick={handleCreateToken}
          disabled={!isConnected}
          style={{ marginRight: '10px', padding: '8px 16px' }}
        >
          Create Random Token
        </button>

        <button 
          onClick={() => listTokens(10)}
          disabled={!isConnected}
          style={{ marginRight: '10px', padding: '8px 16px' }}
        >
          List Tokens
        </button>

        <div style={{ marginTop: '10px' }}>
          <input
            type="text"
            placeholder="Token ID"
            value={tokenId}
            onChange={(e) => setTokenId(e.target.value)}
            style={{ padding: '8px', marginRight: '10px', width: '300px' }}
          />
          <button 
            onClick={handleGetToken}
            disabled={!isConnected || !tokenId}
            style={{ padding: '8px 16px' }}
          >
            Get Token
          </button>
        </div>
      </div>

      <div style={{ marginBottom: '20px' }}>
        <h3>Subscriptions</h3>
        
        <button 
          onClick={handleSubscribeTokens}
          disabled={!isConnected}
          style={{ marginRight: '10px', padding: '8px 16px' }}
        >
          Subscribe to Tokens
        </button>

        <button 
          onClick={handleSubscribeGraph}
          disabled={!isConnected}
          style={{ padding: '8px 16px' }}
        >
          Subscribe to Graph
        </button>
      </div>

      {/* Token List */}
      {tokens.length > 0 && (
        <div style={{ marginBottom: '20px' }}>
          <h3>Tokens ({tokens.length})</h3>
          <div style={{ 
            maxHeight: '200px', 
            overflow: 'auto',
            border: '1px solid #ddd',
            padding: '10px'
          }}>
            {tokens.map((token, index) => (
              <div key={index} style={{ 
                marginBottom: '8px',
                padding: '8px',
                backgroundColor: '#f8f9fa',
                borderRadius: '4px'
              }}>
                <div><strong>ID:</strong> {token.token_id}</div>
                <div><strong>Type:</strong> {token.type}</div>
                <div><strong>Weight:</strong> {token.weight}</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Message Log */}
      <div>
        <h3>Message Log</h3>
        <button 
          onClick={() => setMessages([])}
          style={{ marginBottom: '10px', padding: '6px 12px' }}
        >
          Clear Log
        </button>
        <div style={{ 
          maxHeight: '400px', 
          overflow: 'auto',
          border: '1px solid #ddd',
          padding: '10px',
          backgroundColor: '#f8f9fa'
        }}>
          {messages.length === 0 && (
            <div style={{ color: '#999' }}>No messages yet...</div>
          )}
          {messages.map((msg, index) => (
            <div key={index} style={{ 
              marginBottom: '10px',
              paddingBottom: '10px',
              borderBottom: '1px solid #ddd'
            }}>
              <div><strong>{msg.type}</strong></div>
              <div style={{ fontSize: '12px', color: '#666' }}>
                {new Date(msg.timestamp).toLocaleTimeString()}
              </div>
              <pre style={{ 
                fontSize: '12px',
                backgroundColor: '#fff',
                padding: '8px',
                borderRadius: '4px',
                marginTop: '5px',
                overflow: 'auto'
              }}>
                {JSON.stringify(msg.payload, null, 2)}
              </pre>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

export default WebSocketExample;

