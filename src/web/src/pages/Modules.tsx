/**
 * Modules page - module management and monitoring
 */

import { useEffect, useState } from 'react';
import { Row, Col, Button, Space, message, Spin, Modal } from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import ModuleCard from '../components/ModuleCard';
import ModuleDetailsDrawer from '../components/ModuleDetailsDrawer';
import { useModuleStore } from '../stores/moduleStore';
import { ws } from '../services/websocket';
import { WS_CHANNELS } from '../utils/constants';
import type { Module } from '../types/modules';

export default function Modules() {
  const { t } = useTranslation();
  const { modules, setModules, updateModule, selectedModule, setSelectedModule, loading, setLoading } =
    useModuleStore();
  const [detailsOpen, setDetailsOpen] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  useEffect(() => {
    loadModules();

    // Subscribe to module updates via WebSocket
    ws.subscribe(WS_CHANNELS.MODULES, handleModuleUpdate);

    return () => {
      ws.unsubscribe(WS_CHANNELS.MODULES, handleModuleUpdate);
    };
  }, []);

  const loadModules = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/v1/modules');
      const data = await response.json();
      setModules(data.modules || []);
    } catch (error) {
      message.error('Failed to load modules');
      console.error('Load modules error:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleModuleUpdate = (data: Module) => {
    updateModule(data.id, data);
  };

  const handleToggleEnabled = async (id: string, enabled: boolean) => {
    const module = modules.find((m) => m.id === id);
    if (!module) return;

    // Показываем предупреждение, если оно есть
    if (!enabled && module.disable_warning) {
      Modal.confirm({
        title: t('common.confirm'),
        content: module.disable_warning,
        okText: t('common.confirm'),
        okType: 'danger',
        cancelText: t('common.cancel'),
        onOk: async () => {
          await toggleModule(id, enabled);
        },
      });
    } else {
      await toggleModule(id, enabled);
    }
  };

  const toggleModule = async (id: string, enabled: boolean) => {
    setActionLoading(id);
    try {
      const response = await fetch(`/api/v1/modules/${id}/enabled`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ enabled }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.detail || 'Failed to update module');
      }

      const result = await response.json();
      message.success(result.message);

      // Обновляем локальное состояние
      updateModule(id, { enabled });

      // Перезагружаем данные модулей
      await loadModules();
    } catch (error: any) {
      message.error(error.message || 'Failed to update module');
      console.error('Toggle module error:', error);
    } finally {
      setActionLoading(null);
    }
  };

  const handleConfigure = (id: string) => {
    message.info(`Configure module ${id} - coming soon`);
  };

  const handleViewLogs = (id: string) => {
    const module = modules.find((m) => m.id === id);
    if (module) {
      setSelectedModule(id);
      setDetailsOpen(true);
    }
  };

  const handleViewDetails = (id: string) => {
    setSelectedModule(id);
    setDetailsOpen(true);
  };

  const selectedModuleData = modules.find((m) => m.id === selectedModule);

  if (loading && modules.length === 0) {
    return (
      <div style={{ padding: 24, textAlign: 'center' }}>
        <Spin size="large" />
        <div style={{ marginTop: 16 }}>Loading modules...</div>
      </div>
    );
  }

  return (
    <div style={{ padding: 24 }}>
      <div style={{ marginBottom: 24, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h1>{t('modules.title')}</h1>
        <Space>
          <Button icon={<ReloadOutlined />} onClick={loadModules}>
            {t('common.refresh')}
          </Button>
        </Space>
      </div>

      {modules.length === 0 ? (
        <div style={{ textAlign: 'center', padding: 48 }}>
          <p>No modules found</p>
        </div>
      ) : (
        <Row gutter={[16, 16]}>
          {modules.map((module) => (
            <Col xs={24} sm={12} lg={8} xl={6} key={module.id}>
              <ModuleCard
                module={module}
                onToggleEnabled={handleToggleEnabled}
                onConfigure={handleConfigure}
                onViewLogs={handleViewLogs}
                onViewDetails={handleViewDetails}
                loading={actionLoading === module.id}
              />
            </Col>
          ))}
        </Row>
      )}

      <ModuleDetailsDrawer
        module={selectedModuleData || null}
        open={detailsOpen}
        onClose={() => setDetailsOpen(false)}
      />
    </div>
  );
}
