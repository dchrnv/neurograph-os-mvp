/**
 * Chat page - AI chat interface
 */

import { useState, useEffect, useRef } from 'react';
import { Card, Input, Button, Space, Empty, Spin, List, Typography, Drawer } from 'antd';
import {
  SendOutlined,
  DeleteOutlined,
  PlusOutlined,
  MenuOutlined,
  CloseOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import ChatMessage from '../components/ChatMessage';
import { useChatStore } from '../stores/chatStore';
import { api } from '../services/api';
import { ws } from '../services/websocket';
import { WS_CHANNELS } from '../utils/constants';

const { TextArea } = Input;
const { Text } = Typography;

export default function Chat() {
  const { t } = useTranslation();
  const {
    currentSessionId,
    sessions,
    createSession,
    deleteSession,
    setCurrentSession,
    addMessage,
    updateMessage,
    clearMessages,
    getCurrentMessages,
  } = useChatStore();

  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [drawerOpen, setDrawerOpen] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const messages = getCurrentMessages();

  useEffect(() => {
    // Create initial session if none exists
    if (sessions.length === 0) {
      createSession('Chat Session');
    }

    // Subscribe to chat WebSocket
    ws.subscribe(WS_CHANNELS.CHAT, handleChatMessage);

    return () => {
      ws.unsubscribe(WS_CHANNELS.CHAT, handleChatMessage);
    };
  }, []);

  useEffect(() => {
    // Scroll to bottom when messages change
    scrollToBottom();
  }, [messages]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  const handleChatMessage = (data: any) => {
    if (data.type === 'chunk') {
      // Streaming chunk
      const lastMessage = messages[messages.length - 1];
      if (lastMessage && lastMessage.streaming) {
        updateMessage(lastMessage.id, {
          content: lastMessage.content + data.content,
        });
      }
    } else if (data.type === 'complete') {
      // Streaming complete
      const lastMessage = messages[messages.length - 1];
      if (lastMessage) {
        updateMessage(lastMessage.id, { streaming: false });
      }
      setLoading(false);
    }
  };

  const handleSend = async () => {
    if (!input.trim() || loading) return;

    const userMessage = input.trim();
    setInput('');
    setLoading(true);

    // Add user message
    addMessage({
      role: 'user',
      content: userMessage,
    });

    try {
      // Send to API
      const response = await api.sendChatMessage(userMessage);

      // Add assistant response (placeholder for streaming)
      addMessage({
        role: 'assistant',
        content: response.message || '',
        streaming: false,
      });
    } catch (error) {
      console.error('Chat error:', error);
      addMessage({
        role: 'system',
        content: 'Error: Failed to get response from AI',
      });
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleNewChat = () => {
    createSession(`Chat ${sessions.length + 1}`);
    setDrawerOpen(false);
  };

  const handleClearChat = () => {
    clearMessages();
  };

  return (
    <div style={{ padding: 24, height: 'calc(100vh - 120px)', display: 'flex', flexDirection: 'column' }}>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h1 style={{ margin: 0 }}>Chat</h1>
        <Space>
          <Button icon={<PlusOutlined />} onClick={handleNewChat}>
            New Chat
          </Button>
          <Button icon={<MenuOutlined />} onClick={() => setDrawerOpen(true)}>
            Sessions
          </Button>
          <Button
            icon={<DeleteOutlined />}
            onClick={handleClearChat}
            disabled={messages.length === 0}
          >
            Clear
          </Button>
        </Space>
      </div>

      {/* Messages */}
      <Card
        style={{
          flex: 1,
          marginBottom: 16,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
        }}
        bodyStyle={{
          flex: 1,
          overflow: 'auto',
          padding: 24,
        }}
      >
        {messages.length === 0 ? (
          <Empty
            description="No messages yet. Start a conversation!"
            style={{ marginTop: 100 }}
          />
        ) : (
          <div>
            {messages.map((message) => (
              <ChatMessage key={message.id} message={message} />
            ))}
            {loading && (
              <div style={{ textAlign: 'center', padding: 16 }}>
                <Spin />
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        )}
      </Card>

      {/* Input */}
      <div style={{ display: 'flex', gap: 8 }}>
        <TextArea
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Type your message... (Shift+Enter for new line)"
          autoSize={{ minRows: 1, maxRows: 4 }}
          disabled={loading}
          style={{ flex: 1 }}
        />
        <Button
          type="primary"
          icon={<SendOutlined />}
          onClick={handleSend}
          loading={loading}
          disabled={!input.trim()}
          style={{ height: 'auto' }}
        >
          Send
        </Button>
      </div>

      {/* Sessions Drawer */}
      <Drawer
        title="Chat Sessions"
        placement="right"
        onClose={() => setDrawerOpen(false)}
        open={drawerOpen}
        width={300}
      >
        <List
          dataSource={sessions}
          renderItem={(session) => (
            <List.Item
              style={{
                cursor: 'pointer',
                background: session.id === currentSessionId ? '#1890ff10' : undefined,
                padding: 12,
                borderRadius: 4,
              }}
              onClick={() => {
                setCurrentSession(session.id);
                setDrawerOpen(false);
              }}
              actions={[
                <Button
                  key="delete"
                  type="text"
                  size="small"
                  danger
                  icon={<CloseOutlined />}
                  onClick={(e) => {
                    e.stopPropagation();
                    deleteSession(session.id);
                  }}
                />,
              ]}
            >
              <List.Item.Meta
                title={session.title}
                description={
                  <Text type="secondary" style={{ fontSize: 12 }}>
                    {session.messages.length} messages
                  </Text>
                }
              />
            </List.Item>
          )}
        />
      </Drawer>
    </div>
  );
}
