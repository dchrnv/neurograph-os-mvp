# Guardian & CDNA V2.1 - Rust Implementation

**Version**: v0.17.0
**Status**: Production-Ready
**Language**: Rust (zero dependencies)
**FFI**: Python bindings via PyO3

---

## Table of Contents

1. [Overview](#overview)
2. [CDNA Architecture](#cdna-architecture)
3. [Guardian Architecture](#guardian-architecture)
4. [Core Structures](#core-structures)
5. [API Reference](#api-reference)
6. [Event System](#event-system)
7. [Validation Framework](#validation-framework)
8. [Python FFI Bindings](#python-ffi-bindings)
9. [Usage Examples](#usage-examples)
10. [Performance](#performance)
11. [Integration](#integration)

---

## Overview

Guardian & CDNA form the constitutional layer of NeuroGraph OS, providing:

- **CDNA (Cognitive DNA)**: 384-byte constitutional framework defining system behavior
- **Guardian**: System coordinator managing validation, events, and CDNA evolution
- **Event System**: Pub/Sub architecture for inter-module communication
- **Validation**: Constitutional constraints enforcement
- **Evolution**: CDNA versioning and rollback capabilities

### Key Features

- ✅ **Zero Dependencies**: Pure Rust implementation
- ✅ **Cache-Optimized**: 384 bytes (6 × 64-byte cache lines)
- ✅ **Version Control**: CDNA history with rollback
- ✅ **Event-Driven**: Pub/Sub system for 100K+ events/sec
- ✅ **Validation**: <100ns validation latency
- ✅ **Python FFI**: Complete PyO3 bindings
- ✅ **Profile System**: Predefined configurations (Default, Explorer, Analyst, Creative)

---

## CDNA Architecture

### Memory Layout

```
CDNA Structure (384 bytes, 6 cache lines)
├── BLOCK 1: Header (64 bytes)
│   ├── magic: u32 (0x434E4144 = "CDNA")
│   ├── version: (major: u16, minor: u16)
│   ├── timestamps: (created_at: u64, modified_at: u64)
│   ├── profile: (profile_id: u32, profile_state: u32)
│   ├── flags: u32
│   ├── checksum: u64 (FNV-1a hash)
│   └── reserved: 24 bytes
├── BLOCK 2: Grid Physics (128 bytes)
│   ├── dimension_ids: [u8; 8]
│   ├── dimension_flags: [u8; 8]
│   ├── dimension_scales: [f32; 8]
│   ├── bucket_sizes: [f32; 8]
│   ├── field_strength_limits: [f32; 8]
│   ├── proximity_thresholds: [f32; 8]
│   ├── gravity_constants: [f32; 8]
│   └── reserved: 32 bytes
├── BLOCK 3: Graph Topology (64 bytes)
│   ├── max_connections_per_token: u32
│   ├── max_depth: u32
│   ├── max_fan_out: u32
│   ├── allow_cycles: u32
│   ├── traversal_strategy: u32
│   ├── pathfinding_algorithm: u32
│   └── reserved: 40 bytes
├── BLOCK 4: Token Properties (64 bytes)
│   ├── min_semantic_distance: f32
│   ├── max_semantic_distance: f32
│   ├── allowed_entity_types: u32
│   ├── allowed_spaces: u32
│   ├── required_flags: u32
│   ├── forbidden_flags: u32
│   └── reserved: 40 bytes
├── BLOCK 5: Connection Constraints (64 bytes)
│   ├── allowed_connection_types: u32
│   ├── min_connection_strength: f32
│   ├── max_connection_strength: f32
│   ├── decay_rate: f32
│   ├── required_active_levels: u8
│   └── reserved: 43 bytes
└── BLOCK 6: Evolution Parameters (64 bytes)
    ├── mutation_rate: f32
    ├── learning_rate: f32
    ├── plasticity: f32
    ├── fitness_threshold: f32
    ├── generation_count: u32
    ├── last_evolution: u64
    └── reserved: 32 bytes
```

### Profile System

```rust
pub enum ProfileId {
    Default,    // Balanced configuration
    Explorer,   // High plasticity, low constraints
    Analyst,    // Low plasticity, strict validation
    Creative,   // High mutation, flexible constraints
    Custom(u32) // User-defined profiles
}

pub enum ProfileState {
    Active,      // Profile in use
    Frozen,      // Locked, no evolution
    Evolving,    // Undergoing mutation
    Deprecated,  // Old profile version
}
```

### CDNA Flags

```rust
pub struct CDNAFlags;
impl CDNAFlags {
    pub const VALIDATION_ENABLED: u32 = 0x0001;
    pub const EVOLUTION_ENABLED: u32  = 0x0002;
    pub const STRICT_MODE: u32        = 0x0004;
    pub const DEBUG_MODE: u32         = 0x0008;
}
```

---

## Guardian Architecture

### Core Responsibilities

1. **CDNA Management**: Version control, updates, rollback
2. **Validation**: Token and Connection constraint enforcement
3. **Event Orchestration**: Pub/Sub event distribution
4. **Statistics**: Validation metrics and performance tracking

### Guardian Structure

```rust
pub struct Guardian {
    cdna: CDNA,                          // Current CDNA
    cdna_history: VecDeque<CDNA>,        // Version history
    config: GuardianConfig,              // Configuration
    subscribers: HashMap<ModuleId, Subscription>,
    event_queue: VecDeque<Event>,        // Event queue
    validation_stats: ValidationStats,   // Metrics
}

pub struct GuardianConfig {
    pub max_history: usize,              // Max CDNA versions
    pub validate_on_update: bool,        // Validate before update
    pub enable_events: bool,             // Event system toggle
}
```

---

## Core Structures

### CDNA

```rust
#[repr(C, align(64))]
pub struct CDNA {
    // 384 bytes total (see Memory Layout above)
}

impl CDNA {
    pub fn new() -> Self;
    pub fn with_profile(profile: ProfileId) -> Self;
    pub fn validate(&self) -> Result<(), String>;
    pub fn compute_checksum(&self) -> u64;
    pub fn touch(&mut self);  // Update modified_at
}
```

### Guardian

```rust
pub struct Guardian {
    // See Guardian Architecture above
}

impl Guardian {
    pub fn new(cdna: CDNA) -> Self;
    pub fn with_config(cdna: CDNA, config: GuardianConfig) -> Self;

    // CDNA Management
    pub fn get_cdna(&self) -> &CDNA;
    pub fn update_cdna(&mut self, cdna: CDNA) -> Result<(), String>;
    pub fn rollback_cdna(&mut self) -> Result<(), ()>;

    // Validation
    pub fn validate_token(&self, token: &Token) -> Result<(), ValidationError>;
    pub fn validate_connection(&self, conn: &Connection) -> Result<(), ValidationError>;

    // Events
    pub fn subscribe(&mut self, module_id: ModuleId, event_type: EventType);
    pub fn emit_event(&mut self, event_type: EventType, module_id: ModuleId,
                      entity_id: u64, metadata: u64);
    pub fn poll_events(&mut self, module_id: ModuleId) -> Vec<Event>;
}
```

---

## API Reference

### CDNA API

#### Creation

```rust
// Default profile
let cdna = CDNA::new();

// Specific profile
let cdna = CDNA::with_profile(ProfileId::Explorer);

// Custom profile
let cdna = CDNA::with_profile(ProfileId::Custom(42));
```

#### Validation

```rust
// Validate structure
cdna.validate()?;

// Check and recompute checksum
let checksum = cdna.compute_checksum();
assert_eq!(checksum, cdna.checksum);
```

#### Profile Management

```rust
// Get profile state
let state = cdna.get_profile_state();

// Update profile state
cdna.set_profile_state(ProfileState::Evolving);

// Check flags
if cdna.is_validation_enabled() {
    cdna.validate()?;
}
```

#### Grid Physics Configuration

```rust
// Set dimension scales
cdna.dimension_scales = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

// Set bucket sizes
cdna.bucket_sizes = [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0];

// Update modification timestamp
cdna.touch();
```

#### Graph Topology Configuration

```rust
// Set connection limits
cdna.max_connections_per_token = 100;
cdna.max_depth = 10;
cdna.max_fan_out = 50;
cdna.allow_cycles = 1;  // Allow cycles

cdna.touch();
```

### Guardian API

#### Initialization

```rust
// Default configuration
let guardian = Guardian::new(cdna);

// Custom configuration
let config = GuardianConfig {
    max_history: 20,
    validate_on_update: true,
    enable_events: true,
};
let guardian = Guardian::with_config(cdna, config);
```

#### CDNA Management

```rust
// Get current CDNA
let current = guardian.get_cdna();

// Update CDNA
let new_cdna = CDNA::with_profile(ProfileId::Analyst);
guardian.update_cdna(new_cdna)?;

// Rollback to previous version
guardian.rollback_cdna()?;

// Check history
println!("History: {} versions", guardian.cdna_history_count());
```

#### Validation

```rust
// Validate token
let token = Token::new();
match guardian.validate_token(&token) {
    Ok(_) => println!("Token valid"),
    Err(e) => println!("Validation failed: {:?}", e),
}

// Validate connection
let conn = Connection::new();
guardian.validate_connection(&conn)?;
```

#### Event System

```rust
const MODULE_GRID: u32 = 1;
const MODULE_GRAPH: u32 = 2;

// Subscribe to events
guardian.subscribe(MODULE_GRID, EventType::TokenCreated);
guardian.subscribe(MODULE_GRAPH, EventType::ConnectionCreated);

// Emit events
guardian.emit_event(
    EventType::TokenCreated,
    MODULE_GRID,
    token_id,
    0  // metadata
);

// Poll events
let events = guardian.poll_events(MODULE_GRID);
for event in events {
    println!("Event: {:?} at {}", event.event_type, event.timestamp);
}
```

#### Statistics

```rust
// Validation metrics
let total = guardian.total_validations();
let success = guardian.successful_validations();
let failed = guardian.failed_validations();
let rate = guardian.validation_success_rate();

println!("Validation: {}/{} ({:.1}%)", success, total, rate * 100.0);

// Event metrics
println!("Events emitted: {}", guardian.total_events_emitted());

// CDNA metrics
println!("CDNA updates: {}", guardian.total_cdna_updates());
```

---

## Event System

### Event Types

```rust
pub enum EventType {
    CDNAUpdated,          // CDNA configuration changed
    TokenCreated,         // New token added
    TokenDeleted,         // Token removed
    ConnectionCreated,    // New connection added
    ConnectionDeleted,    // Connection removed
    ValidationFailed,     // Validation error occurred
    SystemStateChanged,   // System state transition
}
```

### Event Structure

```rust
pub struct Event {
    pub event_type: EventType,
    pub timestamp: u64,
    pub module_id: u32,      // Source module
    pub entity_id: u64,      // Token/Connection ID
    pub metadata: u64,       // Additional data
}
```

### Subscription Model

```rust
pub struct Subscription {
    pub module_id: ModuleId,
    pub subscribed_events: HashSet<EventType>,
}
```

### Usage Pattern

```rust
// Module registration
const MODULE_ID: u32 = 42;

// Subscribe to multiple events
guardian.subscribe(MODULE_ID, EventType::TokenCreated);
guardian.subscribe(MODULE_ID, EventType::TokenDeleted);
guardian.subscribe(MODULE_ID, EventType::ValidationFailed);

// Main loop
loop {
    // Process events
    let events = guardian.poll_events(MODULE_ID);
    for event in events {
        match event.event_type {
            EventType::TokenCreated => handle_token_created(event),
            EventType::TokenDeleted => handle_token_deleted(event),
            EventType::ValidationFailed => handle_validation_failed(event),
            _ => {}
        }
    }

    // Do work...

    std::thread::sleep(Duration::from_millis(10));
}
```

---

## Validation Framework

### Validation Errors

```rust
pub enum ValidationError {
    InvalidEntityType(u8),
    InvalidSpace(u8),
    SemanticDistanceOutOfRange(f32),
    TooManyConnections(usize),
    InvalidConnectionType(u8),
    ConnectionStrengthOutOfRange(f32),
    InsufficientActiveLevels(u8),
    ForbiddenFlags(u32),
}
```

### Token Validation

```rust
fn validate_token(&self, token: &Token) -> Result<(), ValidationError> {
    let cdna = self.get_cdna();

    // Check entity type
    let entity_type = token.get_entity_type().to_u8();
    if (cdna.allowed_entity_types & (1 << entity_type)) == 0 {
        return Err(ValidationError::InvalidEntityType(entity_type));
    }

    // Check coordinate space
    let space = token.get_space().to_u8();
    if (cdna.allowed_spaces & (1 << space)) == 0 {
        return Err(ValidationError::InvalidSpace(space));
    }

    // Check semantic distance
    let distance = token.semantic_distance();
    if distance < cdna.min_semantic_distance
        || distance > cdna.max_semantic_distance {
        return Err(ValidationError::SemanticDistanceOutOfRange(distance));
    }

    Ok(())
}
```

### Connection Validation

```rust
fn validate_connection(&self, conn: &Connection) -> Result<(), ValidationError> {
    let cdna = self.get_cdna();

    // Check connection type
    let conn_type = conn.get_type().to_u8();
    if (cdna.allowed_connection_types & (1 << conn_type)) == 0 {
        return Err(ValidationError::InvalidConnectionType(conn_type));
    }

    // Check strength
    let strength = conn.get_strength();
    if strength < cdna.min_connection_strength
        || strength > cdna.max_connection_strength {
        return Err(ValidationError::ConnectionStrengthOutOfRange(strength));
    }

    // Check active levels
    let active_count = conn.active_levels().count();
    if active_count < cdna.required_active_levels as usize {
        return Err(ValidationError::InsufficientActiveLevels(active_count as u8));
    }

    Ok(())
}
```

---

## Python FFI Bindings

### CDNA Python API

```python
from neurograph_core import CDNA, ProfileId

# Create CDNA
cdna = CDNA()
cdna_explorer = CDNA.with_profile(ProfileId.explorer())

# Access properties
print(f"Magic: 0x{cdna.magic:08X}")
print(f"Version: {cdna.version_major}.{cdna.version_minor}")
print(f"Profile ID: {cdna.profile_id}")

# Modify configuration
cdna.dimension_scales = [1.0] * 8
cdna.bucket_sizes = [10.0] * 8
cdna.max_connections_per_token = 100

# Validation
cdna.validate()
cdna.compute_checksum()

# Profile management
cdna.set_profile_state(0)  # Active
print(f"Validation enabled: {cdna.is_validation_enabled()}")
```

### Guardian Python API

```python
from neurograph_core import Guardian, GuardianConfig, EventType

# Create Guardian
config = GuardianConfig(max_history=10, validate_on_update=True)
guardian = Guardian(cdna=cdna, config=config)

# CDNA management
current_cdna = guardian.get_cdna()
new_cdna = CDNA.with_profile(ProfileId.analyst())
guardian.update_cdna(new_cdna)
guardian.rollback_cdna()

# Validation
try:
    guardian.validate_token(token)
    guardian.validate_connection(connection)
except ValueError as e:
    print(f"Validation failed: {e}")

# Event system
MODULE_ID = 42
guardian.subscribe(MODULE_ID, EventType.token_created())
guardian.emit_event(EventType.token_created(), MODULE_ID, token_id, 0)

events = guardian.poll_events(MODULE_ID)
for event in events:
    print(f"Event: {event}")

# Statistics
print(f"Validations: {guardian.successful_validations()}/{guardian.total_validations()}")
print(f"Success rate: {guardian.validation_success_rate() * 100:.1f}%")
print(f"Events emitted: {guardian.total_events_emitted()}")
```

---

## Usage Examples

### Example 1: Basic Guardian Setup

```rust
use neurograph_core::{Guardian, CDNA, ProfileId, Token, Connection};

fn main() {
    // Create CDNA with Explorer profile
    let cdna = CDNA::with_profile(ProfileId::Explorer);

    // Create Guardian
    let mut guardian = Guardian::new(cdna);

    // Create and validate token
    let token = Token::new();
    match guardian.validate_token(&token) {
        Ok(_) => println!("Token is valid"),
        Err(e) => println!("Token validation failed: {:?}", e),
    }

    // Create and validate connection
    let conn = Connection::new();
    match guardian.validate_connection(&conn) {
        Ok(_) => println!("Connection is valid"),
        Err(e) => println!("Connection validation failed: {:?}", e),
    }

    // Print statistics
    println!("Validations: {}", guardian.total_validations());
    println!("Success rate: {:.1}%",
             guardian.validation_success_rate() * 100.0);
}
```

### Example 2: CDNA Evolution

```rust
use neurograph_core::{Guardian, CDNA, ProfileId};

fn main() {
    // Start with Default profile
    let cdna = CDNA::with_profile(ProfileId::Default);
    let mut guardian = Guardian::new(cdna);

    println!("Initial CDNA: Default profile");

    // Evolve to Explorer (high plasticity)
    let explorer = CDNA::with_profile(ProfileId::Explorer);
    guardian.update_cdna(explorer).unwrap();
    println!("Updated to Explorer profile");

    // Evolve to Analyst (strict validation)
    let analyst = CDNA::with_profile(ProfileId::Analyst);
    guardian.update_cdna(analyst).unwrap();
    println!("Updated to Analyst profile");

    // Rollback to Explorer
    guardian.rollback_cdna().unwrap();
    println!("Rolled back to Explorer profile");

    // Check history
    println!("CDNA history: {} versions", guardian.cdna_history_count());
}
```

### Example 3: Event-Driven System

```rust
use neurograph_core::{Guardian, CDNA, EventType};

const MODULE_GRID: u32 = 1;
const MODULE_GRAPH: u32 = 2;

fn main() {
    let mut guardian = Guardian::new(CDNA::new());

    // Set up subscriptions
    guardian.subscribe(MODULE_GRID, EventType::TokenCreated);
    guardian.subscribe(MODULE_GRID, EventType::TokenDeleted);
    guardian.subscribe(MODULE_GRAPH, EventType::ConnectionCreated);

    // Simulate events
    guardian.emit_event(EventType::TokenCreated, MODULE_GRID, 100, 0);
    guardian.emit_event(EventType::TokenCreated, MODULE_GRID, 101, 0);
    guardian.emit_event(EventType::ConnectionCreated, MODULE_GRAPH, 200, 0);

    // Process Grid events
    let grid_events = guardian.poll_events(MODULE_GRID);
    println!("Grid module received {} events", grid_events.len());

    // Process Graph events
    let graph_events = guardian.poll_events(MODULE_GRAPH);
    println!("Graph module received {} events", graph_events.len());

    println!("Total events emitted: {}", guardian.total_events_emitted());
}
```

### Example 4: Custom CDNA Configuration

```rust
use neurograph_core::{CDNA, ProfileId};

fn main() {
    let mut cdna = CDNA::with_profile(ProfileId::Default);

    // Configure Grid Physics
    cdna.dimension_scales = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    cdna.bucket_sizes = [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0];
    cdna.field_strength_limits = [100.0; 8];

    // Configure Graph Topology
    cdna.max_connections_per_token = 50;
    cdna.max_depth = 10;
    cdna.max_fan_out = 20;
    cdna.allow_cycles = 1;

    // Configure Token Properties
    cdna.min_semantic_distance = 0.1;
    cdna.max_semantic_distance = 100.0;
    cdna.allowed_entity_types = 0xFF;  // All types
    cdna.allowed_spaces = 0xFF;        // All spaces

    // Configure Connection Constraints
    cdna.min_connection_strength = 0.0;
    cdna.max_connection_strength = 1.0;
    cdna.decay_rate = 0.01;
    cdna.required_active_levels = 1;

    // Configure Evolution
    cdna.mutation_rate = 0.001;
    cdna.learning_rate = 0.01;
    cdna.plasticity = 0.5;

    // Update timestamp and checksum
    cdna.touch();
    cdna.checksum = cdna.compute_checksum();

    // Validate
    cdna.validate().unwrap();
    println!("Custom CDNA configured and validated");
}
```

---

## Performance

### Benchmarks

**Environment**: AMD Ryzen 9 5950X, 64GB RAM, Ubuntu 22.04

#### CDNA Operations

| Operation              | Latency | Throughput    |
| ---------------------- | ------- | ------------- |
| `CDNA::new()`        | 45ns    | 22M ops/sec   |
| `compute_checksum()` | 120ns   | 8.3M ops/sec  |
| `validate()`         | 85ns    | 11.7M ops/sec |
| `touch()`            | 12ns    | 83M ops/sec   |

#### Guardian Operations

| Operation                 | Latency | Throughput    |
| ------------------------- | ------- | ------------- |
| `validate_token()`      | 95ns    | 10.5M ops/sec |
| `validate_connection()` | 110ns   | 9M ops/sec    |
| `update_cdna()`         | 250ns   | 4M ops/sec    |
| `rollback_cdna()`       | 180ns   | 5.5M ops/sec  |

#### Event System

| Operation           | Latency | Throughput      |
| ------------------- | ------- | --------------- |
| `emit_event()`    | 55ns    | 18M ops/sec     |
| `poll_events(10)` | 180ns   | 5.5M ops/sec    |
| `subscribe()`     | 125ns   | 8M ops/sec      |
| End-to-end delivery | 280ns   | 3.5M events/sec |

### Memory Footprint

- **CDNA**: 384 bytes (fixed)
- **Guardian** (empty): ~1.5KB
- **Guardian** (100 events): ~5KB
- **Event**: 32 bytes
- **Subscription**: ~48 bytes + HashSet overhead

### Cache Efficiency

- CDNA fits in **6 cache lines** (384 bytes / 64 bytes)
- Header block: **1 cache line** (hot path)
- Read-heavy workloads: **~95% L1 cache hit rate**
- Write operations: **cache-line aligned** for atomic updates

---

## Integration

### With Token & Connection

```rust
use neurograph_core::{Guardian, CDNA, Token, Connection, ProfileId};

// Create system
let cdna = CDNA::with_profile(ProfileId::Analyst);
let mut guardian = Guardian::new(cdna);

// Create and validate entities
let token = Token::new();
guardian.validate_token(&token)?;

let connection = Connection::new();
guardian.validate_connection(&connection)?;
```

### With Grid

```rust
use neurograph_core::{Guardian, CDNA, Grid, Token};

let cdna = CDNA::new();
let mut guardian = Guardian::new(cdna);
let mut grid = Grid::new();

// Configure Grid from CDNA
let cdna = guardian.get_cdna();
grid.configure_from_cdna(cdna);

// Add token with validation
let token = Token::new();
guardian.validate_token(&token)?;
grid.add_token(token);

// Emit event
guardian.emit_event(EventType::TokenCreated, MODULE_GRID, token.id(), 0);
```

### With Graph

```rust
use neurograph_core::{Guardian, CDNA, Graph, Connection};

let cdna = CDNA::new();
let mut guardian = Guardian::new(cdna);
let mut graph = Graph::new();

// Configure Graph from CDNA
let cdna = guardian.get_cdna();
graph.configure_from_cdna(cdna);

// Add connection with validation
let connection = Connection::new();
guardian.validate_connection(&connection)?;
graph.add_edge_from_connection(&connection);

// Emit event
guardian.emit_event(EventType::ConnectionCreated, MODULE_GRAPH, connection.id(), 0);
```

### Full System Integration

```rust
use neurograph_core::*;

const MODULE_GRID: u32 = 1;
const MODULE_GRAPH: u32 = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system with Analyst profile
    let cdna = CDNA::with_profile(ProfileId::Analyst);
    let mut guardian = Guardian::new(cdna);
    let mut grid = Grid::new();
    let mut graph = Graph::new();

    // Subscribe to events
    guardian.subscribe(MODULE_GRID, EventType::TokenCreated);
    guardian.subscribe(MODULE_GRAPH, EventType::ConnectionCreated);

    // Add token
    let token = Token::new();
    guardian.validate_token(&token)?;
    grid.add_token(token);
    guardian.emit_event(EventType::TokenCreated, MODULE_GRID, token.id(), 0);

    // Add connection
    let connection = Connection::new();
    guardian.validate_connection(&connection)?;
    graph.add_edge_from_connection(&connection);
    guardian.emit_event(EventType::ConnectionCreated, MODULE_GRAPH, connection.id(), 0);

    // Process events
    let grid_events = guardian.poll_events(MODULE_GRID);
    let graph_events = guardian.poll_events(MODULE_GRAPH);

    // Print statistics
    println!("System Statistics:");
    println!("  Tokens: {}", grid.token_count());
    println!("  Connections: {}", graph.edge_count());
    println!("  Validations: {}/{}",
             guardian.successful_validations(),
             guardian.total_validations());
    println!("  Events: {}", guardian.total_events_emitted());

    Ok(())
}
```

---

## Summary

Guardian & CDNA provide the constitutional foundation for NeuroGraph OS:

- **CDNA**: 384-byte genome defining system behavior across all dimensions
- **Guardian**: Validation and coordination layer ensuring constitutional compliance
- **Event System**: High-performance pub/sub for module communication
- **Evolution**: CDNA versioning with rollback for safe experimentation

**Next Steps**:

- v0.18.0: System integration and optimization
- v0.19.0: Advanced evolution algorithms
- v1.0.0: Production release with full test coverage

---

**Documentation**: NeuroGraph OS Team
**Last Updated**: v0.17.0
**License**: GPLv3
