/**
 * CDNA Scales Editor - visual editor for cognitive dimension scales
 */

import { Card, Slider, Row, Col, Typography, Tag, Space } from 'antd';
import type { CDNAConfig } from '../types/metrics';

const { Title, Text } = Typography;

interface CDNAScalesEditorProps {
  config: CDNAConfig;
  onChange: (scales: CDNAConfig['scales']) => void;
  readonly?: boolean;
}

const SCALE_INFO = {
  physical: {
    label: 'Physical',
    description: 'Body awareness, movement, spatial reasoning',
    color: '#ff4d4f',
  },
  sensory: {
    label: 'Sensory',
    description: 'Perception, pattern recognition, sensory processing',
    color: '#faad14',
  },
  motor: {
    label: 'Motor',
    description: 'Action execution, coordination, skill learning',
    color: '#52c41a',
  },
  emotional: {
    label: 'Emotional',
    description: 'Affect, motivation, emotional intelligence',
    color: '#1890ff',
  },
  cognitive: {
    label: 'Cognitive',
    description: 'Reasoning, problem solving, planning',
    color: '#722ed1',
  },
  social: {
    label: 'Social',
    description: 'Communication, empathy, theory of mind',
    color: '#eb2f96',
  },
  temporal: {
    label: 'Temporal',
    description: 'Time perception, sequence, temporal reasoning',
    color: '#13c2c2',
  },
  abstract: {
    label: 'Abstract',
    description: 'Symbolic thought, metacognition, generalization',
    color: '#2f54eb',
  },
};

export default function CDNAScalesEditor({
  config,
  onChange,
  readonly = false,
}: CDNAScalesEditorProps) {
  const handleScaleChange = (scale: keyof CDNAConfig['scales'], value: number) => {
    onChange({
      ...config.scales,
      [scale]: value,
    });
  };

  const getProfileTag = () => {
    const colors = {
      balanced: 'blue',
      explorer: 'green',
      focused: 'orange',
      creative: 'purple',
    };
    return <Tag color={colors[config.profile]}>{config.profile.toUpperCase()}</Tag>;
  };

  return (
    <div>
      <Space style={{ marginBottom: 24 }}>
        <Title level={4} style={{ margin: 0 }}>CDNA Scales</Title>
        {getProfileTag()}
      </Space>

      <Row gutter={[24, 24]}>
        {Object.entries(SCALE_INFO).map(([key, info]) => {
          const scaleKey = key as keyof CDNAConfig['scales'];
          const value = config.scales[scaleKey];

          return (
            <Col xs={24} md={12} key={key}>
              <Card size="small">
                <div style={{ marginBottom: 8 }}>
                  <Text strong style={{ color: info.color }}>
                    {info.label}
                  </Text>
                  <Text type="secondary" style={{ marginLeft: 8, fontSize: 12 }}>
                    ({value})
                  </Text>
                </div>
                <Text type="secondary" style={{ fontSize: 12, display: 'block', marginBottom: 12 }}>
                  {info.description}
                </Text>
                <Slider
                  min={0}
                  max={100}
                  value={value}
                  onChange={(val) => handleScaleChange(scaleKey, val)}
                  disabled={readonly}
                  trackStyle={{ backgroundColor: info.color }}
                  handleStyle={{ borderColor: info.color }}
                  marks={{
                    0: '0',
                    50: '50',
                    100: '100',
                  }}
                />
              </Card>
            </Col>
          );
        })}
      </Row>
    </div>
  );
}
