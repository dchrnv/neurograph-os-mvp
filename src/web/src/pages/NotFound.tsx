/**
 * 404 Not Found Page
 */

import { Result, Button } from 'antd';
import { HomeOutlined } from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

export default function NotFound() {
  const navigate = useNavigate();
  const { t } = useTranslation();

  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        minHeight: '100vh',
        padding: 24,
      }}
    >
      <Result
        status="404"
        title="404"
        subTitle={t('notFound.subtitle') || 'Sorry, the page you visited does not exist.'}
        extra={
          <Button
            type="primary"
            icon={<HomeOutlined />}
            onClick={() => navigate('/')}
          >
            {t('notFound.backHome') || 'Back Home'}
          </Button>
        }
      />
    </div>
  );
}
