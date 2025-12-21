"""
NeuroGraph OS - Python Bindings v0.40.0

High-performance spatial computing system with:
- 8D semantic space (L1-L8)
- Token-based architecture (64 bytes/token)
- IntuitionEngine v3.0 (Hybrid Reflex System)

# Quick Start

## Single Token (For small numbers)
```python
import neurograph

token = neurograph.Token(42)
print(f"ID: {token.id}")
print(f"Coordinates: {token.coordinates}")
```

## Batch Operations (FAST! - 4x speedup)
```python
# GOOD - Pre-allocated in Rust (175ms for 1M tokens)
tokens = neurograph.Token.create_batch(1_000_000)

# BAD - Python loop (708ms for 1M tokens)
tokens = [neurograph.Token(i) for i in range(1_000_000)]
```

## IntuitionEngine (Hybrid Reflex System)
```python
# Simple API - one line!
intuition = neurograph.IntuitionEngine.with_defaults()

# Get statistics
stats = intuition.stats()
print(f"Reflexes: {stats['total_reflexes']}")
print(f"Fast path hits: {stats['fast_path_hits']}")
print(f"Avg fast path time: {stats['avg_fast_path_time_ns']}ns")
```

# Performance Notes

- Token creation: ~677ns each (1.47M/sec)
- Batch creation: 4x faster with pre-allocation
- Memory: 64 bytes/token (61MB for 1M tokens)
- Fast path reflexes: ~30-50ns lookup
- Parallel scaling: 6.75x speedup on 4 cores

# Memory Usage

| Tokens | Memory (Rust) | Memory (Python) |
|--------|---------------|-----------------|
| 1K     | 0.06 MB       | ~0.1 MB         |
| 10K    | 0.61 MB       | ~1.1 MB         |
| 100K   | 6.1 MB        | ~10.7 MB        |
| 1M     | 61 MB         | ~107 MB         |

Python adds ~48 bytes/object overhead (PyObject header).

# License

AGPL-3.0 - Copyright (C) 2024-2025 Chernov Denys
"""

from ._core import (
    Token,
    IntuitionEngine,
    IntuitionConfig,
    SignalSystem,
    __version__,
    __author__,
    __license__,
)

__all__ = [
    "Token",
    "IntuitionEngine",
    "IntuitionConfig",
    "SignalSystem",
    "__version__",
    "__author__",
    "__license__",
]
