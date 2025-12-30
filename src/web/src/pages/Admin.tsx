/**
 * Admin page - System administration and CDNA configuration
 */

import { useEffect, useState } from 'react';
import { Card, Button, Space, message, Row, Col, Divider } from 'antd';
import {
  ExportOutlined,
  ImportOutlined,
  SaveOutlined,
  DownloadOutlined,
  UploadOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';
import CDNAScalesEditor from '../components/CDNAScalesEditor';
import CDNAProfileSelector from '../components/CDNAProfileSelector';
import DangerZone from '../components/DangerZone';
import { api } from '../services/api';
import type { CDNAScales } from '../types/metrics';

export default function Admin() {
  const { t } = useTranslation();
  const [scales, setScales] = useState<CDNAScales>({
    physical: 1.0,
    sensory: 1.0,
    motor: 1.0,
    emotional: 1.0,
    cognitive: 1.0,
    social: 1.0,
    temporal: 1.0,
    abstract: 1.0,
  });
  const [selectedProfile, setSelectedProfile] = useState('balanced');
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [importing, setImporting] = useState(false);
  const [backingUp, setBackingUp] = useState(false);

  useEffect(() => {
    loadCDNAConfig();
  }, []);

  const loadCDNAConfig = async () => {
    setLoading(true);
    try {
      const config = await api.getCDNAScales();
      setScales(config);
    } catch (error) {
      message.error(t('config.loadError'));
      console.error('Load CDNA config error:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleScaleChange = (dimension: keyof CDNAScales, value: number) => {
    setScales((prev) => ({
      ...prev,
      [dimension]: value,
    }));
  };

  const handleProfileChange = (profile: string, profileScales: CDNAScales) => {
    setSelectedProfile(profile);
    setScales(profileScales);
  };

  const handleSaveScales = async () => {
    setSaving(true);
    try {
      await api.updateCDNAScales(scales);
      message.success(t('admin.cdna.saveSuccess'));
    } catch (error) {
      message.error(t('admin.cdna.saveError'));
      console.error('Save CDNA scales error:', error);
    } finally {
      setSaving(false);
    }
  };

  const handleResetScales = () => {
    setScales({
      physical: 1.0,
      sensory: 1.0,
      motor: 1.0,
      emotional: 1.0,
      cognitive: 1.0,
      social: 1.0,
      temporal: 1.0,
      abstract: 1.0,
    });
    setSelectedProfile('balanced');
    message.info(t('admin.cdna.resetInfo'));
  };

  const handleExportData = async () => {
    setExporting(true);
    try {
      const data = await api.exportData();

      // Download as JSON file
      const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `neurograph_export_${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);

      message.success(t('admin.operations.exportSuccess'));
    } catch (error) {
      message.error(t('admin.operations.exportError'));
      console.error('Export data error:', error);
    } finally {
      setExporting(false);
    }
  };

  const handleImportData = () => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = 'application/json';
    input.onchange = async (e: Event) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;

      setImporting(true);
      try {
        const text = await file.text();
        const data = JSON.parse(text);
        await api.importData(data);
        message.success(t('admin.operations.importSuccess'));
        loadCDNAConfig(); // Reload config after import
      } catch (error) {
        message.error(t('admin.operations.importError'));
        console.error('Import data error:', error);
      } finally {
        setImporting(false);
      }
    };
    input.click();
  };

  const handleCreateBackup = async () => {
    setBackingUp(true);
    try {
      const backup = await api.createBackup();

      // Download backup file
      const blob = new Blob([JSON.stringify(backup, null, 2)], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `neurograph_backup_${Date.now()}.json`;
      a.click();
      URL.revokeObjectURL(url);

      message.success(t('admin.operations.backupSuccess'));
    } catch (error) {
      message.error(t('admin.operations.backupError'));
      console.error('Create backup error:', error);
    } finally {
      setBackingUp(false);
    }
  };

  return (
    <div style={{ padding: 24 }}>
      <h1>{t('admin.title')}</h1>

      {/* CDNA Configuration */}
      <Card
        title={t('admin.cdna.title')}
        style={{ marginBottom: 24 }}
        loading={loading}
      >
        <Space direction="vertical" style={{ width: '100%' }} size="large">
          {/* Profile Selector */}
          <CDNAProfileSelector
            value={selectedProfile}
            onChange={handleProfileChange}
          />

          <Divider />

          {/* Dimension Scales */}
          <div>
            <h3>{t('admin.cdna.dimensionScales')}</h3>
            <CDNAScalesEditor
              scales={scales}
              onChange={handleScaleChange}
              min={0.0}
              max={2.0}
              step={0.1}
            />
          </div>

          {/* Actions */}
          <div style={{ textAlign: 'right' }}>
            <Space>
              <Button onClick={handleResetScales}>
                {t('common.reset')}
              </Button>
              <Button
                type="primary"
                icon={<SaveOutlined />}
                onClick={handleSaveScales}
                loading={saving}
              >
                {t('admin.cdna.apply')}
              </Button>
            </Space>
          </div>
        </Space>
      </Card>

      {/* System Operations */}
      <Card
        title={t('admin.operations.title')}
        style={{ marginBottom: 24 }}
      >
        <Row gutter={[16, 16]}>
          <Col xs={24} sm={12} md={8}>
            <Button
              block
              size="large"
              icon={<ExportOutlined />}
              onClick={handleExportData}
              loading={exporting}
            >
              {t('admin.operations.exportData')}
            </Button>
          </Col>
          <Col xs={24} sm={12} md={8}>
            <Button
              block
              size="large"
              icon={<ImportOutlined />}
              onClick={handleImportData}
              loading={importing}
            >
              {t('admin.operations.importData')}
            </Button>
          </Col>
          <Col xs={24} sm={12} md={8}>
            <Button
              block
              size="large"
              icon={<DownloadOutlined />}
              onClick={handleCreateBackup}
              loading={backingUp}
            >
              {t('admin.operations.createBackup')}
            </Button>
          </Col>
        </Row>
      </Card>

      {/* Danger Zone */}
      <DangerZone />
    </div>
  );
}
