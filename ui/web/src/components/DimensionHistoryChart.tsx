import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts'

interface HistoryEntry {
  time: string
  desc: string
  impact: string
  scales?: number[]
}

interface DimensionHistoryChartProps {
  history: HistoryEntry[]
  dimensionNames: string[]
  selectedDimension: number
}

export default function DimensionHistoryChart({ history, dimensionNames, selectedDimension }: DimensionHistoryChartProps) {
  // Transform history data for the chart
  const data = history
    .filter(h => h.scales)
    .reverse()
    .map((entry, index) => ({
      time: `T-${history.length - index}`,
      value: entry.scales![selectedDimension],
      label: entry.time
    }))

  return (
    <div className="history-chart-container">
      <div className="chart-header">
        <h4>History: {dimensionNames[selectedDimension]}</h4>
      </div>
      <ResponsiveContainer width="100%" height={200}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" stroke="rgba(0, 240, 255, 0.1)" />
          <XAxis
            dataKey="time"
            stroke="var(--text-secondary)"
            tick={{ fill: 'var(--text-secondary)', fontSize: 10 }}
          />
          <YAxis
            stroke="var(--text-secondary)"
            tick={{ fill: 'var(--text-secondary)', fontSize: 10 }}
            domain={[0, 50]}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'var(--bg-card)',
              border: '1px solid var(--border)',
              borderRadius: '8px',
              color: 'var(--text-primary)'
            }}
          />
          <Line
            type="monotone"
            dataKey="value"
            stroke="var(--accent-cyan)"
            strokeWidth={2}
            dot={{ fill: 'var(--accent-cyan)', r: 4 }}
            activeDot={{ r: 6 }}
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  )
}
