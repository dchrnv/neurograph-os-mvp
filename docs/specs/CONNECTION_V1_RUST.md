## Connection V1.0 - Rust Implementation

**Version:** 0.13.0 mvp_ConnectionR
**Status:** ✅ Production Ready
**Language:** Rust 2021 Edition
**Dependencies:** Zero (pure Rust)

## Overview

This is the **official Rust implementation** of NeuroGraph OS Connection V1.0 - the 32-byte data structure representing directed or undirected relationships between tokens in 8-dimensional semantic space.

## What's New in v0.13.0

- ✅ Complete Rust implementation of Connection V1.0 specification
- ✅ 32-byte packed structure (naturally aligned)
- ✅ 40+ connection types across 11 categories
- ✅ Bitfield support for 8 active levels
- ✅ Physical force model (pull_strength, preferred_distance)
- ✅ Activation tracking and lifecycle management
- ✅ 10+ comprehensive unit tests
- ✅ Demo application showing all features
- ✅ Binary-compatible with future Python version

## Quick Start

```bash
cd src/core_rust

# Build and test
cargo build --release
cargo test

# Run Connection demo
cargo run --bin connection-demo
```

## Features

### Core Implementation

- **32-byte packed structure** - Naturally aligned, zero padding
- **40+ connection types** - Organized in 11 semantic categories
- **8-level activation** - Bitfield for selective space influence
- **Physical force model** - Attraction/repulsion with rigidity
- **Lifecycle tracking** - Creation time, last activation, age
- **Type-safe enums** - Compile-time validation
- **Zero-copy serialization** - using transmute

### Connection Types (11 Categories)

1. **Semantic** (0x00-0x0F) - Synonym, Antonym, Hypernym, Hyponym...
2. **Causal** (0x10-0x1F) - Cause, Effect, EnabledBy, PreventedBy...
3. **Temporal** (0x20-0x2F) - Before, After, During, Simultaneous...
4. **Spatial** (0x30-0x3F) - Near, Far, Inside, Outside, Above, Below...
5. **Logical** (0x40-0x4F) - And, Or, Not, Implies, Equivalent...
6. **Associative** (0x50-0x5F) - AssociatedWith, SimilarTo, RelatedTo...
7. **Structural** (0x60-0x6F) - PartOf, HasPart, MemberOf, HasMember...
8. **Functional** (0x70-0x7F) - UsedFor, CapableOf, RequiredFor...
9. **Emotional** (0x80-0x8F) - Likes, Dislikes, Fears, Desires...
10. **Rules/Metaphors** (0x90-0x9F) - Rule, Metaphor, Analogy...
11. **Dynamic** (0xA0-0xAF) - Transition, Transform, Evolve...

### Physical Force Model

```rust
// Attraction (positive strength)
connection.pull_strength = 0.80;
connection.preferred_distance = 0.10;  // Keep tokens close

// Repulsion (negative strength)
connection.pull_strength = -0.60;
connection.preferred_distance = 3.00;  // Push tokens apart

// Rigidity (how firm is the connection)
connection.set_rigidity(0.85);  // 0.0 = soft, 1.0 = rigid
```

**Physical formula:**
```
Force = (preferred_distance - current_distance) × rigidity × pull_strength
```

### Active Levels (Bitfield)

```rust
use neurograph_core::active_levels;

// Activate specific levels
connection.activate_level(active_levels::L5_COGNITIVE);
connection.activate_level(active_levels::L8_ABSTRACT);

// Check if level is active
if connection.is_level_active(active_levels::L8_ABSTRACT) {
    // Connection affects abstract space
}

// Predefined combinations
connection.active_levels = active_levels::COGNITIVE_ABSTRACT;  // L5 + L8
connection.active_levels = active_levels::EMOTIONAL_SOCIAL;    // L4 + L6
```

## API Highlights

```rust
use neurograph_core::{Connection, ConnectionType, active_levels, connection_flags};

// Create connection
let mut conn = Connection::new(token_a_id, token_b_id);

// Set type
conn.set_connection_type(ConnectionType::Cause);

// Configure physical parameters (precision x.xx)
conn.set_rigidity(0.85);
conn.pull_strength = 0.70;
conn.preferred_distance = 1.50;

// Set active levels
conn.active_levels = active_levels::COGNITIVE_ABSTRACT;

// Set flags
conn.set_flag(connection_flags::PERSISTENT);
conn.set_flag(connection_flags::REINFORCED);

// Activate (increment counter, update timestamp)
conn.activate();

// Validate
assert!(conn.validate().is_ok());

// Serialize (zero-copy)
let bytes = conn.to_bytes();  // [u8; 32]
let restored = Connection::from_bytes(&bytes);
```

## Binary Layout (32 bytes)

```
Offset  Size    Type      Field
------  ------  --------  ------------------
0-3     4       u32       token_a_id
4-7     4       u32       token_b_id
8       1       u8        connection_type
9       1       u8        rigidity
10      1       u8        active_levels
11      1       u8        flags
12-15   4       u32       activation_count
16-19   4       f32       pull_strength
20-23   4       f32       preferred_distance
24-27   4       u32       created_at
28-31   4       u32       last_activation
------  ------  --------  ------------------
TOTAL   32 bytes (naturally aligned)
```

## Connection Flags

- `ACTIVE` - Connection is active
- `BIDIRECTIONAL` - Can traverse both directions
- `PERSISTENT` - Should be saved
- `MUTABLE` - Can be modified
- `REINFORCED` - Recently reinforced
- `DECAYING` - Losing strength over time
- `USER_1`, `USER_2` - User-defined flags

## Active Levels

```
Bit 0: L1 Physical    - 0x01
Bit 1: L2 Sensory     - 0x02
Bit 2: L3 Motor       - 0x04
Bit 3: L4 Emotional   - 0x08
Bit 4: L5 Cognitive   - 0x10
Bit 5: L6 Social      - 0x20
Bit 6: L7 Temporal    - 0x40
Bit 7: L8 Abstract    - 0x80
```

## Test Coverage

All core functionality is tested:

- ✅ Connection size verification (exactly 32 bytes)
- ✅ Connection creation and initialization
- ✅ Connection type management
- ✅ Rigidity encoding/decoding
- ✅ Flag operations (set, clear, check)
- ✅ Active levels bitfield operations
- ✅ Activation tracking
- ✅ Serialization roundtrip
- ✅ Validation logic
- ✅ Age and lifecycle calculations

```bash
cargo test

running 10 tests
test connection::tests::test_connection_size ... ok
test connection::tests::test_connection_new ... ok
test connection::tests::test_connection_type ... ok
test connection::tests::test_rigidity ... ok
test connection::tests::test_flags ... ok
test connection::tests::test_active_levels ... ok
test connection::tests::test_activation ... ok
test connection::tests::test_serialization ... ok
test connection::tests::test_validation ... ok
test connection::tests::test_age ... ok

test result: ok. 10 passed
```

## Performance

- **Memory:** Exactly 32 bytes per connection (verified at compile-time)
- **Operations:** O(1) for all operations
- **Serialization:** Zero-copy using transmute
- **Cache-friendly:** Naturally aligned structure

## Use Cases

### Semantic Relations

```rust
// Synonyms (should be very close)
let mut syn = Connection::new(word1, word2);
syn.set_connection_type(ConnectionType::Synonym);
syn.pull_strength = 0.90;
syn.preferred_distance = 0.05;
syn.active_levels = active_levels::L8_ABSTRACT;
```

### Causal Relations

```rust
// Cause and effect
let mut cause = Connection::new(event1, event2);
cause.set_connection_type(ConnectionType::Cause);
cause.pull_strength = 0.70;
cause.preferred_distance = 1.00;
cause.active_levels = active_levels::COGNITIVE_ABSTRACT;
```

### Spatial Relations

```rust
// Physical proximity
let mut spatial = Connection::new(object1, object2);
spatial.set_connection_type(ConnectionType::Near);
spatial.pull_strength = 0.50;
spatial.preferred_distance = 0.50;
spatial.active_levels = active_levels::L1_PHYSICAL;
```

### Emotional Relations

```rust
// Likes/dislikes
let mut emotional = Connection::new(agent, object);
emotional.set_connection_type(ConnectionType::Likes);
emotional.pull_strength = 0.60;
emotional.preferred_distance = 0.80;
emotional.active_levels = active_levels::EMOTIONAL_SOCIAL;
```

## Compatibility

- ✅ Binary-compatible format (little-endian)
- ✅ Same memory layout as specification
- ✅ Ready for FFI bindings (Python, C, etc.)

## Documentation

- [Connection V2 Specification](docs/Connection%20V2.md) - Full spec
- [src/core_rust/src/connection.rs](src/core_rust/src/connection.rs) - Implementation
- [Architecture Blueprint](architecture_blueprint.json) - System overview

## Roadmap

### v0.13.0 (Current)
- ✅ Connection V1.0 Rust implementation
- ✅ 10+ unit tests
- ✅ Demo application

### v0.14.0 (Next)
- 📋 Grid V2.0 Rust implementation
- 📋 Spatial indexing (R-tree/Octree)
- 📋 Field physics simulation

### v0.15.0 (Future)
- 📋 Graph V2.0 Rust implementation
- 📋 FFI bindings (PyO3)
- 📋 Performance benchmarks

## Example: Building a Knowledge Graph

```rust
use neurograph_core::{Token, Connection, ConnectionType, active_levels};

// Create tokens
let mut cat = Token::new(1);
let mut animal = Token::new(2);
let mut dog = Token::new(3);

// Cat is-a Animal (hypernym)
let mut is_a = Connection::new(cat.id, animal.id);
is_a.set_connection_type(ConnectionType::Hypernym);
is_a.pull_strength = 0.80;
is_a.preferred_distance = 0.50;
is_a.active_levels = active_levels::L8_ABSTRACT;

// Dog is-a Animal
let mut is_a2 = Connection::new(dog.id, animal.id);
is_a2.set_connection_type(ConnectionType::Hypernym);
is_a2.pull_strength = 0.80;
is_a2.preferred_distance = 0.50;

// Cat and Dog are similar (but not same)
let mut similar = Connection::new(cat.id, dog.id);
similar.set_connection_type(ConnectionType::SimilarTo);
similar.pull_strength = 0.60;
similar.preferred_distance = 1.00;

// Activate connections to reinforce them
is_a.activate();
is_a2.activate();
similar.activate();
```

## Future Work: Connection v2.0

### Motivation for v2.0 (planned for v0.27.0+)

Connection v1.0 focuses on **physical force model** and **topology**. However, with the introduction of **Learner Module (v0.26.0)**, we need support for **Hebbian learning weights**.

### Current Limitations (v1.0)

**No built-in Hebbian weights:**
- Connection v1.0 (32 bytes) stores physical properties (`pull_strength`, `preferred_distance`)
- `pull_strength` is for physics, not learning
- No field for learned connection strength

**Workaround for v0.26.0:**
- Learner Module stores weights **separately** in `HashMap<EdgeId, f32>`
- This works but requires extra lookups
- Less cache-friendly than embedded weights

### Proposed Connection v2.0 Structure (40 bytes)

```rust
#[repr(C)]
pub struct Connection {
    // === Core fields (16 bytes) ===
    pub token_a_id: u32,
    pub token_b_id: u32,
    pub connection_type: u8,
    pub rigidity: u8,
    pub active_levels: u8,
    pub flags: u8,
    pub activation_count: u32,

    // === Physical model (8 bytes) ===
    pub pull_strength: f32,
    pub preferred_distance: f32,

    // === Learning model (8 bytes) - NEW in v2.0 ===
    pub hebbian_weight: f32,        // Learned connection strength (0.0-1.0)
    pub learning_rate: f32,         // Per-connection learning rate

    // === Lifecycle (8 bytes) ===
    pub created_at: u32,
    pub last_activation: u32,
}
```

**Size:** 40 bytes (32 + 8 for learning)

### Migration Path (v1.0 → v2.0)

#### Step 1: v0.26.0 - External weights (current approach)

```rust
// Learner stores weights separately
pub struct Learner {
    weights: HashMap<EdgeId, f32>,  // External storage
}
```

#### Step 2: v0.27.0+ - Connection v2.0 with embedded weights

```rust
// Weights moved into Connection
pub struct Connection {
    // ... existing fields
    pub hebbian_weight: f32,  // Now embedded!
    pub learning_rate: f32,
}

// Learner can access weights directly
pub struct Learner {
    // No more HashMap needed!
}
```

### Design Considerations for v2.0

#### 1. Weight semantics

- `pull_strength` = physical force (can be negative for repulsion)
- `hebbian_weight` = learned association strength (always 0.0-1.0)
- Both can coexist: physics for Grid, learning for Graph

#### 2. Learning rate per connection

- Different connection types learn at different rates:
  - `ASSOCIATION` → fast learning (high rate)
  - `CAUSALITY` → medium learning
  - `HIERARCHY` → slow learning (stable structure)
  - `DEFINITION` → no learning (fixed connections)

#### 3. Metadata field usage

- v1.0: 8-byte `metadata` field (currently unused)
- Could be repurposed for `hebbian_weight` + `learning_rate` without size increase
- **Trade-off:** Breaks binary compatibility with v1.0

### Compatibility Strategy

#### Option A: New Connection v2.0 type (40 bytes)

- ✅ Clean separation
- ✅ Both versions can coexist
- ❌ 25% memory increase per connection

#### Option B: Repurpose metadata field (32 bytes)

- ✅ No size increase
- ✅ More cache-friendly
- ❌ Breaks v1.0 compatibility

**Recommendation:** Option A for v2.0, keep v1.0 for non-learning connections.

### Timeline

- **v0.26.0** (current) - Learner with external weights
- **v0.27.0-v0.28.0** - Evaluate performance impact of external storage
- **v0.29.0+** - Implement Connection v2.0 if needed

### Notes for Implementation

When implementing Connection v2.0, consider:

1. **Backward compatibility:**
   - Read v1.0 files and auto-upgrade to v2.0
   - Initialize `hebbian_weight = 0.5`, `learning_rate = 0.01`

2. **Serialization:**
   - Add version marker to distinguish v1.0 vs v2.0
   - Support both formats in deserialization

3. **Memory management:**
   - Connections with `hebbian_weight = 0.0` can be pruned
   - Memory pools for 32-byte and 40-byte connections

4. **CDNA validation:**
   - Update CDNA to validate `hebbian_weight` range
   - Add constraints for `learning_rate` (e.g., 0.001-0.1)

---

## Summary

Version 0.13.0 successfully delivers a **production-ready Rust implementation** of Connection V1.0, providing:

- 🔗 **40+ connection types** across 11 categories
- 🧲 **Physical force model** for token attraction/repulsion
- 🎯 **Selective activation** on 8 semantic spaces
- ⚡ **High performance** with zero-copy operations
- ✅ **Comprehensive testing** with 10+ unit tests
- 📚 **Complete documentation** and examples

Combined with Token V2.0 (v0.12.0), this provides the foundation for building rich semantic networks in NeuroGraph OS.

---

**NeuroGraph OS** - Token-based computing for artificial intelligence
Version 0.13.0 "mvp_ConnectionR" - Connection Rust Implementation
