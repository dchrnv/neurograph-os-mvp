# FFI Integration Guide - Python Bindings

**Version:** 0.14.0
**Status:** Production Ready
**Last Updated:** 2025-10-25

This guide explains how to use the Rust-powered NeuroGraph Core from Python via PyO3 FFI bindings.

---

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [API Reference](#api-reference)
5. [Performance](#performance)
6. [Architecture](#architecture)
7. [Examples](#examples)
8. [Troubleshooting](#troubleshooting)

---

## Overview

NeuroGraph Core v0.14.0 provides Python bindings for the high-performance Rust implementation of:

- **Token V2.0** (64-byte structure, 8-dimensional semantic space)
- **Connection V1.0** (32-byte structure, 40+ connection types)

### Key Features

✅ **Zero-copy serialization** - Instant conversion to/from bytes
✅ **Native performance** - 10-100x faster than pure Python
✅ **Type-safe** - Full type annotations and PyO3 validation
✅ **Memory-efficient** - Fixed 64/32 byte structures
✅ **Thread-safe** - Rust guarantees memory safety
✅ **Binary-compatible** - Works with Rust, TypeScript, C++

---

## Installation

### Prerequisites

- Python 3.8+
- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- pip (Python package manager)

### Method 1: Using Maturin (Recommended)

```bash
# Install maturin
pip install maturin

# Navigate to core_rust directory
cd src/core_rust

# Build and install in development mode
maturin develop --release --features python

# Or build wheel for distribution
maturin build --release --features python
pip install target/wheels/*.whl
```

### Method 2: Using setup.py

```bash
cd src/core_rust
pip install setuptools-rust
python setup.py develop
```

### Verify Installation

```python
import neurograph_core
from neurograph import Token, Connection

token = Token(42)
print(token)  # Token(id=42, weight=1.00, active=False, entity_type=Undefined)
```

---

## Quick Start

### Creating Tokens

```python
from neurograph import Token, CoordinateSpace, EntityType

# Create token
token = Token(42)

# Set coordinates in physical space
token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)

# Set properties
token.weight = 2.50
token.set_entity_type(EntityType.Concept())
token.set_active(True)

# Get coordinates
x, y, z = token.get_coordinates(CoordinateSpace.L1Physical())
print(f"Position: ({x:.2f}, {y:.2f}, {z:.2f})")
```

### Creating Connections

```python
from neurograph import Connection, ConnectionType

# Create connection
conn = Connection(1, 2, ConnectionType.Synonym())

# Configure
conn.pull_strength = 0.70  # Attraction
conn.preferred_distance = 1.50
conn.rigidity = 0.80
conn.set_bidirectional(True)

# Activate
conn.activate()
print(f"Activations: {conn.activation_count}")
```

### Serialization

```python
# Serialize token
data = token.to_bytes()  # Returns Python bytes (64 bytes)

# Deserialize
restored = Token.from_bytes(data)

# Zero-copy - instant serialization!
```

---

## API Reference

### Token Class

#### Constructor

```python
Token(id: int) -> Token
```

Create a new token with the given ID.

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| `id` | `int` | Token ID (read-only) |
| `weight` | `float` | Token weight (0.0-∞) |
| `field_radius` | `int` | Field radius (0-255) |
| `field_strength` | `int` | Field strength (0-255) |
| `timestamp` | `int` | Timestamp (0-4294967295) |

#### Methods

**Coordinates**

```python
set_coordinates(space: CoordinateSpace, x: float, y: float, z: float) -> None
get_coordinates(space: CoordinateSpace) -> tuple[float, float, float]
all_coordinates() -> list[tuple[str, tuple[float, float, float]]]
distance_to(other: Token, space: CoordinateSpace) -> float
```

**Entity Type**

```python
set_entity_type(entity_type: EntityType) -> None
get_entity_type() -> str
```

**Flags**

```python
set_active(active: bool) -> None
is_active() -> bool
set_persistent(persistent: bool) -> None
is_persistent() -> bool
set_mutable(mutable: bool) -> None
is_mutable() -> bool
set_system(system: bool) -> None
is_system() -> bool
```

**Serialization**

```python
to_bytes() -> bytes  # Returns 64 bytes
from_bytes(data: bytes) -> Token  # Static method
size() -> int  # Static method, returns 64
```

### Connection Class

#### Constructor

```python
Connection(token_a_id: int, token_b_id: int, connection_type: ConnectionType) -> Connection
```

#### Properties

| Property | Type | Description |
|----------|------|-------------|
| `token_a_id` | `int` | Source token ID (read-only) |
| `token_b_id` | `int` | Target token ID (read-only) |
| `pull_strength` | `float` | Attraction/repulsion strength |
| `preferred_distance` | `float` | Equilibrium distance |
| `rigidity` | `float` | Stiffness (0.0-1.0) |
| `activation_count` | `int` | Number of activations (read-only) |
| `created_at` | `int` | Creation timestamp (read-only) |
| `last_activation` | `int` | Last activation timestamp (read-only) |

#### Methods

**Type**

```python
get_connection_type() -> str
```

**Flags**

```python
set_active(active: bool) -> None
is_active() -> bool
set_bidirectional(bidirectional: bool) -> None
is_bidirectional() -> bool
set_persistent(persistent: bool) -> None
is_persistent() -> bool
```

**Activation**

```python
activate() -> None  # Increments count, updates timestamp
```

**Levels**

```python
is_level_active(level: int) -> bool  # level: 0-7
set_level_active(level: int, active: bool) -> None
get_active_levels() -> list[int]
```

**Physics**

```python
calculate_force(current_distance: float) -> float
```

**Serialization**

```python
to_bytes() -> bytes  # Returns 32 bytes
from_bytes(data: bytes) -> Connection  # Static method
size() -> int  # Static method, returns 32
```

### Enums

#### CoordinateSpace

```python
CoordinateSpace.L1Physical()    # Physical 3D space
CoordinateSpace.L2Sensory()     # Sensory perception
CoordinateSpace.L3Motor()       # Motor control
CoordinateSpace.L4Emotional()   # Emotional (VAD)
CoordinateSpace.L5Cognitive()   # Cognitive processing
CoordinateSpace.L6Social()      # Social interaction
CoordinateSpace.L7Temporal()    # Temporal location
CoordinateSpace.L8Abstract()    # Abstract semantics
```

#### EntityType

```python
EntityType.Undefined()
EntityType.Concept()
EntityType.Object()
EntityType.Event()
EntityType.Agent()
EntityType.Process()
EntityType.State()
EntityType.Relation()
EntityType.Attribute()
EntityType.Action()
EntityType.Perception()
EntityType.Memory()
EntityType.Goal()
EntityType.Rule()
EntityType.Pattern()
EntityType.Cluster()
```

#### ConnectionType

See [CONNECTION_V1_RUST.md](CONNECTION_V1_RUST.md) for the complete list of 40+ connection types.

**Examples:**

```python
ConnectionType.Synonym()      # Semantic
ConnectionType.Cause()        # Causal
ConnectionType.Before()       # Temporal
ConnectionType.Near()         # Spatial
ConnectionType.Implies()      # Logical
ConnectionType.Desires()      # Emotional
```

### Helper Functions

```python
from neurograph import (
    create_example_token,
    create_emotional_token,
    create_semantic_connection
)

# Create token with defaults
token = create_example_token(42, x=10.50, y=20.30, z=5.20)

# Create emotional token (VAD)
emotional = create_emotional_token(
    token_id=1,
    valence=0.80,
    arousal=0.60,
    dominance=0.50
)

# Create semantic connection
conn = create_semantic_connection(
    token_a_id=1,
    token_b_id=2,
    connection_type=ConnectionType.Synonym(),
    strength=0.70,
    bidirectional=True
)
```

---

## Performance

### Benchmarks (typical results)

| Operation | Time (μs) | Speedup vs Python |
|-----------|-----------|-------------------|
| Create Token | 0.10-0.20 | 10-20x |
| Set coordinates | 0.05-0.10 | 15-30x |
| Get coordinates | 0.03-0.05 | 20-40x |
| Serialize Token | 0.02-0.05 | 50-100x |
| Calculate distance | 0.10-0.15 | 20-40x |
| Create Connection | 0.08-0.15 | 10-20x |
| Activate connection | 0.05-0.10 | 15-25x |
| Calculate force | 0.03-0.06 | 20-30x |

**Run benchmarks:**

```bash
cd src/core_rust
python examples/benchmark.py
```

### Memory Usage

- **Token:** Exactly 64 bytes (no overhead)
- **Connection:** Exactly 32 bytes (no overhead)
- **Python wrapper:** ~100 bytes per object (PyObject overhead)

**Total memory:**
- Python Token object: ~164 bytes
- Python Connection object: ~132 bytes

Still 5-10x more efficient than pure Python dict/class representations.

---

## Architecture

### FFI Layer Structure

```
┌─────────────────────────────────────┐
│      Python Application             │
│  (user code, examples, tests)       │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  neurograph.py (Python wrapper)     │
│  - Helper functions                 │
│  - Convenience methods              │
│  - Type annotations                 │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  neurograph_core (PyO3 extension)   │
│  - PyToken, PyConnection classes    │
│  - Enum wrappers                    │
│  - Automatic type conversion        │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  Rust Core (token.rs, connection.rs)│
│  - Pure Rust implementation         │
│  - Zero dependencies                │
│  - Binary layout guarantees         │
└─────────────────────────────────────┘
```

### Build Process

1. **Rust compilation** - `cargo build --features python`
   - Compiles Rust code with PyO3 bindings
   - Produces `.so` (Linux), `.dylib` (macOS), or `.pyd` (Windows)

2. **Python module** - `maturin develop`
   - Builds Rust extension
   - Installs Python wrapper
   - Sets up import paths

3. **Usage** - `import neurograph`
   - Python loads compiled extension
   - Zero-cost abstraction over Rust

---

## Examples

### Example 1: Emotional Tokens

```python
from neurograph import create_emotional_token, CoordinateSpace

# Create emotional states
happy = create_emotional_token(1, valence=0.80, arousal=0.60, dominance=0.70)
sad = create_emotional_token(2, valence=-0.70, arousal=0.30, dominance=0.40)

# Calculate emotional distance
distance = happy.distance_to(sad, CoordinateSpace.L4Emotional())
print(f"Emotional distance: {distance:.2f}")  # ~1.90
```

### Example 2: Multi-dimensional Token

```python
from neurograph import Token, CoordinateSpace

token = Token(100)

# Position in multiple spaces
token.set_coordinates(CoordinateSpace.L1Physical(), 10.50, 20.30, 5.20)
token.set_coordinates(CoordinateSpace.L4Emotional(), 0.80, 0.60, 0.50)
token.set_coordinates(CoordinateSpace.L5Cognitive(), 0.70, 0.90, 0.85)

# Get all coordinates
for space, (x, y, z) in token.all_coordinates():
    print(f"{space}: ({x:.2f}, {y:.2f}, {z:.2f})")
```

### Example 3: Connection Force Model

```python
from neurograph import create_semantic_connection, ConnectionType

conn = create_semantic_connection(
    1, 2,
    ConnectionType.Related(),
    strength=0.70
)
conn.preferred_distance = 2.00
conn.rigidity = 0.80

# Calculate forces at different distances
for distance in [0.50, 1.00, 2.00, 3.00, 4.00]:
    force = conn.calculate_force(distance)
    effect = "Pull" if force > 0 else "Push" if force < 0 else "Balance"
    print(f"Distance {distance:.2f}m -> Force {force:.3f} ({effect})")
```

**Output:**
```
Distance 0.50m -> Force  0.840 (Pull)
Distance 1.00m -> Force  0.560 (Pull)
Distance 2.00m -> Force  0.000 (Balance)
Distance 3.00m -> Force -0.560 (Push)
Distance 4.00m -> Force -1.120 (Push)
```

### Example 4: Batch Processing

```python
from neurograph import Token, CoordinateSpace
import time

start = time.time()

# Create 10,000 tokens
tokens = []
for i in range(10000):
    token = Token(i)
    token.set_coordinates(
        CoordinateSpace.L1Physical(),
        float(i % 100),
        float(i % 100),
        0.00
    )
    tokens.append(token)

elapsed = time.time() - start
print(f"Created 10,000 tokens in {elapsed:.3f}s")
# Typical: ~0.010-0.020s (500,000+ ops/sec)
```

---

## Troubleshooting

### Import Error: `ModuleNotFoundError: No module named 'neurograph_core'`

**Solution:** Build and install the extension first:

```bash
cd src/core_rust
maturin develop --release --features python
```

### Build Error: `feature 'python' not found`

**Check:** Cargo.toml has the python feature defined:

```toml
[features]
python = ["pyo3"]
```

### Runtime Error: `Token requires exactly 64 bytes`

**Cause:** Trying to deserialize from incorrect byte array.

**Solution:** Ensure you're passing exactly 64 bytes:

```python
# Wrong
data = b"hello"
token = Token.from_bytes(data)  # Error!

# Correct
data = original_token.to_bytes()  # Always 64 bytes
token = Token.from_bytes(data)  # OK
```

### Performance Issue: Slower than expected

**Check:**
1. Built with `--release` flag (not debug)
2. Using latest Rust toolchain
3. Not running in debugger/profiler

```bash
# Rebuild with optimizations
maturin develop --release --features python
```

### Type Hints Not Working

**Solution:** Install type stubs (if available) or use runtime type checking:

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from neurograph import Token

def process_token(token: Token) -> None:
    ...
```

---

## Next Steps

- **Examples:** See [examples/python_usage.py](../src/core_rust/examples/python_usage.py)
- **Benchmarks:** Run [examples/benchmark.py](../src/core_rust/examples/benchmark.py)
- **Token Spec:** Read [TOKEN_V2_RUST.md](TOKEN_V2_RUST.md)
- **Connection Spec:** Read [CONNECTION_V1_RUST.md](CONNECTION_V1_RUST.md)
- **Release Notes:** See [V0.14.0_RELEASE_NOTES.md](V0.14.0_RELEASE_NOTES.md)

---

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/dchrnv/neurograph-os-mvp/issues
- Documentation: /docs/
- Examples: /src/core_rust/examples/

---

**NeuroGraph OS v0.14.0** - FFI Integration
*High-performance Rust core with Python convenience*
