/**
 * Connection Indicator Component
 * Shows WebSocket connection status
 */

import { useEffect, useState } from 'react';
import { Badge, Tooltip } from 'antd';
import { ws } from '../services/websocket';
import { useTranslation } from 'react-i18next';

export default function ConnectionIndicator() {
  const { t } = useTranslation();
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    const checkConnection = () => {
      // Check if WebSocket is connected
      setIsConnected(ws.isConnected());
    };

    // Check initially
    checkConnection();

    // Check periodically
    const interval = setInterval(checkConnection, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <Tooltip
      title={
        isConnected
          ? t('connection.connected')
          : t('connection.disconnected')
      }
    >
      <Badge
        status={isConnected ? 'success' : 'error'}
        text={
          <span style={{ fontSize: 12, cursor: 'pointer' }}>
            {isConnected ? 'Connected' : 'Disconnected'}
          </span>
        }
      />
    </Tooltip>
  );
}
