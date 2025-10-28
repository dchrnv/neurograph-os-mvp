import { useState } from 'react'

interface Preset {
  id: string
  name: string
  description: string
  scales: number[]
  timestamp: string
  tags: string[]
}

interface PresetsPanelProps {
  currentValues: number[]
  onLoadPreset: (scales: number[]) => void
}

export default function PresetsPanel({ currentValues, onLoadPreset }: PresetsPanelProps) {
  const [presets, setPresets] = useState<Preset[]>([
    {
      id: 'balanced',
      name: 'Balanced',
      description: 'Сбалансированная конфигурация для общего использования',
      scales: [2.0, 2.0, 2.0, 3.0, 5.0, 3.0, 2.0, 8.0],
      timestamp: '2025-01-20',
      tags: ['general', 'safe']
    },
    {
      id: 'high-performance',
      name: 'High Performance',
      description: 'Оптимизация для скорости обработки',
      scales: [1.0, 1.0, 1.0, 1.5, 8.0, 2.0, 1.5, 12.0],
      timestamp: '2025-01-22',
      tags: ['performance', 'fast']
    },
    {
      id: 'experimental',
      name: 'Experimental',
      description: 'Для экспериментов и тестирования',
      scales: [3.0, 4.0, 5.0, 7.0, 12.0, 15.0, 18.0, 30.0],
      timestamp: '2025-01-25',
      tags: ['experimental', 'high-risk']
    }
  ])

  const [newPresetName, setNewPresetName] = useState('')
  const [newPresetDesc, setNewPresetDesc] = useState('')
  const [showSaveForm, setShowSaveForm] = useState(false)

  const saveCurrentAsPreset = () => {
    if (!newPresetName.trim()) return

    const newPreset: Preset = {
      id: `preset-${Date.now()}`,
      name: newPresetName,
      description: newPresetDesc || 'Custom preset',
      scales: [...currentValues],
      timestamp: new Date().toISOString().split('T')[0],
      tags: ['custom']
    }

    setPresets([...presets, newPreset])
    setNewPresetName('')
    setNewPresetDesc('')
    setShowSaveForm(false)

    // Save to localStorage
    localStorage.setItem('cdna-presets', JSON.stringify([...presets, newPreset]))
  }

  const deletePreset = (id: string) => {
    if (!id.startsWith('preset-')) {
      alert('Невозможно удалить встроенный пресет')
      return
    }

    const filtered = presets.filter(p => p.id !== id)
    setPresets(filtered)
    localStorage.setItem('cdna-presets', JSON.stringify(filtered))
  }

  return (
    <div className="presets-panel">
      <div className="presets-header">
        <h3>📁 Saved Presets</h3>
        <button
          className="button-small"
          onClick={() => setShowSaveForm(!showSaveForm)}
        >
          {showSaveForm ? '✕ Отмена' : '💾 Сохранить текущую'}
        </button>
      </div>

      {showSaveForm && (
        <div className="save-preset-form">
          <input
            type="text"
            placeholder="Название пресета"
            value={newPresetName}
            onChange={(e) => setNewPresetName(e.target.value)}
            className="preset-input"
          />
          <textarea
            placeholder="Описание (опционально)"
            value={newPresetDesc}
            onChange={(e) => setNewPresetDesc(e.target.value)}
            className="preset-textarea"
          />
          <button className="button" onClick={saveCurrentAsPreset}>
            Сохранить
          </button>
        </div>
      )}

      <div className="presets-list">
        {presets.map((preset) => (
          <div key={preset.id} className="preset-item">
            <div className="preset-content">
              <div className="preset-name">{preset.name}</div>
              <div className="preset-description">{preset.description}</div>
              <div className="preset-meta">
                <span className="preset-date">{preset.timestamp}</span>
                <div className="preset-tags">
                  {preset.tags.map(tag => (
                    <span key={tag} className="preset-tag">{tag}</span>
                  ))}
                </div>
              </div>
            </div>
            <div className="preset-actions">
              <button
                className="button-small"
                onClick={() => onLoadPreset(preset.scales)}
              >
                Загрузить
              </button>
              {preset.id.startsWith('preset-') && (
                <button
                  className="button-small danger"
                  onClick={() => deletePreset(preset.id)}
                >
                  Удалить
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}
