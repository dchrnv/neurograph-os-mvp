import { useState, useRef, useEffect } from 'react'

interface DimensionInfo {
  name: string
  icon: string
  description: string
  examples: string[]
  safeRange: string
  impact: string
}

const dimensionInfo: Record<string, DimensionInfo> = {
  physical: {
    name: 'Physical',
    icon: '🏃',
    description: 'Физическое 3D пространство. Определяет масштаб координат в метрах.',
    examples: ['Позиция объектов', 'Расстояния', 'Траектории движения'],
    safeRange: '1-5',
    impact: 'Влияет на spatial queries и field calculations'
  },
  sensory: {
    name: 'Sensory',
    icon: '👁️',
    description: 'Сенсорное восприятие. Салиентность, валентность, новизна стимулов.',
    examples: ['Яркость', 'Контрастность', 'Внимание'],
    safeRange: '1-5',
    impact: 'Определяет приоритет обработки стимулов'
  },
  motor: {
    name: 'Motor',
    icon: '✋',
    description: 'Моторная активность. Скорость, ускорение, точность движений.',
    examples: ['Velocity', 'Acceleration', 'Precision'],
    safeRange: '1-5',
    impact: 'Влияет на динамику системы'
  },
  emotional: {
    name: 'Emotional',
    icon: '❤️',
    description: 'Эмоциональное состояние по VAD модели (Valence, Arousal, Dominance).',
    examples: ['Радость (0.8, 0.6, 0.7)', 'Страх (0.2, 0.8, 0.3)', 'Спокойствие (0.5, 0.2, 0.5)'],
    safeRange: '1-8',
    impact: 'Модулирует обработку и принятие решений'
  },
  cognitive: {
    name: 'Cognitive',
    icon: '🧠',
    description: 'Когнитивная нагрузка, уровень абстракции, уверенность в решениях.',
    examples: ['Working memory load', 'Abstraction level', 'Confidence'],
    safeRange: '1-15',
    impact: 'Самое большое измерение, критично для обработки'
  },
  social: {
    name: 'Social',
    icon: '👥',
    description: 'Социальное взаимодействие. Дистанция, статус, групповая принадлежность.',
    examples: ['Personal space', 'Authority', 'Group membership'],
    safeRange: '1-10',
    impact: 'Определяет социальный контекст'
  },
  temporal: {
    name: 'Temporal',
    icon: '⏰',
    description: 'Временная локализация. Смещение во времени, длительность, частота.',
    examples: ['Time offset', 'Duration', 'Frequency'],
    safeRange: '1-8',
    impact: 'Управляет временными связями'
  },
  abstract: {
    name: 'Abstract',
    icon: '💭',
    description: 'Семантическое пространство. Близость значений, каузальность, модальность.',
    examples: ['Word embeddings', 'Concept similarity', 'Logic relations'],
    safeRange: '1-20',
    impact: 'Основа для semantic queries, самое гибкое'
  }
}

interface DimensionTooltipProps {
  dimensionId: string
  children: React.ReactNode
}

export default function DimensionTooltip({ dimensionId, children }: DimensionTooltipProps) {
  const [isVisible, setIsVisible] = useState(false)
  const [position, setPosition] = useState<'right' | 'left'>('right')
  const wrapperRef = useRef<HTMLDivElement>(null)
  const info = dimensionInfo[dimensionId]

  useEffect(() => {
    if (isVisible && wrapperRef.current) {
      const rect = wrapperRef.current.getBoundingClientRect()
      const windowWidth = window.innerWidth

      // Если элемент в правой половине экрана - показываем tooltip слева
      if (rect.right > windowWidth / 2) {
        setPosition('left')
      } else {
        setPosition('right')
      }
    }
  }, [isVisible])

  if (!info) return <>{children}</>

  return (
    <div
      ref={wrapperRef}
      className="tooltip-wrapper"
      onMouseEnter={() => setIsVisible(true)}
      onMouseLeave={() => setIsVisible(false)}
    >
      {children}
      {isVisible && (
        <div className={`dimension-tooltip tooltip-${position}`}>
          <div className="tooltip-header">
            <span className="tooltip-icon">{info.icon}</span>
            <span className="tooltip-name">{info.name}</span>
          </div>
          <div className="tooltip-body">
            <p className="tooltip-description">{info.description}</p>

            <div className="tooltip-section">
              <strong>Примеры:</strong>
              <ul>
                {info.examples.map((ex, i) => (
                  <li key={i}>{ex}</li>
                ))}
              </ul>
            </div>

            <div className="tooltip-section">
              <strong>Безопасный диапазон:</strong> {info.safeRange}
            </div>

            <div className="tooltip-section">
              <strong>Влияние:</strong> {info.impact}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
