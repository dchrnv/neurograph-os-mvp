## Connection V2.0 - Learning-Capable Links

**Version:** 2.0 (v0.29.0)
**Status:** 🚧 In Development
**Language:** Rust 2021 Edition
**Dependencies:** Zero (pure Rust)
**Previous Version:** V1.0 (32 bytes, static connections)

## Philosophical Foundation: Static vs Dynamic Knowledge

### The Problem

In V1.0, Connections were **purely structural** - they described relationships between tokens but couldn't evolve based on experience. This created a fundamental tension:

- **Ontological relations** (IsA, Synonym, PartOf) are **facts about the world** - they shouldn't change
- **Causal relations** (Cause, Effect, EnabledBy) are **hypotheses about dynamics** - they should be refined through experience
- **Functional relations** (UsedFor, RequiredFor) are **learned behaviors** - they depend on context and outcomes

Example conflict:
```
Connection says: Action "open door" CAUSES State "room accessible"
But experience shows: Sometimes door is locked, action fails
Should the Connection be updated? Deleted? Have confidence score?
```

### The Solution: Hybrid Learning Model

V2.0 introduces **three tiers of mutability**:

#### 1. Immutable Connections (Ontological Truth)
- Semantic relations: Synonym, Antonym, Hypernym, Hyponym
- Structural facts: PartOf, HasPart, MemberOf
- Logical axioms: And, Or, Not, Implies

**Philosophy**: These are definitional truths, not learned from experience.
**Created by**: Manual knowledge base, WordNet import, ontology definition
**Cannot be modified by**: IntuitionEngine

#### 2. Learnable Connections (Causal Hypotheses)
- Causal relations: Cause, Effect, EnabledBy, PreventedBy
- Functional relations: UsedFor, CapableOf, RequiredFor
- Temporal patterns: Before, After, During

**Philosophy**: These are working hypotheses refined through observation.
**Created by**: Manual hypothesis, IntuitionEngine discovery
**Modified by**: IntuitionEngine proposals validated by Guardian
**Evidence-based**: Confidence score increases with supporting evidence, decreases with contradictions

#### 3. Hypothesis Connections (Tentative Learning)
- Newly discovered patterns not yet validated
- Temporary connections with expiration
- High learning rate, fast decay

**Philosophy**: These are experimental ideas the system is testing.
**Created by**: IntuitionEngine pattern discovery
**Lifecycle**: Promoted to Learnable if evidence accumulates, deleted if disproven
**Guardian oversight**: Can veto hypothesis creation in protected domains

### Biological Analogy: Synaptic Plasticity

This mirrors how biological synapses work:
- **Structural connections** (axon/dendrite wiring) = Immutable Connections
- **Synaptic strength** (neurotransmitter release probability) = Learnable Connection confidence
- **Transient potentiation** (temporary strength boost) = Hypothesis Connections

## Overview

This is the **official Rust implementation** of NeuroGraph Connection V2.0 - the 64-byte data structure representing **learning-capable** relationships between tokens in 8-dimensional semantic space.

## What's New in v2.0

**Major Changes:**
- 🆕 **64-byte structure** (expanded from 32 bytes)
- 🆕 **Learning capabilities** - connections can evolve based on experience
- 🆕 **Three mutability tiers** - Immutable, Learnable, Hypothesis
- 🆕 **Confidence tracking** - evidence-based belief in connection validity
- 🆕 **Learning rate & decay** - configurable adaptation speed
- 🆕 **Source tracking** - know which IntuitionEngine proposal created connection
- 🆕 **Guardian integration** - proposals validated before application
- 🆕 **IntuitionEngine integration** - can generate Connection modification proposals

**Carried Forward from V1.0:**
- ✅ 40+ connection types across 11 categories
- ✅ Bitfield support for 8 active levels
- ✅ Physical force model (pull_strength, preferred_distance)
- ✅ Activation tracking and lifecycle management
- ✅ Zero-copy serialization
- ✅ Type-safe enums

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

## Binary Layout V2.0 (64 bytes)

```
Offset  Size    Type      Field                Description
------  ------  --------  ------------------   -----------------------------
                          [CORE STRUCTURE - 32 bytes, compatible with V1.0]
0-3     4       u32       token_a_id           ID of first token
4-7     4       u32       token_b_id           ID of second token
8       1       u8        connection_type      Type of connection (0-255)
9       1       u8        rigidity             Rigidity (0-255 = 0.0-1.0)
10      1       u8        active_levels        Bitmask of active levels
11      1       u8        flags                Connection flags
12-15   4       u32       activation_count     Times activated
16-19   4       f32       pull_strength        Attraction/repulsion force
20-23   4       f32       preferred_distance   Ideal distance between tokens
24-27   4       u32       created_at           Unix timestamp (creation)
28-31   4       u32       last_activation      Unix timestamp (last use)

                          [LEARNING EXTENSION - 32 bytes, NEW in V2.0]
32      1       u8        mutability           0=Immutable, 1=Learnable, 2=Hypothesis
33      1       u8        confidence           0-255 represents 0.0-1.0
34-35   2       u16       evidence_count       Number of supporting observations
36-39   4       u32       last_update          Unix timestamp (last learning update)
40      1       u8        learning_rate        Adaptation speed (0-255 = 0.0-1.0)
41      1       u8        decay_rate           Hypothesis decay (0-255 = 0.0-1.0)
42-43   2       u16       _reserved_future     Reserved for future use
44-47   4       u32       source_id            Creator ID (0=manual, >0=proposal ID)
48-63   16      [u8; 16]  _reserved            Reserved for future expansion
------  ------  --------  ------------------   -----------------------------
TOTAL   64 bytes (cache-line aligned, 2^6)
```

### Mutability Levels

```rust
#[repr(u8)]
pub enum ConnectionMutability {
    /// Cannot be changed by learning (ontological facts)
    Immutable = 0,

    /// Can be refined by IntuitionEngine (causal hypotheses)
    Learnable = 1,

    /// Temporary hypothesis with expiration (experimental)
    Hypothesis = 2,
}
```

### Default Values by Mutability

**Immutable Connection:**
- `confidence`: 255 (1.0) - absolute certainty
- `evidence_count`: 0 (not applicable)
- `learning_rate`: 0 (cannot learn)
- `decay_rate`: 0 (never decays)

**Learnable Connection:**
- `confidence`: 128 (0.5) - moderate initial belief
- `evidence_count`: 1 (starts with one observation)
- `learning_rate`: 32 (0.125) - slow, careful learning
- `decay_rate`: 0 (persists without evidence)

**Hypothesis Connection:**
- `confidence`: 64 (0.25) - low initial belief
- `evidence_count`: 1 (tentative)
- `learning_rate`: 128 (0.5) - fast learning
- `decay_rate`: 16 (0.0625) - decays ~6% per cycle without evidence

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

## Use Cases V2.0 - Learning Examples

### 1. Immutable Semantic Relations (Ontological Facts)

```rust
// Synonyms (definitional truth, cannot change)
let mut syn = Connection::new(word1, word2);
syn.set_connection_type(ConnectionType::Synonym);
syn.pull_strength = 0.90;
syn.preferred_distance = 0.05;
syn.active_levels = active_levels::L8_ABSTRACT;
syn.mutability = ConnectionMutability::Immutable;  // ← NEW
syn.confidence = 255;  // Absolute certainty
syn.learning_rate = 0; // Cannot learn

// This connection will NEVER be modified by IntuitionEngine
// Guardian will reject any proposals to change it
```

### 2. Learnable Causal Relations (Refined Through Experience)

```rust
// Initial hypothesis: "Action X causes State Y"
let mut cause = Connection::new(action_x, state_y);
cause.set_connection_type(ConnectionType::Cause);
cause.pull_strength = 0.70;
cause.preferred_distance = 1.00;
cause.active_levels = active_levels::COGNITIVE_ABSTRACT;
cause.mutability = ConnectionMutability::Learnable;  // ← NEW
cause.confidence = 128;        // 0.5 - moderate initial belief
cause.evidence_count = 1;       // One initial observation
cause.learning_rate = 32;       // 0.125 - slow learning
cause.source_id = 0;            // Manually created

// IntuitionEngine observes 10 successes → increases confidence
// IntuitionEngine observes 3 failures → decreases confidence
// Final confidence = f(success_rate, evidence_count)
```

### 3. Hypothesis Connections (Discovered Patterns)

```rust
// IntuitionEngine discovers: "State A often followed by State B"
let mut hypothesis = Connection::new(state_a, state_b);
hypothesis.set_connection_type(ConnectionType::After);
hypothesis.pull_strength = 0.40;  // Weak initial force
hypothesis.preferred_distance = 1.50;
hypothesis.active_levels = active_levels::L7_TEMPORAL;
hypothesis.mutability = ConnectionMutability::Hypothesis;  // ← NEW
hypothesis.confidence = 64;         // 0.25 - low initial belief
hypothesis.evidence_count = 5;       // Seen 5 times
hypothesis.learning_rate = 128;      // 0.5 - fast learning
hypothesis.decay_rate = 16;          // Decays without evidence
hypothesis.source_id = 12345;        // Created by IntuitionEngine proposal #12345

// Lifecycle:
// - If evidence accumulates → promoted to Learnable
// - If contradicted → confidence drops → deleted when < threshold
// - If no new evidence → decays over time → eventually deleted
```

### 4. IntuitionEngine Proposal Flow

```rust
// IntuitionEngine analyzes ExperienceStream
// Finds pattern: When in state X, action A succeeds more than action B

// Generate proposal to CREATE or MODIFY connection
let proposal = Proposal::ModifyConnection {
    connection_id: 67890,
    field: ConnectionField::Confidence,
    old_value: 128,  // Current confidence: 0.5
    new_value: 160,  // Proposed confidence: 0.625
    justification: "Action succeeded in 15/20 trials (75%), t-test p<0.01",
    evidence: vec![event_id_1, event_id_2, ...],  // Supporting ExperienceEvents
    expected_impact: 0.15,  // Expected improvement in reward
};

// Guardian validates:
// 1. Is connection Learnable or Hypothesis? (not Immutable)
// 2. Is evidence statistically significant?
// 3. Does change respect CDNA constraints?
// 4. Is confidence delta reasonable? (not jumping 0.1 → 1.0)

// If accepted:
connection.confidence = 160;
connection.evidence_count += 15;
connection.last_update = now();
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
