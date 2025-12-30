/**
 * Modules page - module management and monitoring
 */

import { useEffect, useState } from 'react';
import { Row, Col, Button, Space, message, Spin } from 'antd';
import { ReloadOutlined, PlayCircleOutlined, PauseCircleOutlined } from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import ModuleCard from '../components/ModuleCard';
import ModuleDetailsDrawer from '../components/ModuleDetailsDrawer';
import { useModuleStore } from '../stores/moduleStore';
import { api } from '../services/api';
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
      const data = await api.getModules();
      setModules(data);
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

  const handleStart = async (id: string) => {
    setActionLoading(id);
    try {
      await api.startModule(id);
      updateModule(id, { status: 'starting' });
      message.success(t('modules.startSuccess', `Starting module ${id}`));
    } catch (error) {
      message.error(t('modules.startError', 'Failed to start module'));
      console.error('Start module error:', error);
    } finally {
      setActionLoading(null);
    }
  };

  const handleStop = async (id: string) => {
    setActionLoading(id);
    try {
      await api.stopModule(id);
      updateModule(id, { status: 'stopped' });
      message.success(t('modules.stopSuccess', `Stopping module ${id}`));
    } catch (error) {
      message.error(t('modules.stopError', 'Failed to stop module'));
      console.error('Stop module error:', error);
    } finally {
      setActionLoading(null);
    }
  };

  const handleRestart = async (id: string) => {
    setActionLoading(id);
    try {
      await api.restartModule(id);
      updateModule(id, { status: 'restarting' });
      message.success(t('modules.restartSuccess', `Restarting module ${id}`));
    } catch (error) {
      message.error(t('modules.restartError', 'Failed to restart module'));
      console.error('Restart module error:', error);
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

  const handleStartAll = async () => {
    const stoppedModules = modules.filter((m) => m.status === 'stopped');
    for (const module of stoppedModules) {
      await handleStart(module.id);
    }
  };

  const handleStopAll = async () => {
    const runningModules = modules.filter((m) => m.status === 'running');
    for (const module of runningModules) {
      await handleStop(module.id);
    }
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
          <Button icon={<PlayCircleOutlined />} onClick={handleStartAll}>
            Start All
          </Button>
          <Button icon={<PauseCircleOutlined />} onClick={handleStopAll}>
            Stop All
          </Button>
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
                onStart={handleStart}
                onStop={handleStop}
                onRestart={handleRestart}
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
