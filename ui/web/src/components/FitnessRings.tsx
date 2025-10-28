interface FitnessRingsProps {
  x: number  // 0-100%
  y: number  // 0-100%
  z: number  // 0-100%
  size?: number
}

export default function FitnessRings({ x, y, z, size = 60 }: FitnessRingsProps) {
  const ringWidth = 6
  const gap = 4

  // Calculate radius for each ring
  const outerRadius = size / 2
  const middleRadius = outerRadius - ringWidth - gap
  const innerRadius = middleRadius - ringWidth - gap

  // Calculate circumference
  const outerCircumference = 2 * Math.PI * (outerRadius - ringWidth / 2)
  const middleCircumference = 2 * Math.PI * (middleRadius - ringWidth / 2)
  const innerCircumference = 2 * Math.PI * (innerRadius - ringWidth / 2)

  // Calculate dash offset based on percentage
  const outerOffset = outerCircumference * (1 - x / 100)
  const middleOffset = middleCircumference * (1 - y / 100)
  const innerOffset = innerCircumference * (1 - z / 100)

  return (
    <div className="fitness-rings" style={{ width: size, height: size }}>
      <svg width={size} height={size} viewBox={`0 0 ${size} ${size}`}>
        {/* Background rings */}
        <circle
          cx={size / 2}
          cy={size / 2}
          r={outerRadius - ringWidth / 2}
          fill="none"
          stroke="rgba(0, 240, 255, 0.1)"
          strokeWidth={ringWidth}
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={middleRadius - ringWidth / 2}
          fill="none"
          stroke="rgba(255, 0, 110, 0.1)"
          strokeWidth={ringWidth}
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={innerRadius - ringWidth / 2}
          fill="none"
          stroke="rgba(255, 190, 11, 0.1)"
          strokeWidth={ringWidth}
        />

        {/* Progress rings */}
        {/* X-axis (Outer - Cyan) */}
        <circle
          cx={size / 2}
          cy={size / 2}
          r={outerRadius - ringWidth / 2}
          fill="none"
          stroke="var(--accent-cyan)"
          strokeWidth={ringWidth}
          strokeLinecap="round"
          strokeDasharray={outerCircumference}
          strokeDashoffset={outerOffset}
          transform={`rotate(-90 ${size / 2} ${size / 2})`}
          style={{
            transition: 'stroke-dashoffset 0.6s cubic-bezier(0.4, 0, 0.2, 1)',
            filter: 'drop-shadow(0 0 4px var(--accent-cyan))'
          }}
        />

        {/* Y-axis (Middle - Magenta) */}
        <circle
          cx={size / 2}
          cy={size / 2}
          r={middleRadius - ringWidth / 2}
          fill="none"
          stroke="var(--accent-magenta)"
          strokeWidth={ringWidth}
          strokeLinecap="round"
          strokeDasharray={middleCircumference}
          strokeDashoffset={middleOffset}
          transform={`rotate(-90 ${size / 2} ${size / 2})`}
          style={{
            transition: 'stroke-dashoffset 0.6s cubic-bezier(0.4, 0, 0.2, 1)',
            filter: 'drop-shadow(0 0 4px var(--accent-magenta))'
          }}
        />

        {/* Z-axis (Inner - Yellow) */}
        <circle
          cx={size / 2}
          cy={size / 2}
          r={innerRadius - ringWidth / 2}
          fill="none"
          stroke="var(--accent-yellow)"
          strokeWidth={ringWidth}
          strokeLinecap="round"
          strokeDasharray={innerCircumference}
          strokeDashoffset={innerOffset}
          transform={`rotate(-90 ${size / 2} ${size / 2})`}
          style={{
            transition: 'stroke-dashoffset 0.6s cubic-bezier(0.4, 0, 0.2, 1)',
            filter: 'drop-shadow(0 0 4px var(--accent-yellow))'
          }}
        />

        {/* Center icon/text */}
        <text
          x={size / 2}
          y={size / 2}
          textAnchor="middle"
          dominantBaseline="central"
          fill="var(--text-primary)"
          fontSize="10"
          fontWeight="bold"
        >
          XYZ
        </text>
      </svg>

      {/* Legend */}
      <div className="rings-legend">
        <div className="rings-legend-item">
          <span className="rings-dot" style={{ background: 'var(--accent-cyan)' }}></span>
          <span>X: {x.toFixed(0)}%</span>
        </div>
        <div className="rings-legend-item">
          <span className="rings-dot" style={{ background: 'var(--accent-magenta)' }}></span>
          <span>Y: {y.toFixed(0)}%</span>
        </div>
        <div className="rings-legend-item">
          <span className="rings-dot" style={{ background: 'var(--accent-yellow)' }}></span>
          <span>Z: {z.toFixed(0)}%</span>
        </div>
      </div>
    </div>
  )
}
