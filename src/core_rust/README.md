# NeuroGraph Core (Rust)

Rust implementation of **NeuroGraph Token V2.0** - the fundamental data structure for token-based computing in 8-dimensional semantic space.

## Features

- ✅ **64-byte binary format** - Cache-friendly, packed structure
- ✅ **8 semantic spaces** (L1-L8) with 3D coordinates each
- ✅ **Zero dependencies** - Pure Rust implementation
- ✅ **Type-safe** - Strongly typed entity types and flags
- ✅ **Serializable** - Binary-compatible across languages
- ✅ **Validated** - Built-in validation logic
- ✅ **Well-tested** - Comprehensive unit tests

## Token V2.0 Specification

### Binary Layout (64 bytes)

```
Offset  Size    Field              Description
------  ------  -----------------  ---------------------------
0-47    48      coordinates        8 spaces × 3 axes × i16
48-51   4       id                 Unique identifier (u32)
52-53   2       flags              System + entity type + user flags
54-57   4       weight             Intensity/importance (f32)
58      1       field_radius       Spatial influence (u8, scale 100)
59      1       field_strength     Force magnitude (u8, scale 255)
60-63   4       timestamp          Unix timestamp (u32)
------  ------  -----------------  ---------------------------
TOTAL   64 bytes
```

### 8 Semantic Spaces

1. **L1 Physical** - 3D physical space (±327.67m, scale 100)
2. **L2 Sensory** - Perception (salience, valence, novelty, scale 10000)
3. **L3 Motor** - Motion (velocity, acceleration, angular, scale 1000)
4. **L4 Emotional** - VAD model (valence, arousal, dominance, scale 10000)
5. **L5 Cognitive** - Processing (load, abstraction, certainty, scale 10000)
6. **L6 Social** - Interaction (distance, status, affiliation, scale 10000)
7. **L7 Temporal** - Time (offset, duration, frequency, scale 100/1000)
8. **L8 Abstract** - Semantics (proximity, causality, modality, scale 10000)

## Quick Start

### Build

```bash
cd src/core_rust
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Run Demo

```bash
cargo run --bin token-demo
```

## Usage Example

```rust
use neurograph_core::{Token, CoordinateSpace, EntityType, flags};

// Create a new token
let mut token = Token::new(Token::create_id(12345, 0, 0));

// Set coordinates in physical space (precision x.xx)
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);

// Set coordinates in emotional space (VAD model, precision x.xx)
token.set_coordinates(CoordinateSpace::L4Emotional, 0.80, 0.60, 0.50);

// Set entity type and flags
token.set_entity_type(EntityType::Concept);
token.set_flag(flags::PERSISTENT);

// Set weight and field properties
token.weight = 0.75;
token.set_field_radius(1.5);
token.set_field_strength(0.85);

// Validate
assert!(token.validate().is_ok());

// Serialize/deserialize
let bytes = token.to_bytes();
let token_copy = Token::from_bytes(&bytes);
```

## API Reference

### Token Creation

- `Token::new(id: u32) -> Token` - Create new token
- `Token::create_id(local_id, entity_type, domain) -> u32` - Construct ID

### Coordinates

- `set_coordinates(space, x, y, z)` - Set coordinates (auto-encoded)
- `get_coordinates(space) -> [f32; 3]` - Get coordinates (auto-decoded)
- `encode_coordinate(value, space) -> i16` - Manual encoding
- `decode_coordinate(encoded, space) -> f32` - Manual decoding

### Entity Type & Flags

- `set_entity_type(type)` - Set entity type
- `get_entity_type() -> EntityType` - Get entity type
- `set_flag(flag)` - Set a flag
- `clear_flag(flag)` - Clear a flag
- `has_flag(flag) -> bool` - Check if flag is set

### Field Properties

- `set_field_radius(radius: f32)` - Set radius (0.0-2.55)
- `get_field_radius() -> f32` - Get radius
- `set_field_strength(strength: f32)` - Set strength (0.0-1.0)
- `get_field_strength() -> f32` - Get strength

### ID Components

- `local_id() -> u32` - Extract local ID (bits 0-23)
- `id_entity_type() -> u8` - Extract entity type from ID (bits 24-27)
- `domain() -> u8` - Extract domain (bits 28-31)

### Serialization

- `to_bytes() -> [u8; 64]` - Serialize to bytes
- `from_bytes(&[u8; 64]) -> Token` - Deserialize from bytes

### Validation

- `validate() -> Result<(), &str>` - Validate token structure

## Entity Types

- `Undefined` - Unspecified type
- `Object` - Physical object
- `Event` - Temporal event
- `State` - State representation
- `Process` - Running process
- `Concept` - Abstract concept
- `Relation` - Relationship
- `Pattern` - Detected pattern
- `Rule` - Logic rule
- `Goal` - Objective
- `Memory` - Stored memory
- `Sensor` - Sensor data
- `Actuator` - Actuator command
- `Controller` - Control signal
- `Buffer` - Data buffer
- `Reserved` - Reserved for future use

## System Flags

- `ACTIVE` - Token is active
- `PERSISTENT` - Should be persisted
- `MUTABLE` - Can be modified
- `SYNCHRONIZED` - Is synchronized
- `COMPRESSED` - Data is compressed
- `ENCRYPTED` - Data is encrypted
- `DIRTY` - Modified but not saved
- `LOCKED` - Token is locked

## Testing

Run the full test suite:

```bash
cargo test -- --nocapture
```

Run specific test:

```bash
cargo test test_coordinate_encoding -- --nocapture
```

## Performance

- Token size: **Exactly 64 bytes** (cache-line friendly)
- Encoding/decoding: **O(1)** operations
- Serialization: **Zero-copy** using transmute
- Memory layout: **Packed** for efficiency

## Version

Current version: **0.12.0** (mvp_TokenR)

## License

See LICENSE file in project root.

## Related

- Python implementation: `src/core/token/token_v2.py`
- Full specification: `docs/Token V2.md`
- Architecture: `architecture_blueprint.json`
