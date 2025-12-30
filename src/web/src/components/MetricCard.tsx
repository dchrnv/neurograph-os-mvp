/**
 * Metric card component for displaying system metrics
 */

import { Card, Statistic, Progress } from 'antd';
import { ArrowUpOutlined, ArrowDownOutlined } from '@ant-design/icons';
import type { ReactNode } from 'react';

interface MetricCardProps {
  title: string;
  value: number | string;
  suffix?: string;
  prefix?: ReactNode;
  icon?: ReactNode;
  trend?: number;
  progress?: number;
  color?: string;
  loading?: boolean;
}

export default function MetricCard({
  title,
  value,
  suffix,
  prefix,
  icon,
  trend,
  progress,
  color = '#1890ff',
  loading = false,
}: MetricCardProps) {
  const getTrendIcon = () => {
    if (trend === undefined) return null;
    if (trend > 0) {
      return <ArrowUpOutlined style={{ color: '#52c41a' }} />;
    }
    if (trend < 0) {
      return <ArrowDownOutlined style={{ color: '#ff4d4f' }} />;
    }
    return null;
  };

  return (
    <Card loading={loading} bordered={false} style={{ height: '100%' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
        <Statistic
          title={title}
          value={value}
          suffix={suffix}
          prefix={prefix}
          valueStyle={{ color }}
        />
        {icon && <div style={{ fontSize: 32, color, opacity: 0.6 }}>{icon}</div>}
      </div>

      {trend !== undefined && (
        <div style={{ marginTop: 8, fontSize: 14 }}>
          {getTrendIcon()} {Math.abs(trend)}%
        </div>
      )}

      {progress !== undefined && (
        <Progress
          percent={progress}
          strokeColor={color}
          showInfo={false}
          style={{ marginTop: 8 }}
        />
      )}
    </Card>
  );
}
