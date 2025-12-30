/**
 * Main layout using Ant Design ProLayout
 */

import { ProLayout } from '@ant-design/pro-components';
import {
  DashboardOutlined,
  AppstoreOutlined,
  SettingOutlined,
  RocketOutlined,
  MessageOutlined,
  CodeOutlined,
  SafetyOutlined,
  BulbOutlined,
  GlobalOutlined,
} from '@ant-design/icons';
import { useNavigate, useLocation, Outlet } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { Switch, Dropdown } from 'antd';
import { useAppStore } from '../stores/appStore';
import { ROUTES } from '../utils/constants';
import ConnectionIndicator from '../components/ConnectionIndicator';

export default function MainLayout() {
  const navigate = useNavigate();
  const location = useLocation();
  const { t, i18n } = useTranslation();
  const { theme, toggleTheme, language, setLanguage, sidebarCollapsed, setSidebarCollapsed } = useAppStore();

  const menuItems = [
    {
      path: ROUTES.DASHBOARD,
      name: t('nav.dashboard'),
      icon: <DashboardOutlined />,
    },
    {
      path: ROUTES.MODULES,
      name: t('nav.modules'),
      icon: <AppstoreOutlined />,
    },
    {
      path: ROUTES.CONFIG,
      name: t('nav.config'),
      icon: <SettingOutlined />,
    },
    {
      path: ROUTES.BOOTSTRAP,
      name: t('nav.bootstrap'),
      icon: <RocketOutlined />,
    },
    {
      path: ROUTES.CHAT,
      name: t('nav.chat'),
      icon: <MessageOutlined />,
    },
    {
      path: ROUTES.TERMINAL,
      name: t('nav.terminal'),
      icon: <CodeOutlined />,
    },
    {
      path: ROUTES.ADMIN,
      name: t('nav.admin'),
      icon: <SafetyOutlined />,
    },
  ];

  const languageMenu = {
    items: [
      {
        key: 'en',
        label: 'English',
        onClick: () => {
          setLanguage('en');
          i18n.changeLanguage('en');
        },
      },
      {
        key: 'ru',
        label: 'Русский',
        onClick: () => {
          setLanguage('ru');
          i18n.changeLanguage('ru');
        },
      },
    ],
  };

  return (
    <ProLayout
      title={t('app.title')}
      logo={<BulbOutlined style={{ fontSize: 24 }} />}
      layout="mix"
      theme={theme}
      route={{
        path: '/',
        routes: menuItems,
      }}
      location={location}
      collapsed={sidebarCollapsed}
      onCollapse={setSidebarCollapsed}
      menuItemRender={(item, dom) => (
        <div onClick={() => navigate(item.path || '/')}>{dom}</div>
      )}
      actionsRender={() => [
        <ConnectionIndicator key="connection" />,
        <Switch
          key="theme"
          checked={theme === 'dark'}
          onChange={toggleTheme}
          checkedChildren="🌙"
          unCheckedChildren="☀️"
        />,
        <Dropdown key="language" menu={languageMenu}>
          <GlobalOutlined style={{ fontSize: 18, cursor: 'pointer' }} />
        </Dropdown>,
      ]}
    >
      <Outlet />
    </ProLayout>
  );
}
