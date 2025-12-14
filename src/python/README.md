# neurograph — Python Library

**Version:** 0.1.0
**Status:** Alpha
**License:** AGPL-3.0-or-later (dual licensing available)

High-performance Python library for semantic knowledge graphs, built on top of neurograph-core (Rust FFI).

## Quick Start

```python
import neurograph as ng

# Initialize runtime
runtime = ng.Runtime()

# Load embeddings
runtime.bootstrap("glove.6B.50d.txt", limit=50000)

# Query
result = runtime.query("cat")
print(result.top(5))
# [('dog', 0.92), ('kitten', 0.87), ('pet', 0.81), ...]

# Feedback
runtime.feedback(result.signal_id, "positive")
```

## Installation

### From PyPI (when published)

```bash
pip install neurograph
```

### From source

```bash
# Clone repository
git clone https://github.com/dchrnv/neurograph-os
cd neurograph-os/src/python

# Install in development mode
pip install -e ".[dev]"
```

## Features

- ⚡ **High Performance** — Rust core with PyO3 bindings
- 🔍 **Semantic Search** — Fast similarity queries in embedding space
- 📊 **Bootstrap Support** — Load GloVe, Word2Vec embeddings
- 🎯 **Feedback System** — Improve results with user feedback
- 📈 **Metrics** — Built-in Prometheus metrics
- 🔧 **Configuration** — Flexible runtime configuration

## Architecture

```
┌────────────────────────────────────┐
│   neurograph (Python API)          │
│   - Runtime Manager                │
│   - Query Engine                   │
│   - Bootstrap Loader               │
└──────────────┬─────────────────────┘
               │ PyO3 FFI
┌──────────────▼─────────────────────┐
│   neurograph-core (Rust)           │
│   - Token System                   │
│   - Graph Engine                   │
│   - IntuitionEngine                │
│   - Guardian (CDNA)                │
└────────────────────────────────────┘
```

## Documentation

- [Quick Start Guide](docs/quickstart.md)
- [API Reference](docs/api.md)
- [Configuration](docs/configuration.md)
- [Examples](examples/)

## Development

### Setup

```bash
# Install dev dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Format code
black neurograph/
ruff neurograph/

# Type checking
mypy neurograph/
```

### Building

```bash
# Build wheel
maturin build --release

# Build and install locally
maturin develop
```

## Examples

See [examples/](examples/) directory for:
- Basic usage
- Custom configurations
- Integration with Jupyter
- Advanced queries

## License

This project is dual-licensed:

- **Open Source**: AGPL-3.0-or-later
- **Commercial**: Contact dreeftwood@gmail.com for commercial licensing

## Contributing

We welcome contributions! Please:
1. Sign the CLA (Contributor License Agreement)
2. Follow code style (black + ruff)
3. Add tests for new features
4. Update documentation

## Support

- **Issues**: https://github.com/dchrnv/neurograph-os/issues
- **Discussions**: https://github.com/dchrnv/neurograph-os/discussions
- **Email**: dreeftwood@gmail.com

---

Built with ❤️ by NeuroGraph Team
