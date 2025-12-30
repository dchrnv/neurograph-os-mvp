/**
 * Loading Screen Component
 * Full-screen loading indicator
 */

import { Spin } from 'antd';
import { LoadingOutlined } from '@ant-design/icons';

interface LoadingScreenProps {
  tip?: string;
}

export default function LoadingScreen({ tip = 'Loading...' }: LoadingScreenProps) {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        justifyContent: 'center',
        alignItems: 'center',
        minHeight: '100vh',
        background: 'var(--bg-color, #f0f2f5)',
      }}
    >
      <Spin
        indicator={<LoadingOutlined style={{ fontSize: 48 }} spin />}
        tip={tip}
        size="large"
      />
    </div>
  );
}
