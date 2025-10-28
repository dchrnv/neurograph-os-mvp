import { useEffect, useState } from 'react'

interface Warning {
  id: string
  type: 'info' | 'warning' | 'danger'
  message: string
  dimension?: string
  value?: number
}

interface WarningSystemProps {
  currentValues: number[]
  dimensionNames: string[]
  dimensionLimits: Array<{ min: number; max: number; green: [number, number]; yellow: [number, number] }>
}

export default function WarningSystem({ currentValues, dimensionNames, dimensionLimits }: WarningSystemProps) {
  const [warnings, setWarnings] = useState<Warning[]>([])
  const [dismissed, setDismissed] = useState<Set<string>>(new Set())

  useEffect(() => {
    const newWarnings: Warning[] = []

    currentValues.forEach((value, index) => {
      const limit = dimensionLimits[index]
      const dimension = dimensionNames[index]

      // Danger zone (red)
      if (value > limit.yellow[1]) {
        newWarnings.push({
          id: `danger-${index}`,
          type: 'danger',
          message: `${dimension} в опасной зоне!`,
          dimension,
          value
        })
      }
      // Warning zone (yellow)
      else if (value > limit.green[1]) {
        newWarnings.push({
          id: `warning-${index}`,
          type: 'warning',
          message: `${dimension} в зоне риска`,
          dimension,
          value
        })
      }
      // Too low
      else if (value < limit.green[0] && value > 0) {
        newWarnings.push({
          id: `low-${index}`,
          type: 'info',
          message: `${dimension} ниже рекомендуемого`,
          dimension,
          value
        })
      }
    })

    // Check for extreme configurations
    const maxValue = Math.max(...currentValues)
    const avgValue = currentValues.reduce((a, b) => a + b, 0) / currentValues.length

    if (maxValue > 40) {
      newWarnings.push({
        id: 'extreme-max',
        type: 'danger',
        message: 'Обнаружено экстремальное значение! Возможна нестабильность системы.'
      })
    }

    if (avgValue > 15) {
      newWarnings.push({
        id: 'high-avg',
        type: 'warning',
        message: 'Средний уровень высокий. Рассмотрите использование профиля Analyzer.'
      })
    }

    setWarnings(newWarnings.filter(w => !dismissed.has(w.id)))
  }, [currentValues, dimensionNames, dimensionLimits, dismissed])

  const dismissWarning = (id: string) => {
    setDismissed(prev => new Set(prev).add(id))
  }

  const clearAll = () => {
    setDismissed(new Set(warnings.map(w => w.id)))
  }

  if (warnings.length === 0) {
    return (
      <div className="warning-system empty">
        <div className="no-warnings">
          ✅ Все параметры в безопасных пределах
        </div>
      </div>
    )
  }

  return (
    <div className="warning-system">
      <div className="warning-header">
        <span>⚠️ Предупреждения ({warnings.length})</span>
        <button className="btn-clear-warnings" onClick={clearAll}>
          Скрыть все
        </button>
      </div>
      <div className="warning-list">
        {warnings.map((warning) => (
          <div key={warning.id} className={`warning-item ${warning.type}`}>
            <div className="warning-content">
              <div className="warning-icon">
                {warning.type === 'danger' ? '🔴' : warning.type === 'warning' ? '🟡' : '🔵'}
              </div>
              <div className="warning-text">
                <div className="warning-message">{warning.message}</div>
                {warning.dimension && warning.value && (
                  <div className="warning-details">
                    {warning.dimension}: {warning.value.toFixed(1)}
                  </div>
                )}
              </div>
            </div>
            <button
              className="btn-dismiss"
              onClick={() => dismissWarning(warning.id)}
              title="Скрыть"
            >
              ✕
            </button>
          </div>
        ))}
      </div>
    </div>
  )
}
