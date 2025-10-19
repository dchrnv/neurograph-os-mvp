import { useState, useEffect } from 'react'

interface Token {
  id: number
  id_hex: string
  entity_type: number
  weight: number
  timestamp: number
  age_seconds: number
}

const API_BASE = '/api/v1'

export default function App() {
  const [tokens, setTokens] = useState<Token[]>([])
  const [stats, setStats] = useState({ total: 0 })
  const [loading, setLoading] = useState(true)

  const fetchTokens = async () => {
    try {
      const res = await fetch(`${API_BASE}/tokens`)
      const data = await res.json()
      setTokens(data.tokens)
      setStats({ total: data.total })
    } catch (err) {
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const createExample = async () => {
    await fetch(`${API_BASE}/tokens/examples/create`, { method: 'POST' })
    fetchTokens()
  }

  const clearAll = async () => {
    if (confirm('Clear all tokens?')) {
      await fetch(`${API_BASE}/tokens/admin/clear`, { method: 'DELETE' })
      fetchTokens()
    }
  }

  useEffect(() => {
    fetchTokens()
    const interval = setInterval(fetchTokens, 5000)
    return () => clearInterval(interval)
  }, [])

  if (loading) return <div className="loading">Loading NeuroGraph OS...</div>

  return (
    <div className="app">
      <header className="header">
        <h1>⚡ NEUROGRAPH OS ⚡</h1>
        <p>Token v2.0 | 8 Semantic Coordinate Spaces | MVP Dashboard</p>
      </header>

      <div className="dashboard">
        <div className="card">
          <div className="card-header">
            <span className="card-title">SYSTEM STATUS</span>
          </div>
          <div>
            <div className="meta-item">
              <span className="meta-label">Total Tokens:</span>
              <span className="stat-value">{stats.total}</span>
            </div>
            <div className="meta-item">
              <span className="meta-label">Memory:</span>
              <span className="meta-value">{stats.total * 64} bytes</span>
            </div>
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <span className="card-title">CONTROLS</span>
          </div>
          <div className="controls">
            <button className="button" onClick={createExample}>
              Create Examples
            </button>
            <button className="button" onClick={fetchTokens}>
              Refresh
            </button>
            <button className="button" onClick={clearAll}>
              Clear All
            </button>
          </div>
        </div>
      </div>

      <div className="card">
        <div className="card-header">
          <span className="card-title">TOKENS ({tokens.length})</span>
        </div>
        <div className="token-list">
          {tokens.length === 0 ? (
            <p style={{color: 'var(--text-secondary)', textAlign: 'center', padding: '20px'}}>
              No tokens yet. Click "Create Examples" to start.
            </p>
          ) : (
            tokens.map(token => (
              <div key={token.id} className="token-item">
                <div className="token-id">ID: {token.id_hex}</div>
                <div className="token-meta">
                  <div className="meta-item">
                    <span className="meta-label">Type:</span>
                    <span className="meta-value">{token.entity_type}</span>
                  </div>
                  <div className="meta-item">
                    <span className="meta-label">Weight:</span>
                    <span className="meta-value">{token.weight.toFixed(2)}</span>
                  </div>
                  <div className="meta-item">
                    <span className="meta-label">Age:</span>
                    <span className="meta-value">{token.age_seconds}s</span>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  )
}
