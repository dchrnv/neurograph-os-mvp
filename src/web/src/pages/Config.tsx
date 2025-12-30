/**
 * Configuration page - system settings and CDNA scales
 */

import { useState, useEffect } from 'react';
import { Card, Tabs, Form, Input, InputNumber, Switch, Button, Space, message, Spin } from 'antd';
import { SaveOutlined, UndoOutlined } from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import CDNAScalesEditor from '../components/CDNAScalesEditor';
import { api } from '../services/api';
import type { CDNAConfig } from '../types/metrics';

interface SystemConfig {
  max_tokens?: number;
  query_timeout_ms?: number;
  cache_enabled?: boolean;
  log_level?: string;
  websocket_port?: number;
  api_port?: number;
}

export default function Config() {
  const { t } = useTranslation();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [systemConfig, setSystemConfig] = useState<SystemConfig>({});
  const [cdnaConfig, setCdnaConfig] = useState<CDNAConfig>({
    profile: 'balanced',
    scales: {
      physical: 50,
      sensory: 50,
      motor: 50,
      emotional: 50,
      cognitive: 50,
      social: 50,
      temporal: 50,
      abstract: 50,
    },
  });

  const [form] = Form.useForm();

  useEffect(() => {
    loadConfiguration();
  }, []);

  const loadConfiguration = async () => {
    setLoading(true);
    try {
      const [sysConfig, cdna] = await Promise.all([
        api.getConfig(),
        api.getCDNAConfig(),
      ]);

      setSystemConfig(sysConfig);
      setCdnaConfig(cdna);
      form.setFieldsValue(sysConfig);
    } catch (error) {
      message.error('Failed to load configuration');
      console.error('Load config error:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleSaveSystem = async () => {
    setSaving(true);
    try {
      const values = await form.validateFields();
      await api.updateConfig('system', values);
      setSystemConfig(values);
      message.success(t('config.saveSuccess', 'Configuration saved successfully'));
    } catch (error) {
      message.error(t('config.saveError', 'Failed to save configuration'));
      console.error('Save config error:', error);
    } finally {
      setSaving(false);
    }
  };

  const handleSaveCDNA = async () => {
    setSaving(true);
    try {
      await api.updateCDNAScales(cdnaConfig.scales);
      message.success(t('config.saveSuccess', 'CDNA scales saved successfully'));
    } catch (error) {
      message.error(t('config.saveError', 'Failed to save CDNA scales'));
      console.error('Save CDNA error:', error);
    } finally {
      setSaving(false);
    }
  };

  const handleResetSystem = () => {
    form.resetFields();
    message.info(t('config.resetInfo', 'Configuration reset to last saved values'));
  };

  const handleResetCDNA = () => {
    setCdnaConfig({
      profile: 'balanced',
      scales: {
        physical: 50,
        sensory: 50,
        motor: 50,
        emotional: 50,
        cognitive: 50,
        social: 50,
        temporal: 50,
        abstract: 50,
      },
    });
    message.info(t('config.resetInfo', 'CDNA scales reset to balanced profile'));
  };

  if (loading) {
    return (
      <div style={{ padding: 24, textAlign: 'center' }}>
        <Spin size="large" />
        <div style={{ marginTop: 16 }}>Loading configuration...</div>
      </div>
    );
  }

  const items = [
    {
      key: 'system',
      label: t('config.system'),
      children: (
        <Card>
          <Form form={form} layout="vertical" initialValues={systemConfig}>
            <Form.Item
              label="Max Tokens"
              name="max_tokens"
              rules={[{ type: 'number', min: 1000 }]}
            >
              <InputNumber style={{ width: '100%' }} min={1000} step={1000} />
            </Form.Item>

            <Form.Item
              label="Query Timeout (ms)"
              name="query_timeout_ms"
              rules={[{ type: 'number', min: 100 }]}
            >
              <InputNumber style={{ width: '100%' }} min={100} step={100} />
            </Form.Item>

            <Form.Item label="Cache Enabled" name="cache_enabled" valuePropName="checked">
              <Switch />
            </Form.Item>

            <Form.Item label="Log Level" name="log_level">
              <Input placeholder="DEBUG, INFO, WARNING, ERROR" />
            </Form.Item>

            <Form.Item
              label="WebSocket Port"
              name="websocket_port"
              rules={[{ type: 'number', min: 1024, max: 65535 }]}
            >
              <InputNumber style={{ width: '100%' }} min={1024} max={65535} />
            </Form.Item>

            <Form.Item
              label="API Port"
              name="api_port"
              rules={[{ type: 'number', min: 1024, max: 65535 }]}
            >
              <InputNumber style={{ width: '100%' }} min={1024} max={65535} />
            </Form.Item>

            <Form.Item>
              <Space>
                <Button
                  type="primary"
                  icon={<SaveOutlined />}
                  onClick={handleSaveSystem}
                  loading={saving}
                >
                  {t('config.save')}
                </Button>
                <Button icon={<UndoOutlined />} onClick={handleResetSystem}>
                  {t('config.reset')}
                </Button>
              </Space>
            </Form.Item>
          </Form>
        </Card>
      ),
    },
    {
      key: 'cdna',
      label: t('config.cdna'),
      children: (
        <Card>
          <CDNAScalesEditor
            config={cdnaConfig}
            onChange={(scales) => setCdnaConfig({ ...cdnaConfig, scales })}
          />
          <div style={{ marginTop: 24 }}>
            <Space>
              <Button
                type="primary"
                icon={<SaveOutlined />}
                onClick={handleSaveCDNA}
                loading={saving}
              >
                {t('config.save')}
              </Button>
              <Button icon={<UndoOutlined />} onClick={handleResetCDNA}>
                {t('config.reset')}
              </Button>
            </Space>
          </div>
        </Card>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <h1>{t('config.title')}</h1>
      <Tabs items={items} />
    </div>
  );
}
