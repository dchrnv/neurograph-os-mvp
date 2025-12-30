import { ConfigProvider, theme } from 'antd';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { useEffect } from 'react';
import { useAppStore } from './stores/appStore';
import { ws } from './services/websocket';
import { ROUTES } from './utils/constants';
import ErrorBoundary from './components/ErrorBoundary';
import MainLayout from './layouts/MainLayout';
import Dashboard from './pages/Dashboard';
import Modules from './pages/Modules';
import Config from './pages/Config';
import Bootstrap from './pages/Bootstrap';
import Chat from './pages/Chat';
import Terminal from './pages/Terminal';
import Admin from './pages/Admin';
import NotFound from './pages/NotFound';
import './i18n';

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
    <ErrorBoundary>
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
              <Route path={ROUTES.ADMIN} element={<Admin />} />
              <Route path="*" element={<NotFound />} />
            </Route>
          </Routes>
        </BrowserRouter>
      </ConfigProvider>
    </ErrorBoundary>
  );
}

export default App;
