/**
 * Dashboard page - main system overview
 */

import { Row, Col, Card, Table, Tag, Space } from 'antd';
import { useTranslation } from 'react-i18next';
import { useEffect } from 'react';
import {
  DatabaseOutlined,
  ApiOutlined,
  ThunderboltOutlined,
  ClockCircleOutlined,
} from '@ant-design/icons';
import MetricCard from '../components/MetricCard';
import { useSystemStore } from '../stores/systemStore';
import { api } from '../services/api';
import { ws } from '../services/websocket';
import { WS_CHANNELS } from '../utils/constants';
import { formatNumber, formatDuration, formatUptime } from '../utils/formatters';
import type { ActivityEvent } from '../types/api';

export default function Dashboard() {
  const { t } = useTranslation();
  const { status, metrics, activities, setStatus, setMetrics, addActivity, addMetricsHistory } = useSystemStore();

  useEffect(() => {
    // Initial data fetch
    loadDashboardData();

    // Subscribe to WebSocket updates
    ws.subscribe(WS_CHANNELS.METRICS, handleMetricsUpdate);
    ws.subscribe(WS_CHANNELS.ACTIVITY, handleActivityUpdate);

    return () => {
      ws.unsubscribe(WS_CHANNELS.METRICS, handleMetricsUpdate);
      ws.unsubscribe(WS_CHANNELS.ACTIVITY, handleActivityUpdate);
    };
  }, []);

  const loadDashboardData = async () => {
    try {
      const [statusData, metricsData] = await Promise.all([
        api.getHealth(),
        api.getMetrics(),
      ]);
      setStatus(statusData);
      setMetrics(metricsData);
      addMetricsHistory(metricsData);
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    }
  };

  const handleMetricsUpdate = (data: any) => {
    setMetrics(data);
    addMetricsHistory(data);
  };

  const handleActivityUpdate = (data: ActivityEvent) => {
    addActivity(data);
  };

  const activityColumns = [
    {
      title: t('dashboard.activity.time'),
      dataIndex: 'time',
      key: 'time',
      width: 100,
      render: (time: string) => new Date(time).toLocaleTimeString(),
    },
    {
      title: t('dashboard.activity.event'),
      dataIndex: 'event',
      key: 'event',
      width: 120,
      render: (event: string) => {
        const colors: Record<string, string> = {
          query: 'blue',
          feedback: 'green',
          module: 'orange',
          system: 'red',
        };
        return <Tag color={colors[event] || 'default'}>{event}</Tag>;
      },
    },
    {
      title: t('dashboard.activity.details'),
      dataIndex: 'details',
      key: 'details',
      ellipsis: true,
    },
    {
      title: t('dashboard.activity.duration'),
      dataIndex: 'duration_ms',
      key: 'duration',
      width: 100,
      render: (ms?: number) => (ms ? formatDuration(ms) : '-'),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <h1>{t('dashboard.title')}</h1>

      {/* System Status */}
      {status && (
        <Card size="small" style={{ marginBottom: 24 }}>
          <Space size="large">
            <Tag color={status.status === 'running' ? 'success' : 'error'}>
              {t(`dashboard.status.${status.status}`)}
            </Tag>
            <span>
              <ClockCircleOutlined /> {t('dashboard.status.uptime')}:{' '}
              {formatUptime(status.uptime)}
            </span>
            <span>Version: {status.version}</span>
          </Space>
        </Card>
      )}

      {/* Main Metrics */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={12} lg={6}>
          <MetricCard
            title={t('dashboard.metrics.tokens')}
            value={formatNumber(metrics?.tokens || 0)}
            icon={<DatabaseOutlined />}
            color="#1890ff"
          />
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <MetricCard
            title={t('dashboard.metrics.connections')}
            value={metrics?.connections || 0}
            icon={<ApiOutlined />}
            color="#52c41a"
          />
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <MetricCard
            title={t('dashboard.metrics.queriesPerHour')}
            value={formatNumber(metrics?.queries_per_hour || 0)}
            icon={<ThunderboltOutlined />}
            color="#faad14"
          />
        </Col>
        <Col xs={24} sm={12} lg={6}>
          <MetricCard
            title={t('dashboard.metrics.eventsPerSec')}
            value={metrics?.events_per_sec || 0}
            suffix="/s"
            icon={<ClockCircleOutlined />}
            color="#f5222d"
          />
        </Col>
      </Row>

      {/* Performance Metrics */}
      <Card title={t('dashboard.performance.title')} style={{ marginBottom: 24 }}>
        <Row gutter={[16, 16]}>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.latency')}
              value={metrics?.avg_latency_us || 0}
              suffix="μs"
              color="#1890ff"
            />
          </Col>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.fastPath')}
              value={metrics?.fast_path_percent || 0}
              suffix="%"
              progress={metrics?.fast_path_percent}
              color="#52c41a"
            />
          </Col>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.cacheHit')}
              value={metrics?.cache_hit_percent || 0}
              suffix="%"
              progress={metrics?.cache_hit_percent}
              color="#722ed1"
            />
          </Col>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.cpu')}
              value={metrics?.cpu_percent || 0}
              suffix="%"
              progress={metrics?.cpu_percent}
              color="#faad14"
            />
          </Col>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.memory')}
              value={metrics?.memory_percent || 0}
              suffix="%"
              progress={metrics?.memory_percent}
              color="#13c2c2"
            />
          </Col>
          <Col xs={24} sm={12} lg={8}>
            <MetricCard
              title={t('dashboard.performance.disk')}
              value={metrics?.disk_percent || 0}
              suffix="%"
              progress={metrics?.disk_percent}
              color="#eb2f96"
            />
          </Col>
        </Row>
      </Card>

      {/* Recent Activity */}
      <Card title={t('dashboard.activity.title')}>
        <Table
          dataSource={activities}
          columns={activityColumns}
          pagination={{ pageSize: 10 }}
          rowKey={(record) => `${record.time}-${record.event}`}
          locale={{ emptyText: t('dashboard.activity.empty') }}
        />
      </Card>
    </div>
  );
}
