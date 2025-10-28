import { RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, Radar, Legend, ResponsiveContainer } from 'recharts'

interface ProfileRadarChartProps {
  currentValues: number[]
  defaultValues?: number[]
  dimensionNames: string[]
}

export default function ProfileRadarChart({ currentValues, defaultValues, dimensionNames }: ProfileRadarChartProps) {
  const data = dimensionNames.map((name, index) => ({
    dimension: name,
    current: currentValues[index],
    default: defaultValues ? defaultValues[index] : 0
  }))

  return (
    <div className="radar-chart-container">
      <ResponsiveContainer width="100%" height={400}>
        <RadarChart data={data}>
          <PolarGrid stroke="rgba(0, 240, 255, 0.3)" />
          <PolarAngleAxis
            dataKey="dimension"
            tick={{ fill: 'var(--text-primary)', fontSize: 11 }}
          />
          <PolarRadiusAxis
            angle={90}
            domain={[0, 50]}
            tick={{ fill: 'var(--text-secondary)', fontSize: 10 }}
          />
          <Radar
            name="Current"
            dataKey="current"
            stroke="var(--accent-cyan)"
            fill="var(--accent-cyan)"
            fillOpacity={0.3}
            strokeWidth={2}
          />
          {defaultValues && (
            <Radar
              name="Default"
              dataKey="default"
              stroke="var(--accent-magenta)"
              fill="var(--accent-magenta)"
              fillOpacity={0.1}
              strokeWidth={1}
              strokeDasharray="5 5"
            />
          )}
          <Legend
            wrapperStyle={{ color: 'var(--text-primary)' }}
            iconType="circle"
          />
        </RadarChart>
      </ResponsiveContainer>
    </div>
  )
}
