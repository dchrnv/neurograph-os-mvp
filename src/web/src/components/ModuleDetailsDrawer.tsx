/**
 * Module details drawer - shows logs, config, and detailed info
 */

import { Drawer, Tabs, Descriptions, Tag, List, Typography, Empty } from 'antd';
import { useTranslation } from 'react-i18next';
import type { Module } from '../types/modules';
import { MODULE_STATUS_COLORS } from '../utils/constants';

const { Text, Paragraph } = Typography;

interface ModuleDetailsDrawerProps {
  module: Module | null;
  open: boolean;
  onClose: () => void;
}

export default function ModuleDetailsDrawer({
  module,
  open,
  onClose,
}: ModuleDetailsDrawerProps) {
  const { t } = useTranslation();

  if (!module) return null;

  const items = [
    {
      key: 'info',
      label: 'Info',
      children: (
        <Descriptions column={1} bordered>
          <Descriptions.Item label={t('modules.name')}>
            {module.name}
          </Descriptions.Item>
          <Descriptions.Item label={t('modules.version')}>
            {module.version}
          </Descriptions.Item>
          <Descriptions.Item label={t('modules.status')}>
            <Tag color={MODULE_STATUS_COLORS[module.status]}>
              {module.status.toUpperCase()}
            </Tag>
          </Descriptions.Item>
          {module.restarts !== undefined && (
            <Descriptions.Item label="Restarts">
              {module.restarts}
            </Descriptions.Item>
          )}
        </Descriptions>
      ),
    },
    {
      key: 'metrics',
      label: 'Metrics',
      children: module.metrics && Object.keys(module.metrics).length > 0 ? (
        <Descriptions column={1} bordered>
          {Object.entries(module.metrics).map(([key, value]) => (
            <Descriptions.Item key={key} label={key}>
              <Text code>{String(value)}</Text>
            </Descriptions.Item>
          ))}
        </Descriptions>
      ) : (
        <Empty description="No metrics available" />
      ),
    },
    {
      key: 'config',
      label: 'Config',
      children: module.config && Object.keys(module.config).length > 0 ? (
        <Paragraph>
          <pre style={{
            background: '#1f1f1f',
            padding: 16,
            borderRadius: 4,
            overflow: 'auto',
            maxHeight: '60vh'
          }}>
            {JSON.stringify(module.config, null, 2)}
          </pre>
        </Paragraph>
      ) : (
        <Empty description="No configuration available" />
      ),
    },
    {
      key: 'logs',
      label: 'Logs',
      children: module.logs && module.logs.length > 0 ? (
        <List
          dataSource={module.logs}
          renderItem={(log) => (
            <List.Item>
              <Text code style={{ width: '100%', display: 'block' }}>
                {log}
              </Text>
            </List.Item>
          )}
          style={{ maxHeight: '60vh', overflow: 'auto' }}
        />
      ) : (
        <Empty description="No logs available" />
      ),
    },
  ];

  return (
    <Drawer
      title={`Module: ${module.name}`}
      placement="right"
      onClose={onClose}
      open={open}
      width={600}
    >
      <Tabs items={items} />
    </Drawer>
  );
}
