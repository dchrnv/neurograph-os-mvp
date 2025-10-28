import { useState } from 'react'
import ProfileRadarChart from './ProfileRadarChart'
import DimensionHistoryChart from './DimensionHistoryChart'
import DimensionTooltip from './DimensionTooltip'
import WarningSystem from './WarningSystem'
import QuarantinePanel from './QuarantinePanel'
import Recommendations from './Recommendations'
import PresetsPanel from './PresetsPanel'
import DebugPanel from './DebugPanel'
import FitnessRings from './FitnessRings'

interface Dimension {
  id: string
  name: string
  icon: string
  default: number
  min: number
  max: number
  green: [number, number]
  yellow: [number, number]
  red: [number, number]
}

interface Profile {
  name: string
  scales: number[]
  description: string
  restricted?: boolean
  maxChange?: number
  requireValidation?: boolean
}

interface HistoryEntry {
  time: string
  desc: string
  impact: string
  scales?: number[]
}

const dimensions: Dimension[] = [
  { id: 'physical', name: 'PHYSICAL', icon: '🏃', default: 1.0, min: 0, max: 20, green: [1, 5], yellow: [5, 15], red: [15, 20] },
  { id: 'sensory', name: 'SENSORY', icon: '👁️', default: 1.5, min: 0, max: 20, green: [1, 5], yellow: [5, 15], red: [15, 20] },
  { id: 'motor', name: 'MOTOR', icon: '✋', default: 1.2, min: 0, max: 20, green: [1, 5], yellow: [5, 15], red: [15, 20] },
  { id: 'emotional', name: 'EMOTIONAL', icon: '❤️', default: 2.0, min: 0, max: 20, green: [1, 8], yellow: [8, 15], red: [15, 20] },
  { id: 'cognitive', name: 'COGNITIVE', icon: '🧠', default: 3.0, min: 0, max: 30, green: [1, 15], yellow: [15, 25], red: [25, 30] },
  { id: 'social', name: 'SOCIAL', icon: '👥', default: 2.5, min: 0, max: 20, green: [1, 10], yellow: [10, 15], red: [15, 20] },
  { id: 'temporal', name: 'TEMPORAL', icon: '⏰', default: 2.0, min: 0, max: 20, green: [1, 8], yellow: [8, 15], red: [15, 20] },
  { id: 'abstract', name: 'ABSTRACT', icon: '💭', default: 10.0, min: 0, max: 50, green: [1, 20], yellow: [20, 40], red: [40, 50] }
]

const profiles: Record<string, Profile> = {
  explorer: {
    name: 'Explorer',
    scales: [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
    description: 'Свободная структура, высокая пластичность'
  },
  analyzer: {
    name: 'Analyzer',
    scales: [1.0, 1.0, 1.0, 1.5, 10.0, 5.0, 3.0, 20.0],
    description: 'Строгие правила, низкая эволюция'
  },
  creative: {
    name: 'Creative',
    scales: [1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0],
    description: 'Экспериментальный режим'
  },
  quarantine: {
    name: 'Quarantine',
    scales: [1.0, 1.0, 1.0, 1.0, 2.0, 1.5, 1.0, 3.0],
    description: 'Изолированный режим тестирования',
    restricted: true,
    maxChange: 0.5,
    requireValidation: true
  }
}

const profileIcons: Record<string, string> = {
  explorer: '🔍',
  analyzer: '🔬',
  creative: '🎨',
  quarantine: '🛡️'
}

export default function CDNADashboard() {
  const [currentProfile, setCurrentProfile] = useState('explorer')
  const [currentValues, setCurrentValues] = useState<number[]>([...profiles.explorer.scales])
  const [isQuarantineActive, setIsQuarantineActive] = useState(false)
  const [quarantineTime, setQuarantineTime] = useState(300)
  const [quarantineProgress, setQuarantineProgress] = useState(0)
  const [selectedDimension, setSelectedDimension] = useState(0)
  const [showVisualization, setShowVisualization] = useState(true)
  const [showRecommendations, setShowRecommendations] = useState(true)
  const [showPresets, setShowPresets] = useState(false)
  const [history, setHistory] = useState<HistoryEntry[]>([
    {
      time: 'Сейчас (активна)',
      desc: 'Профиль: Explorer',
      impact: '',
      scales: [...profiles.explorer.scales]
    }
  ])

  const updateValue = (index: number, value: number) => {
    const profile = profiles[currentProfile]

    if (profile.restricted && profile.maxChange) {
      const originalValue = profile.scales[index]
      const diff = Math.abs(value - originalValue)

      if (diff > profile.maxChange) {
        value = value > originalValue
          ? originalValue + profile.maxChange
          : originalValue - profile.maxChange
      }
    }

    const newValues = [...currentValues]
    newValues[index] = value
    setCurrentValues(newValues)
  }

  const switchProfile = (profileId: string) => {
    if (profileId === currentProfile) return

    const newScales = [...profiles[profileId].scales]

    setHistory(prev => [
      {
        time: 'Сейчас (активна)',
        desc: `Профиль: ${profiles[profileId].name}`,
        impact: 'high',
        scales: newScales
      },
      ...prev.map((item, idx) => ({
        ...item,
        time: idx === 0 ? '1 минуту назад' : item.time
      }))
    ])

    setCurrentProfile(profileId)
    setCurrentValues(newScales)
  }

  const proposeChanges = () => {
    setIsQuarantineActive(true)
    let timeLeft = 300

    const interval = setInterval(() => {
      timeLeft -= 1
      setQuarantineTime(timeLeft)
      setQuarantineProgress(((300 - timeLeft) / 300) * 100)

      if (timeLeft <= 0) {
        clearInterval(interval)
        applyChanges()
      }
    }, 100)
  }

  const applyChanges = () => {
    setIsQuarantineActive(false)
    setQuarantineTime(300)
    setQuarantineProgress(0)

    setHistory(prev => [
      {
        time: 'Сейчас (активна)',
        desc: profiles[currentProfile].name,
        impact: '',
        scales: [...currentValues]
      },
      {
        time: 'Только что',
        desc: 'Изменения применены ✓',
        impact: 'medium',
        scales: [...currentValues]
      },
      ...prev.slice(1)
    ])

    alert('✓ Изменения успешно применены!\n\nСистема работает стабильно.')
  }

  const cancelQuarantine = () => {
    setIsQuarantineActive(false)
    setQuarantineTime(300)
    setQuarantineProgress(0)
    switchProfile(currentProfile)
    alert('✕ Изменения отменены.\n\nСистема откачена к предыдущей версии.')
  }

  const resetToDefaults = () => {
    if (confirm('Сбросить все параметры к значениям по умолчанию?')) {
      switchProfile('explorer')
      alert('↺ Параметры сброшены к профилю "Explorer"')
    }
  }

  const exportCDNA = () => {
    const cdnaData = {
      version: '2.1.0',
      profile: currentProfile,
      dimension_scales: currentValues,
      timestamp: new Date().toISOString()
    }

    const dataStr = JSON.stringify(cdnaData, null, 2)
    const dataBlob = new Blob([dataStr], { type: 'application/json' })
    const url = URL.createObjectURL(dataBlob)

    const link = document.createElement('a')
    link.href = url
    link.download = `cdna_${currentProfile}_${Date.now()}.json`
    link.click()

    URL.revokeObjectURL(url)
  }

  const loadPreset = (scales: number[]) => {
    setCurrentValues([...scales])
    setHistory(prev => [
      {
        time: 'Сейчас',
        desc: 'Загружен пресет',
        impact: 'medium',
        scales: [...scales]
      },
      ...prev
    ])
  }

  return (
    <div className="cdna-container animate-fade-in">
      {/* Header */}
      <div className="cdna-header">
        <div className="cdna-header-content">
          <h2 className="cdna-title">🧬 CDNA v2.1 — Constitutional Layer</h2>
          <p className="cdna-subtitle">Cognitive DNA Configuration Manager</p>

          <div className="cdna-status-bar">
            <div className={`cdna-status-badge ${isQuarantineActive ? 'quarantine' : 'stable'}`}>
              <span>●</span>
              <span>{isQuarantineActive ? 'QUARANTINE' : 'STABLE'}</span>
            </div>
            <div className="cdna-status-badge info">
              <span>📦</span>
              <span>384 bytes</span>
            </div>
            <div className="cdna-status-badge info">
              <span>⚡</span>
              <span>6 cache lines</span>
            </div>
          </div>
        </div>
      </div>

      {/* Warning System */}
      <WarningSystem
        currentValues={currentValues}
        dimensionNames={dimensions.map(d => d.name)}
        dimensionLimits={dimensions.map(d => ({ min: d.min, max: d.max, green: d.green, yellow: d.yellow }))}
      />

      <div className="cdna-main-grid">
        {/* Sidebar */}
        <div className="cdna-sidebar">
          {/* Profiles */}
          <div className="card">
            <div className="card-header">
              <span className="card-title">📋 PROFILES</span>
            </div>
            <div className="cdna-profile-list">
              {Object.entries(profiles).map(([id, profile]) => (
                <div
                  key={id}
                  className={`cdna-profile-item ${currentProfile === id ? 'active' : ''}`}
                  onClick={() => switchProfile(id)}
                >
                  <div className="cdna-profile-icon">{profileIcons[id]}</div>
                  <div className="cdna-profile-info">
                    <div className="cdna-profile-name">{profile.name}</div>
                    <div className="cdna-profile-desc">{profile.description}</div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Visualization Toggle */}
          <div className="card">
            <div className="card-header">
              <span className="card-title">📊 VISUALIZATIONS</span>
            </div>
            <div className="controls">
              <button
                className={`button-small ${showVisualization ? '' : 'secondary'}`}
                onClick={() => setShowVisualization(!showVisualization)}
              >
                {showVisualization ? '👁️ Скрыть' : '👁️ Показать'} Radar
              </button>
              <button
                className={`button-small ${showPresets ? '' : 'secondary'}`}
                onClick={() => setShowPresets(!showPresets)}
              >
                {showPresets ? '📁 Скрыть' : '📁 Показать'} Presets
              </button>
            </div>
          </div>

          {/* History */}
          <div className="card">
            <div className="card-header">
              <span className="card-title">📜 HISTORY</span>
            </div>
            <div className="cdna-history-list">
              {history.slice(0, 5).map((item, idx) => (
                <div key={idx} className={`cdna-history-item ${idx === 0 ? 'current' : ''}`}>
                  <div className="cdna-history-time">{item.time}</div>
                  <div className="cdna-history-desc">
                    {item.desc}
                    {item.impact && (
                      <span className={`cdna-impact-badge impact-${item.impact}`}>
                        {item.impact.toUpperCase()}
                      </span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Main Content */}
        <div className="cdna-content-area">
          {/* Quarantine Panel */}
          <QuarantinePanel
            isActive={isQuarantineActive}
            timeLeft={quarantineTime}
            progress={quarantineProgress}
            onApply={applyChanges}
            onCancel={cancelQuarantine}
          />

          {/* Radar Chart */}
          {showVisualization && (
            <div className="card animate-slide-down">
              <div className="card-header">
                <span className="card-title">📊 PROFILE VISUALIZATION</span>
              </div>
              <ProfileRadarChart
                currentValues={currentValues}
                defaultValues={profiles.explorer.scales}
                dimensionNames={dimensions.map(d => d.name)}
              />
              {selectedDimension >= 0 && history.length > 1 && (
                <DimensionHistoryChart
                  history={history}
                  dimensionNames={dimensions.map(d => d.name)}
                  selectedDimension={selectedDimension}
                />
              )}
            </div>
          )}

          {/* Recommendations */}
          {showRecommendations && (
            <div className="animate-slide-up">
              <Recommendations
                currentValues={currentValues}
                currentProfile={currentProfile}
              />
            </div>
          )}

          {/* Presets */}
          {showPresets && (
            <div className="animate-slide-down">
              <PresetsPanel
                currentValues={currentValues}
                onLoadPreset={loadPreset}
              />
            </div>
          )}

          {/* Dimension Controls */}
          <div className="card">
            <div className="card-header">
              <span className="card-title">🎛️ DIMENSION SCALES</span>
            </div>
            <div className="cdna-dimension-controls">
              {dimensions.map((dim, index) => (
                <DimensionTooltip key={dim.id} dimensionId={dim.id}>
                  <div
                    className="cdna-dimension-card"
                    onClick={() => setSelectedDimension(index)}
                  >
                    <div className="cdna-dimension-row">
                      <div className="cdna-dimension-main">
                        <div className="cdna-dimension-header">
                          <span className="cdna-dimension-name">{dim.icon} {dim.name}</span>
                          <span className="cdna-dimension-value">{currentValues[index].toFixed(1)}</span>
                        </div>

                        <div className="cdna-zones-indicator">
                          <div className="cdna-zone green">🟢 Safe</div>
                          <div className="cdna-zone yellow">🟡 Caution</div>
                          <div className="cdna-zone red">🔴 Danger</div>
                        </div>

                        <input
                          type="range"
                          className="cdna-slider"
                          min={dim.min}
                          max={dim.max}
                          step={0.1}
                          value={currentValues[index]}
                          onChange={(e) => updateValue(index, parseFloat(e.target.value))}
                        />

                        <div className="cdna-zone-labels">
                          <span>{dim.min}</span>
                          <span>{dim.green[1]}</span>
                          <span>{dim.yellow[1]}</span>
                          <span>{dim.max}</span>
                        </div>
                      </div>

                      <div className="cdna-dimension-rings">
                        <FitnessRings
                          x={(currentValues[index] / dim.max) * 100}
                          y={((currentValues[index] * 0.8) / dim.max) * 100}
                          z={((currentValues[index] * 0.6) / dim.max) * 100}
                          size={120}
                        />
                      </div>
                    </div>
                  </div>
                </DimensionTooltip>
              ))}
            </div>
          </div>

          {/* Actions */}
          <div className="card">
            <div className="card-header">
              <span className="card-title">⚙️ ACTIONS</span>
            </div>
            <div className="controls">
              <button className="button" onClick={proposeChanges}>
                <span>🧪</span> Test Changes
              </button>
              <button className="button" onClick={resetToDefaults}>
                <span>↺</span> Reset to Defaults
              </button>
              <button className="button" onClick={exportCDNA}>
                <span>💾</span> Export CDNA
              </button>
              <button className="button" onClick={() => setShowRecommendations(!showRecommendations)}>
                <span>💡</span> {showRecommendations ? 'Hide' : 'Show'} Tips
              </button>
            </div>
          </div>

          {/* Debug Panel */}
          <DebugPanel
            currentValues={currentValues}
            currentProfile={currentProfile}
            history={history}
            dimensionNames={dimensions.map(d => d.name)}
          />
        </div>
      </div>
    </div>
  )
}
