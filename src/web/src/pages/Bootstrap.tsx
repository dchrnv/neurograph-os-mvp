/**
 * Bootstrap page - system initialization and setup
 */

import { useState, useEffect } from 'react';
import { Card, Steps, Button, Progress, Tag, Space, Alert, Typography, List } from 'antd';
import {
  CheckCircleOutlined,
  LoadingOutlined,
  CloseCircleOutlined,
  PlayCircleOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import { useTranslation } from 'react-i18next';

const { Title, Text } = Typography;

interface BootstrapStep {
  name: string;
  description: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  duration?: number;
  error?: string;
}

const INITIAL_STEPS: BootstrapStep[] = [
  {
    name: 'Database Initialization',
    description: 'Initialize graph database and create schema',
    status: 'pending',
  },
  {
    name: 'Load Modules',
    description: 'Load and register all cognitive modules',
    status: 'pending',
  },
  {
    name: 'CDNA Configuration',
    description: 'Apply CDNA scales configuration',
    status: 'pending',
  },
  {
    name: 'Signal Engine',
    description: 'Start signal engine and event bus',
    status: 'pending',
  },
  {
    name: 'WebSocket Server',
    description: 'Initialize WebSocket connections',
    status: 'pending',
  },
  {
    name: 'System Ready',
    description: 'Final checks and system ready',
    status: 'pending',
  },
];

export default function Bootstrap() {
  const { t } = useTranslation();
  const [steps, setSteps] = useState<BootstrapStep[]>(INITIAL_STEPS);
  const [isBootstrapping, setIsBootstrapping] = useState(false);
  const [currentStep, setCurrentStep] = useState(0);
  const [logs, setLogs] = useState<string[]>([]);

  const addLog = (message: string) => {
    const timestamp = new Date().toLocaleTimeString();
    setLogs((prev) => [`[${timestamp}] ${message}`, ...prev.slice(0, 49)]);
  };

  const simulateBootstrap = async () => {
    setIsBootstrapping(true);
    setSteps(INITIAL_STEPS);
    setCurrentStep(0);
    setLogs([]);
    addLog('Bootstrap process started');

    for (let i = 0; i < INITIAL_STEPS.length; i++) {
      setCurrentStep(i);

      // Update step to running
      setSteps((prev) =>
        prev.map((step, idx) =>
          idx === i ? { ...step, status: 'running' } : step
        )
      );

      addLog(`Starting: ${INITIAL_STEPS[i].name}`);

      // Simulate processing time
      const duration = Math.random() * 2000 + 1000;
      await new Promise((resolve) => setTimeout(resolve, duration));

      // Randomly fail on some steps (10% chance)
      const shouldFail = Math.random() < 0.1 && i > 0;

      if (shouldFail) {
        setSteps((prev) =>
          prev.map((step, idx) =>
            idx === i
              ? {
                  ...step,
                  status: 'failed',
                  error: 'Connection timeout',
                  duration: Math.round(duration),
                }
              : step
          )
        );
        addLog(`❌ Failed: ${INITIAL_STEPS[i].name} - Connection timeout`);
        setIsBootstrapping(false);
        return;
      }

      // Update step to completed
      setSteps((prev) =>
        prev.map((step, idx) =>
          idx === i
            ? {
                ...step,
                status: 'completed',
                duration: Math.round(duration),
              }
            : step
        )
      );

      addLog(`✅ Completed: ${INITIAL_STEPS[i].name} (${Math.round(duration)}ms)`);
    }

    addLog('🎉 Bootstrap completed successfully!');
    setIsBootstrapping(false);
  };

  const handleReset = () => {
    setSteps(INITIAL_STEPS);
    setCurrentStep(0);
    setLogs([]);
    setIsBootstrapping(false);
  };

  const getStepIcon = (status: BootstrapStep['status']) => {
    switch (status) {
      case 'completed':
        return <CheckCircleOutlined style={{ color: '#52c41a' }} />;
      case 'running':
        return <LoadingOutlined />;
      case 'failed':
        return <CloseCircleOutlined style={{ color: '#ff4d4f' }} />;
      default:
        return null;
    }
  };

  const completedSteps = steps.filter((s) => s.status === 'completed').length;
  const progress = Math.round((completedSteps / steps.length) * 100);
  const hasFailure = steps.some((s) => s.status === 'failed');
  const isComplete = completedSteps === steps.length;

  return (
    <div style={{ padding: 24 }}>
      <Title level={2}>System Bootstrap</Title>
      <Text type="secondary">
        Initialize NeuroGraph system components and configuration
      </Text>

      <Card style={{ marginTop: 24 }}>
        <Space direction="vertical" style={{ width: '100%' }} size="large">
          {/* Progress */}
          <div>
            <div style={{ marginBottom: 8 }}>
              <Text strong>Overall Progress:</Text>
              <Text style={{ marginLeft: 16 }}>
                {completedSteps} / {steps.length} steps completed
              </Text>
            </div>
            <Progress
              percent={progress}
              status={hasFailure ? 'exception' : isComplete ? 'success' : 'active'}
              strokeColor={hasFailure ? '#ff4d4f' : isComplete ? '#52c41a' : '#1890ff'}
            />
          </div>

          {/* Status Alert */}
          {hasFailure && (
            <Alert
              message="Bootstrap Failed"
              description="One or more steps failed during bootstrap. Check logs below for details."
              type="error"
              showIcon
            />
          )}
          {isComplete && !hasFailure && (
            <Alert
              message="Bootstrap Complete"
              description="All system components initialized successfully!"
              type="success"
              showIcon
            />
          )}

          {/* Actions */}
          <Space>
            <Button
              type="primary"
              icon={<PlayCircleOutlined />}
              onClick={simulateBootstrap}
              disabled={isBootstrapping}
              loading={isBootstrapping}
            >
              Start Bootstrap
            </Button>
            <Button icon={<ReloadOutlined />} onClick={handleReset} disabled={isBootstrapping}>
              Reset
            </Button>
          </Space>

          {/* Steps */}
          <Steps
            direction="vertical"
            current={currentStep}
            items={steps.map((step, idx) => ({
              title: step.name,
              description: (
                <div>
                  <div>{step.description}</div>
                  {step.status === 'completed' && step.duration && (
                    <Tag color="blue" style={{ marginTop: 4 }}>
                      {step.duration}ms
                    </Tag>
                  )}
                  {step.status === 'failed' && step.error && (
                    <Tag color="red" style={{ marginTop: 4 }}>
                      {step.error}
                    </Tag>
                  )}
                </div>
              ),
              status:
                step.status === 'completed'
                  ? 'finish'
                  : step.status === 'failed'
                  ? 'error'
                  : step.status === 'running'
                  ? 'process'
                  : 'wait',
              icon: getStepIcon(step.status),
            }))}
          />

          {/* Logs */}
          {logs.length > 0 && (
            <Card title="Bootstrap Logs" size="small">
              <List
                dataSource={logs}
                renderItem={(log) => (
                  <List.Item style={{ padding: '4px 0', border: 'none' }}>
                    <Text code style={{ fontSize: 12, width: '100%' }}>
                      {log}
                    </Text>
                  </List.Item>
                )}
                style={{ maxHeight: 300, overflow: 'auto' }}
              />
            </Card>
          )}
        </Space>
      </Card>
    </div>
  );
}
