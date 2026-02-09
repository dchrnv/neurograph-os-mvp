# NeuroGraph

> High-performance cognitive platform with Rust Core, WebSocket API, and Jupyter integration

[![Version](https://img.shields.io/badge/version-1.0.0-brightgreen.svg)](https://github.com/dchrnv/neurograph)
[![Python](https://img.shields.io/badge/python-3.10+-green.svg)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/rust-core-orange.svg)](https://www.rust-lang.org/)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Weights License: CC BY-NC-SA 4.0](https://img.shields.io/badge/Weights_License-CC_BY--NC--SA_4.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)
[![License: Commercial](https://img.shields.io/badge/License-Commercial_Available-purple.svg)](#licensing)


## What is NeuroGraph?

NeuroGraph is a cognitive computing platform that combines:
- **Rust Core** - High-performance event processing (304K events/sec, 0.39μs latency)
- **WebSocket API** - Real-time bidirectional communication (~5ms latency)
- **Jupyter Integration** - Interactive notebooks with magic commands
- **Web Dashboard** - React SPA with real-time monitoring

## Quick Start

### Installation

**From PyPI:**
```bash
pip install ngcore              # Core package
pip install ngcore[jupyter]     # With Jupyter integration
pip install ngcore[api]         # With WebSocket API
pip install ngcore[all]         # Full installation
```

**From Source:**
```bash
# Clone repository
git clone https://github.com/dchrnv/neurograph.git
cd neurograph

# Install dependencies
pip install -e ".[all]"  # Full installation
```

### Usage

**Jupyter Notebook:**
```python
%load_ext neurograph_jupyter
%neurograph init --path ./my_graph.db
%neurograph query "find all nodes"
```

**Python API:**
```python
from neurograph import NeuroGraph

# Your code here
```

**WebSocket Client:**
```python
from neurograph_client import WebSocketClient

client = WebSocketClient("ws://localhost:8000/ws")
await client.subscribe("metrics")
```

## Features

- ✅ **Rust Core** - 304K events/sec processing
- ✅ **WebSocket API** - Real-time events with ~5ms latency
- ✅ **Jupyter Integration** - Magic commands and widgets
- ✅ **Web Dashboard** - React SPA with monitoring
- ✅ **Module Registry** - Dynamic module management
- ✅ **RBAC** - Role-based access control
- ✅ **CI/CD** - GitHub Actions with pytest and cargo test

## Documentation

- **Documentation:** [docs/](docs/)
- **Roadmap:** [ROADMAP.md](ROADMAP.md)
- **Deferred Tasks:** [DEFERRED.md](DEFERRED.md)
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)
- **Current Status:** [STATUS.md](STATUS.md)

## Development

```bash
# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest tests/

# Run Rust tests
cd src/core_rust && cargo test

# Build package
maturin build --release
```

## Project Status

**Current:** v1.0.0 - Production Ready! 🎉

**Recent Updates (2026-01-14):**
- ✅ Fixed all Rust compilation issues (0 warnings)
- ✅ Feedback system P1 (rewards) - working
- ✅ Critical discovery: Graph is dead code (1400+ LOC unused)
- ✅ Documented P2 requirements for v1.1.0
- ✅ Fixed documentation broken links
- ✅ Created comprehensive analysis docs ([GRAPH_ANALYSIS.md](GRAPH_ANALYSIS.md))

**Completed Phases:**
- ✅ Phase 1: Code Quality (100%)
- ✅ Phase 2: Documentation (100%)
- ✅ Phase 3: Production Readiness (100%)
- ✅ Phase 4: Final Polish (100%)
  - ✅ Phase 4.1: Code Cleanup
  - ✅ Phase 4.2: Documentation Review
  - ✅ Phase 4.3: Release Preparation

**Key Features:**
- 304K events/sec processing (Rust Core)
- ~5ms WebSocket latency
- 378 comprehensive API tests
- 0 critical security vulnerabilities
- 100M tokens stress tested
- Production monitoring ready (Grafana + Prometheus)
- Docker images <300MB

**Known Limitations (v1.0.0):**
- ✅ Feedback P1 (reward updates) - working (updates last 1000 events)
- Feedback P2 (user connections) - stub, full implementation v1.1.0
- ADNA proposal application - deferred to v1.1.0
- Graph code (1400+ LOC) - unused, will be removed in v1.1.0
- See [DEFERRED.md](DEFERRED.md) for full list and [GRAPH_ANALYSIS.md](GRAPH_ANALYSIS.md) for details

**Roadmap:**
- ✅ v1.0.0: Production Ready (Released!)
- ⏳ v1.1.0: Feedback P2, Graph removal, ADNA proposals (2-3 weeks)
- ⏳ v1.2.0: ConnectionV3 typed links, NN search improvements (2-3 weeks)

See [ROADMAP.md](ROADMAP.md) and [CHANGELOG.md](CHANGELOG.md) for details.

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

<a href="https://buymeacoffee.com/dreeftwood" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;">
</a>

## License

AGPL-3.0 - See [LICENSE](LICENSE) file for details.

## Links

- **Repository:** https://github.com/dchrnv/neurograph
- **Documentation:** [docs/](docs/)
- **Issues:** https://github.com/dchrnv/neurograph/issues

Research and creation of NeuroGraph.


