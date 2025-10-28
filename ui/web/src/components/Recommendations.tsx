interface Recommendation {
  id: string
  type: 'optimization' | 'warning' | 'tip'
  title: string
  description: string
  action?: string
  onApply?: () => void
}

interface RecommendationsProps {
  currentValues: number[]
  currentProfile: string
}

export default function Recommendations({ currentValues, currentProfile }: RecommendationsProps) {
  const generateRecommendations = (): Recommendation[] => {
    const recommendations: Recommendation[] = []

    const avgValue = currentValues.reduce((a, b) => a + b, 0) / currentValues.length
    const maxValue = Math.max(...currentValues)
    const minValue = Math.min(...currentValues.filter(v => v > 0))

    // Check for imbalanced configuration
    if (maxValue / minValue > 10) {
      recommendations.push({
        id: 'imbalance',
        type: 'warning',
        title: 'Несбалансированная конфигурация',
        description: `Разброс значений слишком велик (${minValue.toFixed(1)} - ${maxValue.toFixed(1)}). Это может привести к неравномерной обработке.`,
        action: 'Нормализовать значения'
      })
    }

    // Profile-specific recommendations
    if (currentProfile === 'creative' && avgValue < 10) {
      recommendations.push({
        id: 'creative-low',
        type: 'tip',
        title: 'Низкие значения для Creative профиля',
        description: 'Creative профиль обычно использует более высокие значения. Рассмотрите увеличение Abstract и Cognitive измерений.',
        action: 'Увеличить ключевые измерения'
      })
    }

    if (currentProfile === 'analyzer' && currentValues[7] < 15) {
      recommendations.push({
        id: 'analyzer-abstract',
        type: 'optimization',
        title: 'Рекомендация для Analyzer',
        description: 'Analyzer профиль работает лучше с высоким Abstract значением (15-20) для строгой валидации.',
        action: 'Увеличить Abstract до 20'
      })
    }

    // General optimizations
    if (currentValues[4] > 25) { // Cognitive
      recommendations.push({
        id: 'cognitive-high',
        type: 'warning',
        title: 'Высокая когнитивная нагрузка',
        description: 'Cognitive dimension > 25 может замедлить обработку. Снизьте до 15-20 для оптимальной производительности.',
        action: 'Оптимизировать Cognitive'
      })
    }

    // Add helpful tips
    if (recommendations.length === 0) {
      recommendations.push({
        id: 'optimal',
        type: 'tip',
        title: 'Конфигурация оптимальна',
        description: 'Текущие настройки находятся в рекомендуемых диапазонах. Используйте карантин для тестирования новых изменений.',
        action: null
      })
    }

    return recommendations
  }

  const recommendations = generateRecommendations()

  return (
    <div className="recommendations-panel">
      <div className="recommendations-header">
        <h3>💡 Рекомендации</h3>
      </div>
      <div className="recommendations-list">
        {recommendations.map((rec) => (
          <div key={rec.id} className={`recommendation-item ${rec.type}`}>
            <div className="recommendation-icon">
              {rec.type === 'optimization' ? '⚡' : rec.type === 'warning' ? '⚠️' : '💡'}
            </div>
            <div className="recommendation-content">
              <h4>{rec.title}</h4>
              <p>{rec.description}</p>
              {rec.action && rec.onApply && (
                <button className="btn-apply-recommendation" onClick={rec.onApply}>
                  {rec.action}
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
