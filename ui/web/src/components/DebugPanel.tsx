import { useState } from 'react'

interface DebugPanelProps {
  currentValues: number[]
  currentProfile: string
  history: any[]
  dimensionNames: string[]
}

export default function DebugPanel({ currentValues, currentProfile, history, dimensionNames }: DebugPanelProps) {
  const [isExpanded, setIsExpanded] = useState(false)
  const [copiedSection, setCopiedSection] = useState<string | null>(null)

  const cdnaData = {
    version: '2.1.0',
    profile: currentProfile,
    dimension_scales: currentValues,
    dimensions: dimensionNames.map((name, i) => ({
      name,
      value: currentValues[i]
    })),
    timestamp: new Date().toISOString(),
    stats: {
      avg: (currentValues.reduce((a, b) => a + b, 0) / currentValues.length).toFixed(2),
      min: Math.min(...currentValues).toFixed(2),
      max: Math.max(...currentValues).toFixed(2),
      sum: currentValues.reduce((a, b) => a + b, 0).toFixed(2)
    }
  }

  const copyToClipboard = (text: string, section: string) => {
    navigator.clipboard.writeText(text)
    setCopiedSection(section)
    setTimeout(() => setCopiedSection(null), 2000)
  }

  return (
    <div className="debug-panel">
      <div
        className="debug-header"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span>🐛 Debug Panel</span>
        <span className="toggle-icon">{isExpanded ? '▼' : '▶'}</span>
      </div>

      {isExpanded && (
        <div className="debug-content">
          <div className="debug-section">
            <div className="debug-section-header">
              <h4>Current Configuration</h4>
              <button
                className="btn-copy"
                onClick={() => copyToClipboard(JSON.stringify(cdnaData, null, 2), 'config')}
              >
                {copiedSection === 'config' ? '✓ Copied' : '📋 Copy'}
              </button>
            </div>
            <pre className="debug-code">
              {JSON.stringify(cdnaData, null, 2)}
            </pre>
          </div>

          <div className="debug-section">
            <div className="debug-section-header">
              <h4>History ({history.length} entries)</h4>
              <button
                className="btn-copy"
                onClick={() => copyToClipboard(JSON.stringify(history, null, 2), 'history')}
              >
                {copiedSection === 'history' ? '✓ Copied' : '📋 Copy'}
              </button>
            </div>
            <pre className="debug-code">
              {JSON.stringify(history.slice(0, 5), null, 2)}
            </pre>
          </div>

          <div className="debug-section">
            <div className="debug-section-header">
              <h4>System Info</h4>
            </div>
            <div className="debug-info">
              <div className="debug-info-item">
                <span className="label">CDNA Version:</span>
                <span className="value">2.1.0</span>
              </div>
              <div className="debug-info-item">
                <span className="label">Active Profile:</span>
                <span className="value">{currentProfile}</span>
              </div>
              <div className="debug-info-item">
                <span className="label">Dimensions:</span>
                <span className="value">{dimensionNames.length}</span>
              </div>
              <div className="debug-info-item">
                <span className="label">Browser:</span>
                <span className="value">{navigator.userAgent.split(' ').pop()}</span>
              </div>
              <div className="debug-info-item">
                <span className="label">Timestamp:</span>
                <span className="value">{new Date().toLocaleString()}</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
