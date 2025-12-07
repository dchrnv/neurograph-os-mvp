# NeuroGraph OS - Python Bindings v0.44.1

High-performance Python bindings for NeuroGraph OS core library built with [PyO3](https://pyo3.rs/).

## ✅ Production-Ready (with documented bottlenecks)

**Status:** Production-Ready with known performance characteristics

**Ready for:**
- ✅ Local development and production deployment
- ✅ Docker/Kubernetes deployment
- ✅ Proof-of-concept and benchmarks
- ✅ Performance evaluation
- ✅ Crash-safe persistence (WAL)
- ✅ OOM prevention (Guardian quotas)
- ✅ Production monitoring (Prometheus + Grafana)
- ✅ Post-mortem debugging (Black Box dumps)
- ⚠️ Distributed tracing (17x overhead - use sampling in v0.44.3)

**v0.44.1 improvements:**
- ✅ **Comprehensive Stress Testing** - 9.5M tokens full-stack observability test
- ✅ **Performance Analysis** - documented WAL (971x) and tracing (17x) bottlenecks
- ✅ **Production Recommendations** - known issues and workarounds until v0.44.3
- ✅ **v0.44.x Roadmap** - async WAL (v0.44.2) and tracing sampling (v0.44.3)

**v0.44.0 Final improvements:**
- ✅ **Distributed Tracing** - OpenTelemetry integration with Jaeger backend
- ✅ **Context Propagation** - W3C TraceContext for request correlation
- ✅ **Trace Visualization** - Jaeger UI for end-to-end debugging

**v0.43.0 Final improvements:**
- ✅ **Docker Deployment** - Multi-stage Dockerfile (<50MB Alpine-based)
- ✅ **Docker Compose** - Full stack with optional monitoring (Prometheus + Grafana)
- ✅ **Production-ready** - Health checks, resource limits, non-root user

**v0.42.0 Final improvements:**
- ✅ **Prometheus Metrics** - /metrics endpoint with 15+ metrics
- ✅ **Black Box Recorder** - Flight recorder for crash analysis (last 1000 events)
- ✅ **Logging Utilities** - Structured logging with context

**v0.41.0 Final improvements:**
- ✅ **WAL (Write-Ahead Log)** - CRC32 checksums, binary format, replay mechanism
- ✅ **Resource Quotas** - 10M tokens / 1GB memory limits with aggressive cleanup
- ✅ **Panic recovery** with `catch_unwind` (Rust core)
- ✅ **GIL release** in batch operations and long-running methods
- ✅ **Production panic hook** with structured logging

**For distributed systems (coming soon):**
- ⏳ Cluster coordination (v0.45.0)
- ⏳ Service mesh integration (v0.46.0)

**Production readiness:** See [Production Roadmap](#production-roadmap) below.

## Installation

```bash
# Install from source (requires Rust toolchain)
pip install maturin
maturin develop --release --features python

# Or build wheel
maturin build --release --features python
pip install target/wheels/neurograph-*.whl
```

## Quick Start

### Token Operations

```python
import neurograph

# Single token
token = neurograph.Token(42)
print(f"ID: {token.id}")
print(f"Coordinates: {token.coordinates}")  # 8 layers × 3 coords

# Batch creation (FAST! - 4x speedup)
tokens = neurograph.Token.create_batch(1_000_000)  # 175ms
# vs
tokens = [neurograph.Token(i) for i in range(1_000_000)]  # 708ms
```

### IntuitionEngine (Hybrid Reflex System)

```python
# Simplest API - one line!
intuition = neurograph.IntuitionEngine.with_defaults()

# Get statistics
stats = intuition.stats()
print(f"Reflexes: {stats['total_reflexes']}")
print(f"Fast path hits: {stats['fast_path_hits']}")
print(f"Avg fast path time: {stats['avg_fast_path_time_ns']}ns")

# Custom configuration
config = neurograph.IntuitionConfig(
    analysis_interval_secs=30,
    min_confidence=0.8,
    enable_fast_path=True
)
intuition = neurograph.IntuitionEngine.create(
    config=config,
    capacity=50_000
)
```

## Performance

### Token Operations (from stress tests)

| Operation | Time | Rate | Notes |
|-----------|------|------|-------|
| Create (single) | 677 ns | 1.47M/sec | Per token |
| Create (batch) | 175 ms | 5.71M/sec | 1M tokens |
| Clone | 54 ns | 18.3M/sec | Per token |
| Read (sequential) | 18.6 ns | 53.8M/sec | Cache-friendly |
| Read (random) | 53.4 ns | 18.7M/sec | Still fast |

### Memory Usage

| Tokens | Rust Memory | Python Memory | Notes |
|--------|-------------|---------------|-------|
| 1K | 0.06 MB | ~0.1 MB | - |
| 10K | 0.61 MB | ~1.1 MB | - |
| 100K | 6.1 MB | ~10.7 MB | - |
| 1M | 61 MB | ~107 MB | +48 bytes/object (PyObject) |

### Key Insights

1. **Always use batch operations** for creating large numbers of objects (4x speedup!)
2. **Rust-only path** for hot loops (no Python/Rust boundary crossing)
3. **Linear scaling** confirmed up to 1M+ tokens
4. **Parallel performance**: 6.75x speedup on 4 cores

## Architecture

```
Python Layer (neurograph)
    ↓
PyO3 Bindings (src/python/)
    ↓
Rust Core (neurograph_core)
    - Token (64 bytes)
    - IntuitionEngine v3.0
    - ExperienceStream v2.1
    - Connection v3.0
    - ... (full NeuroGraph OS stack)
```

## API Reference

### Token

```python
class Token:
    def __init__(self, id: int) -> None: ...

    @staticmethod
    def create_batch(count: int) -> list[Token]:
        """Create many tokens at once (4x faster than loop)"""

    @property
    def id(self) -> int:
        """Token unique identifier (u32)"""

    @property
    def coordinates(self) -> list[list[float]]:
        """8D coordinates (8 layers × 3 coords)"""
```

### IntuitionEngine

```python
class IntuitionEngine:
    @staticmethod
    def with_defaults() -> IntuitionEngine:
        """Create with default configuration (simplest API)"""

    @staticmethod
    def create(
        config: IntuitionConfig | None = None,
        capacity: int | None = None,
        channel_size: int | None = None
    ) -> IntuitionEngine:
        """Create with custom configuration"""

    def stats(self) -> dict[str, int]:
        """Get engine statistics

        Returns:
            dict with keys:
                - reflexes_created: Total reflexes created
                - total_reflexes: Current number of reflexes
                - fast_path_hits: Fast path lookup hits
                - avg_fast_path_time_ns: Avg fast path time (ns)
        """
```

### IntuitionConfig

```python
class IntuitionConfig:
    def __init__(
        self,
        analysis_interval_secs: int | None = None,  # Default: 60
        batch_size: int | None = None,              # Default: 1000
        min_confidence: float | None = None,        # Default: 0.7
        max_proposals_per_cycle: int | None = None, # Default: 5
        enable_fast_path: bool | None = None        # Default: True
    ) -> None: ...

    @property
    def analysis_interval_secs(self) -> int: ...

    @property
    def batch_size(self) -> int: ...

    @property
    def min_confidence(self) -> float: ...

    @property
    def enable_fast_path(self) -> bool: ...
```

## Examples

See `examples/python/` directory:

- `token_batch_performance.py` - Demonstrates 4x batch speedup
- `intuition_simple.py` - IntuitionEngine basic usage

## Development

### Building

```bash
# Development build (fast iteration)
maturin develop --features python

# Release build (optimized)
maturin develop --release --features python
```

### Testing

```bash
# Run Python examples
python examples/python/token_batch_performance.py
python examples/python/intuition_simple.py
```

### Benchmarking

Rust benchmarks (run before building Python bindings):

```bash
cd src/core_rust

# 1M tokens stress test
cargo test --test stress_test_1m_tokens -- --ignored --nocapture

# Criterion benchmarks
cargo bench --bench token_1m_bench
cargo bench --bench intuition_bench
```

## Design Principles

### 1. Batch Operations First

**DON'T:**
```python
tokens = [neurograph.Token(i) for i in range(1_000_000)]  # 708ms
```

**DO:**
```python
tokens = neurograph.Token.create_batch(1_000_000)  # 175ms (4x faster!)
```

### 2. Rust Does Heavy Lifting

All expensive operations happen in Rust:
- Token creation
- Reflex lookup (~30-50ns)
- Pattern analysis
- Memory management

Python only receives final results.

### 3. Zero-Copy Where Possible

- Tokens are cloned to Python (64 bytes each - acceptable)
- Large datasets stay in Rust (use iterators/batches)
- Avoid Python↔Rust roundtrips in hot loops

## Production Roadmap

### v0.41.0 - Reliability (Critical) [✅ COMPLETED]
- [x] WAL (Write-Ahead Log) for data persistence ✅
- [x] Panic recovery with `catch_unwind` ✅
- [x] GIL release for long operations (`py.allow_threads()`) ✅
- [x] Resource quotas in Guardian ✅

### v0.42.0 - Observability [✅ COMPLETED]
- [x] Prometheus metrics export ✅
- [x] Black Box Recorder (last 1000 events on crash) ✅
- [x] Structured logging improvements ✅

### v0.43.0 - Docker Deployment [✅ COMPLETED]
- [x] Dockerfile (multi-stage, <50MB) ✅
- [x] Docker Compose for full stack ✅
- [x] Health check endpoints ✅
- [x] Optional monitoring stack (Prometheus + Grafana) ✅

### v0.44.0 - Distributed Tracing [✅ COMPLETED]
- [x] OpenTelemetry integration ✅
- [x] Jaeger backend support ✅
- [x] W3C TraceContext propagation ✅
- [x] Automatic span creation for HTTP requests ✅

### v0.45.0 - Cluster Coordination
- [ ] etcd integration
- [ ] Raft consensus
- [ ] Distributed state management

### v0.46.0+ - Performance & Scaling
- [ ] Zero-copy NumPy views
- [ ] Async Python bindings for Tokio
- [ ] More batch operations (connections, similarity)

---

## License

AGPL-3.0 - Copyright (C) 2024-2025 Chernov Denys

## Credits

- **Core Implementation**: Rust (neurograph_core)
- **Python Bindings**: PyO3 v0.22
- **Build System**: Maturin v1.x
