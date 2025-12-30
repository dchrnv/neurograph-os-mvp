/**
 * CDNA Profile Selector Component
 * Allows selection of predefined CDNA profiles
 */

import { Select, Card, Space, Typography, Row, Col } from 'antd';
import type { CDNAScales } from '../types/metrics';

const { Text, Paragraph } = Typography;
const { Option } = Select;

export interface CDNAProfile {
  name: string;
  description: string;
  scales: CDNAScales;
}

export const CDNA_PROFILES: Record<string, CDNAProfile> = {
  balanced: {
    name: 'Balanced',
    description: 'All dimensions equally weighted for general-purpose use',
    scales: {
      physical: 1.0,
      sensory: 1.0,
      motor: 1.0,
      emotional: 1.0,
      cognitive: 1.0,
      social: 1.0,
      temporal: 1.0,
      abstract: 1.0,
    },
  },
  explorer: {
    name: 'Explorer',
    description: 'Enhanced abstract and cognitive dimensions for research and discovery',
    scales: {
      physical: 0.8,
      sensory: 0.9,
      motor: 0.8,
      emotional: 0.9,
      cognitive: 1.5,
      social: 1.0,
      temporal: 1.1,
      abstract: 1.6,
    },
  },
  focused: {
    name: 'Focused',
    description: 'Higher sensory and motor for precise, action-oriented tasks',
    scales: {
      physical: 1.2,
      sensory: 1.5,
      motor: 1.5,
      emotional: 0.8,
      cognitive: 1.1,
      social: 0.7,
      temporal: 1.2,
      abstract: 0.9,
    },
  },
  creative: {
    name: 'Creative',
    description: 'Emphasizes emotional and social dimensions for creative work',
    scales: {
      physical: 0.9,
      sensory: 1.1,
      motor: 0.9,
      emotional: 1.6,
      cognitive: 1.2,
      social: 1.5,
      temporal: 1.0,
      abstract: 1.3,
    },
  },
};

interface CDNAProfileSelectorProps {
  value: string;
  onChange: (profile: string, scales: CDNAScales) => void;
}

export default function CDNAProfileSelector({ value, onChange }: CDNAProfileSelectorProps) {
  const handleProfileChange = (profileKey: string) => {
    const profile = CDNA_PROFILES[profileKey];
    if (profile) {
      onChange(profileKey, profile.scales);
    }
  };

  const selectedProfile = CDNA_PROFILES[value] || CDNA_PROFILES.balanced;

  return (
    <Space direction="vertical" style={{ width: '100%' }} size="middle">
      <div>
        <Text strong style={{ marginRight: 8 }}>Profile:</Text>
        <Select
          value={value}
          onChange={handleProfileChange}
          style={{ width: 200 }}
        >
          {Object.entries(CDNA_PROFILES).map(([key, profile]) => (
            <Option key={key} value={key}>
              {profile.name}
            </Option>
          ))}
        </Select>
      </div>

      <Card size="small" style={{ background: '#f5f5f5' }}>
        <Paragraph style={{ margin: 0, color: '#666' }}>
          {selectedProfile.description}
        </Paragraph>
      </Card>

      <Card size="small" title="Profile Scale Values">
        <Row gutter={[16, 8]}>
          {Object.entries(selectedProfile.scales).map(([dimension, value]) => (
            <Col key={dimension} xs={12} sm={8} md={6}>
              <Text type="secondary" style={{ textTransform: 'capitalize' }}>
                {dimension}:
              </Text>{' '}
              <Text strong>{value.toFixed(1)}</Text>
            </Col>
          ))}
        </Row>
      </Card>
    </Space>
  );
}
