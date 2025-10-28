# CDNA Dashboard Guide

## Overview

The CDNA Dashboard provides a visual interface for managing and configuring the Cognitive DNA (CDNA) system - the constitutional framework that governs Token and Connection behavior in NeuroGraph OS.

## Features

### 🎛️ Profile Management

Four pre-configured profiles for different system behaviors:

1. **Explorer** 🔍
   - Свободная структура, высокая пластичность
   - Best for: General-purpose exploration and learning
   - Plasticity: 0.8, Evolution: 0.5

2. **Analyzer** 🔬
   - Строгие правила, низкая эволюция
   - Best for: Strict validation and rule enforcement
   - Plasticity: 0.2, Evolution: 0.1

3. **Creative** 🎨
   - Экспериментальный режим
   - Best for: Creative tasks and experimentation
   - Plasticity: 0.95, Evolution: 0.8

4. **Quarantine** 🛡️
   - Изолированный режим тестирования
   - Best for: Safe testing of configuration changes
   - Plasticity: 0.1, Evolution: 0.0
   - Restricted: Max change ±0.5 per dimension

### 📏 Dimension Scales

Control the scale of each of the 8 semantic dimensions:

| Dimension | Icon | Range | Description |
|-----------|------|-------|-------------|
| **PHYSICAL** | 🏃 | 0-20 | Physical 3D space |
| **SENSORY** | 👁️ | 0-20 | Sensory perception |
| **MOTOR** | ✋ | 0-20 | Motor control |
| **EMOTIONAL** | ❤️ | 0-20 | Emotional state (VAD model) |
| **COGNITIVE** | 🧠 | 0-30 | Cognitive processing |
| **SOCIAL** | 👥 | 0-20 | Social interaction |
| **TEMPORAL** | ⏰ | 0-20 | Temporal localization |
| **ABSTRACT** | 💭 | 0-50 | Semantic and logic |

#### Safety Zones

Each dimension has three safety zones:

- 🟢 **Green (Safe)**: Recommended operating range
- 🟡 **Yellow (Caution)**: Proceed with care
- 🔴 **Red (Danger)**: Risk of system instability

### 🧪 Quarantine Mode

Test configuration changes safely:

1. Click **"🧪 Test Changes"** to activate quarantine mode
2. System monitors metrics for 300 seconds:
   - Memory growth
   - Connection breaks
   - Token churn
3. During quarantine:
   - **✓ Apply Now**: Accept changes immediately
   - **✕ Cancel**: Revert to previous configuration

### 📜 History

Track all configuration changes:

- Profile switches
- Manual adjustments
- Quarantine results
- Impact levels (Low/Medium/High)

### 💾 Export/Import

**Export Configuration:**
```json
{
  "version": "2.1.0",
  "profile": "explorer",
  "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 10.0],
  "timestamp": "2025-01-27T20:00:00Z"
}
```

## API Integration

### Endpoints

The dashboard communicates with these API endpoints:

```bash
# Get current CDNA status
GET /api/v1/cdna/status

# Get all profiles
GET /api/v1/cdna/profiles

# Switch profile
POST /api/v1/cdna/profile/{profile_id}

# Update configuration
POST /api/v1/cdna/update
{
  "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 10.0]
}

# Validate configuration
POST /api/v1/cdna/validate

# Quarantine control
POST /api/v1/cdna/quarantine/start
POST /api/v1/cdna/quarantine/stop?apply=true

# Export configuration
POST /api/v1/cdna/export
```

## Usage Tips

### Best Practices

1. **Start with a profile**: Choose a profile that matches your use case
2. **Make incremental changes**: Adjust one dimension at a time
3. **Use quarantine mode**: Always test significant changes in quarantine
4. **Monitor the zones**: Keep most dimensions in the green zone
5. **Check history**: Review past changes to understand system behavior

### Common Workflows

#### Workflow 1: Exploration → Analysis

```
1. Start with "Explorer" profile
2. Gather data and observe system behavior
3. Switch to "Analyzer" profile for validation
4. Fine-tune specific dimensions based on results
```

#### Workflow 2: Creative Experimentation

```
1. Start with "Creative" profile
2. Push abstract dimension to yellow zone
3. Use quarantine mode to test
4. Monitor memory growth and connection breaks
5. Apply if metrics are acceptable
```

#### Workflow 3: Safe Production

```
1. Export current configuration (backup)
2. Switch to "Quarantine" profile
3. Make restricted changes (max ±0.5)
4. Test for full quarantine period
5. Apply and monitor
```

## Troubleshooting

### Dashboard not loading?

1. Ensure API server is running: `http://localhost:8000`
2. Check CORS settings in API
3. Verify CDNA routes are enabled: `GET /api/v1/cdna/status`

### Changes not persisting?

- CDNA configuration is currently in-memory only
- Restart API server resets to default (Explorer profile)
- Use export/import to save configurations

### Sliders not responding?

- In Quarantine profile, changes are limited to ±0.5
- Check console for validation errors
- Ensure values are within dimension ranges

## Technical Details

### Component Structure

```
ui/web/src/
├── components/
│   └── CDNADashboard.tsx    # Main dashboard component
├── styles/
│   └── index.css            # Cyberpunk-themed styles
└── App.tsx                  # Tab navigation integration
```

### State Management

The dashboard uses React hooks for state:

- `currentProfile`: Active CDNA profile
- `currentValues`: Current dimension scale values
- `isQuarantineActive`: Quarantine mode flag
- `history`: Configuration change history

### Styling

The dashboard follows the project's cyberpunk theme:

- Colors: Cyan (#00f0ff), Magenta (#ff006e), Yellow (#ffbe0b)
- Typography: Courier New monospace
- Effects: Glow, shadows, gradients
- Responsive: Grid layout adapts to screen size

## Future Enhancements

Planned features for v1.0:

- [ ] Real-time API integration (replace mock data)
- [ ] WebSocket for live quarantine metrics
- [ ] Persistence layer (save configurations to database)
- [ ] Visualization of system impact (graphs, charts)
- [ ] Profile creation/editing UI
- [ ] Dimension relationship visualization
- [ ] Rollback to any historical configuration
- [ ] Automated testing and validation
- [ ] Export to multiple formats (JSON, YAML, TOML)

## Related Documentation

- [Guardian & CDNA Rust Overview](../GUARDIAN_CDNA_RUST.md)
- [Main README](../README.md)
- [API Documentation](http://localhost:8000/docs)
- [Integration Guide](INTEGRATION_GUIDE.md)

---

**Version**: 0.17.0
**Last Updated**: 2025-01-27
