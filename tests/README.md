# Tests

Test suite for NeuroGraph OS.

## Structure

```
tests/
├── unit/              # Unit tests (component-specific)
│   ├── test_gateway_models.py
│   ├── test_gateway_core.py
│   ├── test_sensor_registry.py
│   ├── test_encoders.py
│   ├── test_adapters.py
│   ├── test_subscription_filter.py
│   ├── test_action_controller_core.py
│   └── test_action_executors.py
│
├── integration/       # Integration tests (full pipeline)
│   ├── test_integration_pipeline.py
│   ├── test_pipeline_with_core.py
│   └── test_signal_system_core.py
│
└── performance/       # Performance benchmarks
    ├── benchmark_stress_test.py
    └── test_status_performance.py
```

## Running Tests

### All Tests

```bash
# Run all tests
python -m pytest tests/

# With coverage
python -m pytest tests/ --cov=src --cov-report=html
```

### By Category

```bash
# Unit tests only
python -m pytest tests/unit/

# Integration tests only
python -m pytest tests/integration/

# Performance tests only
python -m pytest tests/performance/
```

### Individual Tests

```bash
# Gateway tests
python tests/unit/test_gateway_core.py

# ActionController tests
python tests/unit/test_action_controller_core.py

# Full pipeline with Core
python tests/integration/test_pipeline_with_core.py
```

## Test Categories

### Unit Tests

Component-specific tests for individual modules:

- **Gateway:** Models, encoders, sensor registry
- **Adapters:** Text, System, Timer adapters
- **Filters:** Subscription filters
- **ActionController:** Core, executors, registry, selector

**Run time:** Fast (<1s per test)

### Integration Tests

End-to-end pipeline tests:

- **Pipeline:** Gateway → ActionController
- **Pipeline + Core:** Gateway → Rust Core → ActionController
- **SignalSystem:** Rust Core API

**Run time:** Medium (1-5s per test)

### Performance Tests

Benchmarks and stress tests:

- **Stress Test:** 10M events processing
- **Status Performance:** REST API endpoints

**Run time:** Slow (10s-5min)

## Dependencies

```bash
pip install pytest pytest-cov pytest-asyncio
```

## Writing Tests

### Unit Test Template

```python
#!/usr/bin/env python3
"""
Unit test for [Component]
"""

def test_component():
    """Test component functionality."""
    # Arrange
    component = Component()

    # Act
    result = component.action()

    # Assert
    assert result.success == True
```

### Integration Test Template

```python
#!/usr/bin/env python3
"""
Integration test for [Pipeline]
"""

import asyncio

async def test_pipeline():
    """Test full pipeline."""
    pipeline = Pipeline()

    result = await pipeline.process("input")

    assert result['success'] == True

if __name__ == "__main__":
    asyncio.run(test_pipeline())
```

## CI/CD

Tests are automatically run on:
- Push to main
- Pull requests
- Release tags

See `.github/workflows/tests.yml` for CI configuration.

---

**Total Tests:** 13 files
- Unit: 8 files
- Integration: 3 files
- Performance: 2 files
