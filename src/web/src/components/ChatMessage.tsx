/**
 * Chat message component with markdown support
 */

import { Avatar, Typography } from 'antd';
import { UserOutlined, RobotOutlined, InfoCircleOutlined } from '@ant-design/icons';
import type { ChatMessage as ChatMessageType } from '../types/chat';

const { Paragraph } = Typography;

interface ChatMessageProps {
  message: ChatMessageType;
}

export default function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === 'user';
  const isSystem = message.role === 'system';

  const getIcon = () => {
    if (isUser) return <UserOutlined />;
    if (isSystem) return <InfoCircleOutlined />;
    return <RobotOutlined />;
  };

  const getBackgroundColor = () => {
    if (isUser) return '#1890ff';
    if (isSystem) return '#faad14';
    return '#52c41a';
  };

  const timestamp = new Date(message.timestamp).toLocaleTimeString();

  return (
    <div
      style={{
        display: 'flex',
        gap: 12,
        marginBottom: 16,
        justifyContent: isUser ? 'flex-end' : 'flex-start',
      }}
    >
      {!isUser && (
        <Avatar
          icon={getIcon()}
          style={{ backgroundColor: getBackgroundColor(), flexShrink: 0 }}
        />
      )}

      <div
        style={{
          maxWidth: '70%',
          background: isUser ? '#1890ff' : '#262626',
          color: isUser ? '#fff' : '#e0e0e0',
          padding: '12px 16px',
          borderRadius: 8,
          position: 'relative',
        }}
      >
        {/* Message content */}
        <Paragraph
          style={{
            margin: 0,
            color: 'inherit',
            whiteSpace: 'pre-wrap',
            wordBreak: 'break-word',
          }}
        >
          {message.content}
          {message.streaming && (
            <span
              style={{
                display: 'inline-block',
                width: 8,
                height: 16,
                marginLeft: 4,
                background: 'currentColor',
                animation: 'blink 1s infinite',
              }}
            />
          )}
        </Paragraph>

        {/* Timestamp */}
        <div
          style={{
            fontSize: 11,
            opacity: 0.6,
            marginTop: 4,
            textAlign: isUser ? 'right' : 'left',
          }}
        >
          {timestamp}
        </div>
      </div>

      {isUser && (
        <Avatar
          icon={getIcon()}
          style={{ backgroundColor: getBackgroundColor(), flexShrink: 0 }}
        />
      )}

      <style>{`
        @keyframes blink {
          0%, 50% { opacity: 1; }
          51%, 100% { opacity: 0; }
        }
      `}</style>
    </div>
  );
}
