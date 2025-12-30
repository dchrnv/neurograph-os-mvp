/**
 * Danger Zone Component
 * Destructive system operations with confirmations
 */

import { Card, Button, Space, Modal, message, Typography } from 'antd';
import { ExclamationCircleOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { api } from '../services/api';

const { Text, Paragraph } = Typography;
const { confirm } = Modal;

export default function DangerZone() {
  const { t } = useTranslation();
  const [clearingTokens, setClearingTokens] = useState(false);
  const [resetting, setResetting] = useState(false);

  const handleClearTokens = () => {
    confirm({
      title: t('admin.dangerZone.clearTokensConfirm'),
      icon: <ExclamationCircleOutlined />,
      content: t('admin.dangerZone.clearTokensWarning'),
      okText: t('common.confirm'),
      okType: 'danger',
      cancelText: t('common.cancel'),
      onOk: async () => {
        setClearingTokens(true);
        try {
          await api.clearTokens();
          message.success(t('admin.dangerZone.clearTokensSuccess'));
        } catch (error) {
          message.error(t('admin.dangerZone.clearTokensError'));
          console.error('Clear tokens error:', error);
        } finally {
          setClearingTokens(false);
        }
      },
    });
  };

  const handleResetSystem = () => {
    confirm({
      title: t('admin.dangerZone.resetSystemConfirm'),
      icon: <ExclamationCircleOutlined />,
      content: (
        <Space direction="vertical">
          <Paragraph style={{ margin: 0 }}>
            {t('admin.dangerZone.resetSystemWarning')}
          </Paragraph>
          <Paragraph strong style={{ margin: 0, color: '#ff4d4f' }}>
            {t('admin.dangerZone.resetSystemFinalWarning')}
          </Paragraph>
        </Space>
      ),
      okText: t('admin.dangerZone.resetSystemConfirmText'),
      okType: 'danger',
      cancelText: t('common.cancel'),
      onOk: () => {
        return new Promise((resolve, reject) => {
          confirm({
            title: t('admin.dangerZone.resetSystemDoubleConfirm'),
            icon: <ExclamationCircleOutlined />,
            content: t('admin.dangerZone.resetSystemDoubleConfirmText'),
            okText: t('admin.dangerZone.resetSystemFinalConfirmText'),
            okType: 'danger',
            cancelText: t('common.cancel'),
            onOk: async () => {
              setResetting(true);
              try {
                await api.resetSystem();
                message.success(t('admin.dangerZone.resetSystemSuccess'));
                // Reload page after reset
                setTimeout(() => {
                  window.location.reload();
                }, 2000);
                resolve(true);
              } catch (error) {
                message.error(t('admin.dangerZone.resetSystemError'));
                console.error('Reset system error:', error);
                reject(error);
              } finally {
                setResetting(false);
              }
            },
            onCancel: () => reject(new Error('Cancelled')),
          });
        });
      },
    });
  };

  return (
    <Card
      title={
        <Space>
          <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />
          <Text style={{ color: '#ff4d4f' }}>{t('admin.dangerZone.title')}</Text>
        </Space>
      }
      style={{ border: '2px solid #ff4d4f' }}
    >
      <Space direction="vertical" style={{ width: '100%' }} size="middle">
        <div>
          <Paragraph type="danger">
            {t('admin.dangerZone.description')}
          </Paragraph>
        </div>

        <Space wrap>
          <Button
            danger
            icon={<DeleteOutlined />}
            onClick={handleClearTokens}
            loading={clearingTokens}
          >
            {t('admin.dangerZone.clearTokens')}
          </Button>

          <Button
            danger
            type="primary"
            icon={<ReloadOutlined />}
            onClick={handleResetSystem}
            loading={resetting}
          >
            {t('admin.dangerZone.resetSystem')}
          </Button>
        </Space>
      </Space>
    </Card>
  );
}
