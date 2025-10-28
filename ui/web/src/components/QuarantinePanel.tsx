import { useEffect, useState } from 'react'

interface QuarantineMetrics {
  memoryGrowth: number
  connectionBreaks: number
  tokenChurn: number
}

interface QuarantinePanelProps {
  isActive: boolean
  timeLeft: number
  progress: number
  onApply: () => void
  onCancel: () => void
}

export default function QuarantinePanel({ isActive, timeLeft, progress, onApply, onCancel }: QuarantinePanelProps) {
  const [metrics, setMetrics] = useState<QuarantineMetrics>({
    memoryGrowth: 0,
    connectionBreaks: 0,
    tokenChurn: 0
  })

  useEffect(() => {
    if (!isActive) return

    // Simulate real-time metrics updates
    const interval = setInterval(() => {
      setMetrics({
        memoryGrowth: Math.floor(Math.random() * 18) + 3,
        connectionBreaks: Math.floor(Math.random() * 8) + 1,
        tokenChurn: Math.floor(Math.random() * 25) + 5
      })
    }, 500)

    return () => clearInterval(interval)
  }, [isActive])

  if (!isActive) return null

  const getMetricStatus = (value: number, threshold: number): 'ok' | 'warning' | 'error' => {
    if (value < threshold * 0.7) return 'ok'
    if (value < threshold) return 'warning'
    return 'error'
  }

  return (
    <div className="cdna-quarantine-panel animate-slide-down">
      <div className="cdna-quarantine-header">
        <div className="cdna-quarantine-icon pulsing">⏱️</div>
        <div className="cdna-quarantine-info">
          <h3>Режим карантина активен</h3>
          <p>
            Тестирование новой конфигурации: <span className="time-highlight">{timeLeft}</span>с осталось
          </p>
        </div>
      </div>

      <div className="cdna-progress-bar">
        <div
          className="cdna-progress-fill"
          style={{ width: `${progress}%` }}
        />
        <div className="progress-percentage">{Math.round(progress)}%</div>
      </div>

      <div className="cdna-metrics-grid">
        <div className={`cdna-metric-card ${getMetricStatus(metrics.memoryGrowth, 20)}`}>
          <div className="cdna-metric-value animate-number">+{metrics.memoryGrowth}%</div>
          <div className="cdna-metric-label">Рост памяти</div>
          <div className={`cdna-metric-status ${getMetricStatus(metrics.memoryGrowth, 20)}`}>
            {getMetricStatus(metrics.memoryGrowth, 20) === 'ok' ? '✓' : '⚠'}
            {getMetricStatus(metrics.memoryGrowth, 20) === 'ok' ? ' OK' : ' Риск'} (&lt; 20%)
          </div>
        </div>
        <div className={`cdna-metric-card ${getMetricStatus(metrics.connectionBreaks, 10)}`}>
          <div className="cdna-metric-value animate-number">{metrics.connectionBreaks}</div>
          <div className="cdna-metric-label">Разрывы связей</div>
          <div className={`cdna-metric-status ${getMetricStatus(metrics.connectionBreaks, 10)}`}>
            {getMetricStatus(metrics.connectionBreaks, 10) === 'ok' ? '✓' : '⚠'}
            {getMetricStatus(metrics.connectionBreaks, 10) === 'ok' ? ' OK' : ' Риск'} (&lt; 10)
          </div>
        </div>
        <div className={`cdna-metric-card ${getMetricStatus(metrics.tokenChurn, 30)}`}>
          <div className="cdna-metric-value animate-number">{metrics.tokenChurn}%</div>
          <div className="cdna-metric-label">Обновление токенов</div>
          <div className={`cdna-metric-status ${getMetricStatus(metrics.tokenChurn, 30)}`}>
            {getMetricStatus(metrics.tokenChurn, 30) === 'ok' ? '✓' : '⚠'}
            {getMetricStatus(metrics.tokenChurn, 30) === 'ok' ? ' OK' : ' Риск'} (&lt; 30%)
          </div>
        </div>
      </div>

      <div className="quarantine-actions">
        <button className="button" onClick={onApply}>
          <span>✓</span> Применить сейчас
        </button>
        <button className="button danger" onClick={onCancel}>
          <span>✕</span> Отменить
        </button>
      </div>
    </div>
  )
}
