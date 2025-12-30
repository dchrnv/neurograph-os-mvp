/**
 * Module card component for displaying module status and controls
 */

import { Card, Tag, Button, Space, Tooltip, Badge, Switch, Alert } from 'antd';
import {
  SettingOutlined,
  FileTextOutlined,
  InfoCircleOutlined,
  WarningOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import type { Module } from '../types/modules';

interface ModuleCardProps {
  module: Module;
  onToggleEnabled: (id: string, enabled: boolean) => void;
  onConfigure: (id: string) => void;
  onViewLogs: (id: string) => void;
  onViewDetails: (id: string) => void;
  loading?: boolean;
}

const STATUS_COLORS: Record<string, string> = {
  active: 'success',
  disabled: 'default',
  error: 'error',
};

export default function ModuleCard({
  module,
  onToggleEnabled,
  onConfigure,
  onViewLogs,
  onViewDetails,
  loading = false,
}: ModuleCardProps) {
  const { t } = useTranslation();

  const getStatusTag = () => {
    const color = STATUS_COLORS[module.status];
    return (
      <Tag color={color}>
        {t(`modules.status.${module.status}`, module.status.toUpperCase())}
      </Tag>
    );
  };

  return (
    <Badge.Ribbon
      text={module.enabled ? '●' : '○'}
      color={module.enabled ? '#52c41a' : '#d9d9d9'}
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
              disabled={!module.configurable}
            />
          </Tooltip>,
        ]}
      >
        <Space direction="vertical" style={{ width: '100%' }} size="middle">
          <div>
            <strong>{t('modules.version')}:</strong> {module.version}
          </div>

          <div>
            <p style={{ marginBottom: 8, fontSize: 12, color: '#666' }}>
              {module.description}
            </p>
          </div>

          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
            <span>
              <strong>Enabled:</strong>
            </span>
            <Switch
              checked={module.enabled}
              disabled={!module.can_disable || loading}
              onChange={(checked) => onToggleEnabled(module.id, checked)}
              checkedChildren="ON"
              unCheckedChildren="OFF"
            />
          </div>

          {!module.can_disable && (
            <Alert
              message="Core Module"
              description="This module cannot be disabled"
              type="info"
              showIcon
              icon={<InfoCircleOutlined />}
              style={{ fontSize: 12 }}
            />
          )}

          {module.disable_warning && !module.enabled && (
            <Alert
              message="Warning"
              description={module.disable_warning}
              type="warning"
              showIcon
              icon={<WarningOutlined />}
              style={{ fontSize: 12 }}
            />
          )}

          {module.metrics && (
            <div>
              <strong style={{ fontSize: 12 }}>Metrics:</strong>
              <div style={{ marginTop: 4, fontSize: 11 }}>
                <div>Operations: <Tag>{module.metrics.operations}</Tag></div>
                <div>Ops/sec: <Tag>{module.metrics.ops_per_sec.toFixed(2)}</Tag></div>
                <div>Avg Latency: <Tag>{module.metrics.avg_latency_us.toFixed(2)}μs</Tag></div>
                {module.metrics.errors > 0 && (
                  <div>Errors: <Tag color="error">{module.metrics.errors}</Tag></div>
                )}
              </div>
            </div>
          )}
        </Space>
      </Card>
    </Badge.Ribbon>
  );
}
