import { ConfigProvider, theme } from 'antd';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { useEffect } from 'react';
import { useAppStore } from './stores/appStore';
import { ws } from './services/websocket';
import { ROUTES } from './utils/constants';
import MainLayout from './layouts/MainLayout';
import Dashboard from './pages/Dashboard';
import Modules from './pages/Modules';
import Config from './pages/Config';
import Bootstrap from './pages/Bootstrap';
import Chat from './pages/Chat';
import Terminal from './pages/Terminal';
import './i18n';

// Placeholder pages
const PlaceholderPage = ({ title }: { title: string }) => (
  <div style={{ padding: 24 }}>
    <h1>{title}</h1>
    <p>Coming soon...</p>
  </div>
);

function App() {
  const appTheme = useAppStore((state) => state.theme);

  useEffect(() => {
    // Connect WebSocket on mount
    ws.connect();

    // Set initial theme
    document.documentElement.setAttribute('data-theme', appTheme);

    return () => {
      ws.disconnect();
    };
  }, []);

  return (
    <ConfigProvider
      theme={{
        algorithm: appTheme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
        token: {
          colorPrimary: '#1890ff',
        },
      }}
    >
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<MainLayout />}>
            <Route index element={<Dashboard />} />
            <Route path={ROUTES.MODULES} element={<Modules />} />
            <Route path={ROUTES.CONFIG} element={<Config />} />
            <Route path={ROUTES.BOOTSTRAP} element={<Bootstrap />} />
            <Route path={ROUTES.CHAT} element={<Chat />} />
            <Route path={ROUTES.TERMINAL} element={<Terminal />} />
            <Route path={ROUTES.ADMIN} element={<PlaceholderPage title="Admin" />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </ConfigProvider>
  );
}

export default App;
