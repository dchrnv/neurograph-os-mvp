import { ConfigProvider, theme } from 'antd';
import { BrowserRouter } from 'react-router-dom';
import { useEffect } from 'react';
import { ws } from './services/websocket';

function App() {
  useEffect(() => {
    // Connect WebSocket on mount
    ws.connect();

    return () => {
      ws.disconnect();
    };
  }, []);

  return (
    <ConfigProvider
      theme={{
        algorithm: theme.darkAlgorithm,
        token: {
          colorPrimary: '#1890ff',
        },
      }}
    >
      <BrowserRouter>
        <div style={{ padding: '24px', minHeight: '100vh', backgroundColor: '#141414', color: '#fff' }}>
          <h1>🧠 NeuroGraph Dashboard</h1>
          <p>v0.62.0 - Web Dashboard (Phase 1: Project Setup)</p>
          <div style={{ marginTop: '24px', padding: '16px', backgroundColor: '#1f1f1f', borderRadius: '8px' }}>
            <h2>✅ Phase 1 Complete</h2>
            <ul>
              <li>✅ Vite + React + TypeScript configured</li>
              <li>✅ Ant Design Pro setup</li>
              <li>✅ TypeScript types defined</li>
              <li>✅ API service created</li>
              <li>✅ WebSocket service created</li>
              <li>✅ Utils and formatters ready</li>
              <li>✅ Dark/Light theme support</li>
            </ul>
            <p style={{ marginTop: '16px', color: '#52c41a' }}>
              <strong>Status:</strong> Ready for Phase 2 - Dashboard implementation
            </p>
          </div>
        </div>
      </BrowserRouter>
    </ConfigProvider>
  );
}

export default App;
