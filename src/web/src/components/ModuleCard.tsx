/**
 * Module card component for displaying module status and controls
 */

import { Card, Tag, Button, Space, Tooltip, Badge } from 'antd';
import {
  PlayCircleOutlined,
  PauseCircleOutlined,
  ReloadOutlined,
  SettingOutlined,
  FileTextOutlined,
  InfoCircleOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import type { Module } from '../types/modules';
import { MODULE_STATUS_COLORS } from '../utils/constants';

interface ModuleCardProps {
  module: Module;
  onStart: (id: string) => void;
  onStop: (id: string) => void;
  onRestart: (id: string) => void;
  onConfigure: (id: string) => void;
  onViewLogs: (id: string) => void;
  onViewDetails: (id: string) => void;
  loading?: boolean;
}

export default function ModuleCard({
  module,
  onStart,
  onStop,
  onRestart,
  onConfigure,
  onViewLogs,
  onViewDetails,
  loading = false,
}: ModuleCardProps) {
  const { t } = useTranslation();

  const getStatusTag = () => {
    const color = MODULE_STATUS_COLORS[module.status];
    return (
      <Tag color={color}>
        {t(`modules.status.${module.status}`, module.status.toUpperCase())}
      </Tag>
    );
  };

  const canStart = module.status === 'stopped' || module.status === 'error';
  const canStop = module.status === 'running' || module.status === 'starting';
  const canRestart = module.status === 'running';

  return (
    <Badge.Ribbon
      text={module.status === 'running' ? '●' : '○'}
      color={MODULE_STATUS_COLORS[module.status]}
    >
      <Card
        loading={loading}
        title={
          <Space>
            <span>{module.name}</span>
            {getStatusTag()}
          </Space>
        }
        extra={
          <Tooltip title={t('modules.viewDetails')}>
            <Button
              type="text"
              icon={<InfoCircleOutlined />}
              onClick={() => onViewDetails(module.id)}
            />
          </Tooltip>
        }
        actions={[
          <Tooltip key="start" title={t('modules.start')}>
            <Button
              type="text"
              icon={<PlayCircleOutlined />}
              disabled={!canStart || loading}
              onClick={() => onStart(module.id)}
              style={{ color: canStart ? '#52c41a' : undefined }}
            />
          </Tooltip>,
          <Tooltip key="stop" title={t('modules.stop')}>
            <Button
              type="text"
              icon={<PauseCircleOutlined />}
              disabled={!canStop || loading}
              onClick={() => onStop(module.id)}
              style={{ color: canStop ? '#ff4d4f' : undefined }}
            />
          </Tooltip>,
          <Tooltip key="restart" title={t('modules.restart')}>
            <Button
              type="text"
              icon={<ReloadOutlined />}
              disabled={!canRestart || loading}
              onClick={() => onRestart(module.id)}
              style={{ color: canRestart ? '#faad14' : undefined }}
            />
          </Tooltip>,
          <Tooltip key="logs" title={t('modules.viewLogs')}>
            <Button
              type="text"
              icon={<FileTextOutlined />}
              onClick={() => onViewLogs(module.id)}
            />
          </Tooltip>,
          <Tooltip key="config" title={t('modules.configure')}>
            <Button
              type="text"
              icon={<SettingOutlined />}
              onClick={() => onConfigure(module.id)}
            />
          </Tooltip>,
        ]}
      >
        <Space direction="vertical" style={{ width: '100%' }}>
          <div>
            <strong>{t('modules.version')}:</strong> {module.version}
          </div>

          {module.metrics && Object.keys(module.metrics).length > 0 && (
            <div>
              <strong>Metrics:</strong>
              <div style={{ marginTop: 8, fontSize: 12 }}>
                {Object.entries(module.metrics).map(([key, value]) => (
                  <div key={key}>
                    {key}: <Tag>{String(value)}</Tag>
                  </div>
                ))}
              </div>
            </div>
          )}

          {module.restarts && module.restarts > 0 && (
            <div>
              <Tag color="warning">Restarts: {module.restarts}</Tag>
            </div>
          )}
        </Space>
      </Card>
    </Badge.Ribbon>
  );
}
