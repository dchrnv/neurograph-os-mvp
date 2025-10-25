# Integration Guide: Token + Connection + Grid

**Version:** 0.15.0
**Date:** 2025-10-25

---

## Overview

This guide demonstrates how to use **Token**, **Connection**, and **Grid** together to build powerful spatial computing applications in NeuroGraph OS.

All three components work seamlessly together:
- **Token V2.0** - Atomic data units with 8-dimensional coordinates
- **Connection V1.0** - Relationships with physical force models
- **Grid V2.0** - Spatial indexing for fast queries

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Basic Integration](#basic-integration)
3. [Rust Integration](#rust-integration)
4. [Python Integration](#python-integration)
5. [FastAPI Integration](#fastapi-integration)
6. [Advanced Patterns](#advanced-patterns)
7. [Performance Considerations](#performance-considerations)
8. [Best Practices](#best-practices)

---

## Quick Start

### Prerequisites

```bash
# Install Rust bindings
cd src/core_rust
maturin develop --release --features python

# Verify installation
python -c "from neurograph import Token, Connection, Grid; print('OK')"
```

### First Integration Example

```python
from neurograph import Token, Connection, Grid, CoordinateSpace, ConnectionType

# Create grid
grid = Grid()

# Create tokens
token1 = Token(1)
token1.set_coordinates(CoordinateSpace.L1Physical(), 0.0, 0.0, 0.0)
token1.set_active(True)

token2 = Token(2)
token2.set_coordinates(CoordinateSpace.L1Physical(), 5.0, 0.0, 0.0)
token2.set_active(True)

# Add to grid
grid.add(token1)
grid.add(token2)

# Create connection
conn = Connection(1, 2, ConnectionType.Proximity())
conn.preferred_distance = 500  # 5.0m
conn.pull_strength = 127  # 0.5
conn.set_active(True)

# Find neighbors
neighbors = grid.find_neighbors(1, 0, radius=10.0, max_results=5)
print(f"Found {len(neighbors)} neighbors")

# Calculate force
distance = token1.distance_to(token2, CoordinateSpace.L1Physical())
force = conn.calculate_force(distance)
print(f"Force: {force / 255.0:.3f}")
```

---

## Basic Integration

### Creating a Semantic Network

**Goal:** Build a network of concepts with spatial positioning and semantic relationships.

```python
from neurograph import Token, Connection, Grid, GridConfig
from neurograph import CoordinateSpace, EntityType, ConnectionType

# Create grid with custom configuration
config = GridConfig()
config.bucket_size = 5.0  # 5-unit buckets for spatial indexing
grid = Grid(config)

# Define concepts in abstract semantic space (L8)
concepts = {
    "dog": (0.0, 0.0, 0.0),
    "cat": (2.0, 1.0, 0.0),
    "animal": (1.0, 5.0, 0.0),
    "pet": (1.0, 1.0, 0.0),
}

# Create tokens
tokens = {}
for i, (name, (x, y, z)) in enumerate(concepts.items(), start=1):
    token = Token(i)
    token.set_coordinates(CoordinateSpace.L8Abstract(), x, y, z)
    token.set_entity_type(EntityType.Concept())
    token.set_active(True)

    grid.add(token)
    tokens[name] = token

# Create semantic connections
connections = []

# Hypernym relationships (is-a)
hypernyms = [
    ("dog", "animal", 0.90),
    ("cat", "animal", 0.90),
    ("dog", "pet", 0.85),
    ("cat", "pet", 0.85),
]

for source, target, strength in hypernyms:
    conn = Connection(
        tokens[source].id,
        tokens[target].id,
        ConnectionType.Hypernym()
    )
    conn.pull_strength = int(strength * 255)
    conn.rigidity = 200  # 0.80
    conn.set_active(True)
    connections.append(conn)

# Similarity relationship
conn = Connection(tokens["dog"].id, tokens["cat"].id, ConnectionType.Similar())
conn.pull_strength = 178  # 0.70
conn.rigidity = 153  # 0.60
conn.set_bidirectional(True)
conn.set_active(True)
connections.append(conn)

# Spatial query: Find semantic neighbors of "dog"
dog_id = tokens["dog"].id
neighbors = grid.find_neighbors(dog_id, 7, radius=3.0, max_results=10)  # L8Abstract = 7

print(f"Semantic neighbors of 'dog':")
concept_names = {v.id: k for k, v in tokens.items()}
for token_id, distance in neighbors:
    if token_id != dog_id:
        name = concept_names.get(token_id, f"Token{token_id}")
        print(f"  - {name}: distance = {distance:.2f}")
```

### Multi-Dimensional Positioning

**Goal:** Position tokens in multiple semantic spaces simultaneously.

```python
# Create token that exists in multiple spaces
token = Token(1)

# Physical position
token.set_coordinates(CoordinateSpace.L1Physical(), 10.0, 20.0, 5.0)

# Emotional state (VAD model)
token.set_coordinates(CoordinateSpace.L4Emotional(), 0.6, 0.5, 0.7)

# Abstract semantic position
token.set_coordinates(CoordinateSpace.L8Abstract(), 2.0, 3.0, 1.5)

token.set_active(True)
grid.add(token)

# Different neighbors in different spaces
physical_neighbors = grid.find_neighbors(1, 0, radius=5.0, max_results=5)  # L1
emotional_neighbors = grid.find_neighbors(1, 3, radius=1.0, max_results=5)  # L4
semantic_neighbors = grid.find_neighbors(1, 7, radius=3.0, max_results=5)   # L8

print(f"Physical neighbors: {len(physical_neighbors)}")
print(f"Emotional neighbors: {len(emotional_neighbors)}")
print(f"Semantic neighbors: {len(semantic_neighbors)}")
```

---

## Rust Integration

### Complete Rust Example

```rust
use neurograph_core::{
    Token, Connection, Grid, GridConfig,
    CoordinateSpace, EntityType, ConnectionType,
    token_flags, connection_flags,
};

fn main() {
    // Create grid
    let config = GridConfig {
        bucket_size: 10.0,
        density_threshold: 0.5,
        min_field_nodes: 3,
    };
    let mut grid = Grid::with_config(config);

    // Create tokens
    let mut token1 = Token::new(1);
    token1.set_coordinates(CoordinateSpace::L1Physical, 0.0, 0.0, 0.0);
    token1.set_entity_type(EntityType::Object);
    token1.set_flag(token_flags::ACTIVE);

    let mut token2 = Token::new(2);
    token2.set_coordinates(CoordinateSpace::L1Physical, 5.0, 0.0, 0.0);
    token2.set_entity_type(EntityType::Object);
    token2.set_flag(token_flags::ACTIVE);

    // Add to grid
    grid.add(token1).unwrap();
    grid.add(token2).unwrap();

    // Create connection
    let mut conn = Connection::new(1, 2);
    conn.set_connection_type(ConnectionType::Proximity);
    conn.preferred_distance = 500; // 5.0m
    conn.pull_strength = 127; // 0.5
    conn.rigidity = 178; // 0.7
    conn.set_flag(connection_flags::ACTIVE);

    // Find neighbors
    let neighbors = grid.find_neighbors(1, CoordinateSpace::L1Physical, 10.0, 10);
    println!("Found {} neighbors", neighbors.len());

    // Calculate distance and force
    let t1 = grid.get(1).unwrap();
    let t2 = grid.get(2).unwrap();
    let distance = t1.distance_to(t2, CoordinateSpace::L1Physical);

    let force = conn.calculate_force(distance);
    let force_normalized = force as f32 / 255.0;
    println!("Distance: {:.2}m, Force: {:.3}", distance, force_normalized);
}
```

### Field Physics in Rust

```rust
// Create tokens with field properties
let mut token1 = Token::new(1);
token1.set_coordinates(CoordinateSpace::L1Physical, 0.0, 0.0, 0.0);
token1.field_radius = 200; // 2.0m
token1.field_strength = 255; // 1.0
token1.set_flag(token_flags::ACTIVE);

let mut token2 = Token::new(2);
token2.set_coordinates(CoordinateSpace::L1Physical, 1.0, 0.0, 0.0);
token2.field_radius = 150; // 1.5m
token2.field_strength = 178; // 0.7
token2.set_flag(token_flags::ACTIVE);

grid.add(token1).unwrap();
grid.add(token2).unwrap();

// Calculate field influence at different points
for x in [0.0, 0.5, 1.0, 1.5, 2.0] {
    let influence = grid.calculate_field_influence(
        CoordinateSpace::L1Physical,
        x, 0.0, 0.0,
        3.0  // Search radius
    );
    println!("Influence at x={:.1}: {:.3}", x, influence);
}
```

---

## Python Integration

### Emotional Landscape

**Goal:** Model emotional states with field influence.

```python
from neurograph import Grid, create_emotional_token

# Create grid
grid = Grid()

# Create emotional tokens using VAD model
emotions = {
    "joy": (0.8, 0.7, 0.6, 1.5, 0.8),      # valence, arousal, dominance, field_r, field_s
    "sadness": (-0.7, 0.3, 0.3, 1.2, 0.6),
    "anger": (-0.5, 0.8, 0.7, 1.0, 0.7),
    "calm": (0.3, 0.2, 0.6, 2.0, 0.4),
}

tokens = {}
for i, (name, (v, a, d, field_r, field_s)) in enumerate(emotions.items(), start=1):
    token = create_emotional_token(i, valence=v, arousal=a, dominance=d)
    token.field_radius = int(field_r * 100)
    token.field_strength = int(field_s * 255)
    token.set_active(True)

    grid.add(token)
    tokens[name] = token

# Calculate emotional field influence at a point
influence = grid.calculate_field_influence(
    space=3,  # L4Emotional
    x=0.0, y=0.5, z=0.5,
    radius=3.0
)

print(f"Emotional field influence: {influence:.3f}")

# Find emotionally similar states
joy_neighbors = grid.find_neighbors(
    tokens["joy"].id,
    space=3,
    radius=2.0,
    max_results=5
)

print(f"States similar to joy:")
emotion_names = {v.id: k for k, v in tokens.items()}
for token_id, distance in joy_neighbors:
    if token_id != tokens["joy"].id:
        name = emotion_names[token_id]
        print(f"  - {name}: emotional distance = {distance:.3f}")
```

### Dynamic Network with Force Model

```python
from neurograph import Token, Connection, Grid, CoordinateSpace, ConnectionType

# Create grid
grid = Grid()

# Create tokens in physical space
positions = [(0.0, 0.0, 0.0), (5.0, 0.0, 0.0), (10.0, 0.0, 0.0)]

tokens = []
for i, (x, y, z) in enumerate(positions, start=1):
    token = Token(i)
    token.set_coordinates(CoordinateSpace.L1Physical(), x, y, z)
    token.set_active(True)
    grid.add(token)
    tokens.append(token)

# Create proximity-based connections
connections = []
for i, token in enumerate(tokens[:-1]):
    # Find next token
    next_token = tokens[i + 1]

    # Calculate distance
    distance = token.distance_to(next_token, CoordinateSpace.L1Physical())

    # Create connection with current distance as preferred
    conn = Connection(token.id, next_token.id, ConnectionType.Proximity())
    conn.preferred_distance = int(distance * 100)
    conn.pull_strength = 127  # 0.5
    conn.rigidity = 178  # 0.7
    conn.set_active(True)

    connections.append(conn)

    print(f"Connection {token.id} <-> {next_token.id}: preferred_distance = {distance:.2f}m")

# Simulate movement: check forces at different distances
for conn in connections:
    print(f"\nConnection {conn.token_a_id} <-> {conn.token_b_id}:")
    preferred = conn.preferred_distance / 100.0

    for test_d in [preferred * 0.5, preferred, preferred * 1.5]:
        force = conn.calculate_force(test_d)
        force_norm = force / 255.0

        direction = "pull" if force_norm > 0 else "push" if force_norm < 0 else "equilibrium"
        print(f"  At {test_d:.2f}m: force = {force_norm:+.3f} ({direction})")
```

---

## FastAPI Integration

### Using Grid Endpoints

**Start the API:**

```bash
cd src/api_mvp
python main.py
```

**Create Grid and Add Tokens:**

```bash
# Create grid
curl -X POST http://localhost:8000/api/v1/grid/create \
  -H "Content-Type: application/json" \
  -d '{
    "bucket_size": 10.0,
    "density_threshold": 0.5,
    "min_field_nodes": 3
  }'

# Response: {"grid_id": 1, "config": {...}, "status": "created"}

# Create token
curl -X POST http://localhost:8000/api/v1/tokens \
  -H "Content-Type: application/json" \
  -d '{
    "entity_type": 1,
    "domain": 0,
    "weight": 0.7,
    "l1_physical": {"x": 10.5, "y": 20.3, "z": 5.2}
  }'

# Response: {"id": <token_id>, ...}

# Add token to grid
curl -X POST http://localhost:8000/api/v1/grid/1/tokens/<token_id>

# Find neighbors
curl "http://localhost:8000/api/v1/grid/1/neighbors/<token_id>?space=0&radius=10.0&max_results=5"

# Range query
curl "http://localhost:8000/api/v1/grid/1/range?space=0&x=0&y=0&z=0&radius=15.0"

# Field influence
curl "http://localhost:8000/api/v1/grid/1/influence?space=0&x=10.5&y=20.3&z=5.2&radius=10.0"

# Density
curl "http://localhost:8000/api/v1/grid/1/density?space=0&x=0&y=0&z=0&radius=10.0"
```

---

## Advanced Patterns

### 1. Selective Level Activation

**Use Case:** Activate connections only on specific coordinate spaces.

```python
from neurograph import Connection, ConnectionType, active_levels

# Create connection
conn = Connection(1, 2, ConnectionType.Related())

# Activate only on Physical and Cognitive levels
conn.active_levels = active_levels.PHYSICAL | active_levels.COGNITIVE
conn.set_active(True)

# Check activation
print(f"Active on Physical: {conn.is_level_active(active_levels.PHYSICAL)}")
print(f"Active on Emotional: {conn.is_level_active(active_levels.EMOTIONAL)}")
```

### 2. Density-Based Clustering

**Use Case:** Find dense regions of tokens.

```python
# Create grid with many tokens
grid, tokens = create_grid_with_tokens(num_tokens=100, space=0, spread=50.0)

# Sample multiple points to find dense regions
test_points = [
    (0.0, 0.0, 0.0),
    (10.0, 10.0, 0.0),
    (20.0, 20.0, 0.0),
    (-10.0, -10.0, 0.0),
]

densities = []
for x, y, z in test_points:
    density = grid.calculate_density(0, x, y, z, radius=10.0)
    densities.append(((x, y, z), density))

# Find highest density region
densities.sort(key=lambda item: item[1], reverse=True)
center, max_density = densities[0]

print(f"Highest density region: {center} with density {max_density:.4f}")
```

### 3. Multi-Scale Queries

**Use Case:** Query at different spatial scales.

```python
# Query at multiple radii
center_token_id = 1

for radius in [5.0, 10.0, 20.0, 50.0]:
    neighbors = grid.find_neighbors(
        center_token_id,
        space=0,  # L1Physical
        radius=radius,
        max_results=100
    )

    print(f"Within {radius:5.1f}m: {len(neighbors):3d} neighbors")
```

---

## Performance Considerations

### 1. Grid Configuration

**Bucket Size:**
- Smaller buckets (1-5): Better for dense, clustered data
- Larger buckets (10-50): Better for sparse, uniform data
- Default (10.0): Good general-purpose choice

```python
# For dense clusters
config = GridConfig()
config.bucket_size = 2.0
grid = Grid(config)

# For sparse data
config = GridConfig()
config.bucket_size = 50.0
grid = Grid(config)
```

### 2. Query Optimization

**Limit max_results:**
```python
# Bad: Retrieves all neighbors
neighbors = grid.find_neighbors(token_id, 0, radius=100.0, max_results=10000)

# Good: Limits results early
neighbors = grid.find_neighbors(token_id, 0, radius=100.0, max_results=10)
```

**Use appropriate radius:**
```python
# Bad: Too large radius searches many buckets
neighbors = grid.find_neighbors(token_id, 0, radius=1000.0, max_results=5)

# Good: Smaller radius for faster search
neighbors = grid.find_neighbors(token_id, 0, radius=20.0, max_results=5)
```

### 3. Batch Operations

```python
# Create tokens in batch
tokens = []
for i in range(100):
    token = Token(i + 1)
    token.set_coordinates(CoordinateSpace.L1Physical(), i * 1.0, 0.0, 0.0)
    token.set_active(True)
    tokens.append(token)

# Add to grid in batch (faster than individual adds)
for token in tokens:
    grid.add(token)
```

---

## Best Practices

### 1. Always Set Active Flag

```python
# Good
token.set_active(True)
conn.set_active(True)

# May cause issues
# token and conn inactive by default
```

### 2. Use Appropriate Coordinate Spaces

```python
# Physical entities → L1Physical
token.set_coordinates(CoordinateSpace.L1Physical(), x, y, z)

# Emotional states → L4Emotional (VAD)
token.set_coordinates(CoordinateSpace.L4Emotional(), valence, arousal, dominance)

# Abstract concepts → L8Abstract
token.set_coordinates(CoordinateSpace.L8Abstract(), x, y, z)
```

### 3. Validate Distances Before Creating Connections

```python
# Calculate actual distance
distance = token1.distance_to(token2, CoordinateSpace.L1Physical())

# Set realistic preferred_distance
conn.preferred_distance = int(distance * 100)  # Current distance as preferred

# Or set desired distance
conn.preferred_distance = int(5.0 * 100)  # Want 5.0m separation
```

### 4. Clean Up Resources

```python
# Remove tokens from grid when done
removed = grid.remove(token_id)

# For FastAPI: Delete grids when done
# DELETE /api/v1/grid/{grid_id}
```

---

## Examples

See comprehensive examples:
- **Rust:** `src/core_rust/src/bin/integration-demo.rs`
- **Python:** `src/core_rust/examples/integration_demo.py`

Run examples:
```bash
# Rust
cargo run --release --bin integration-demo

# Python
python src/core_rust/examples/integration_demo.py
```

---

## Testing

### Run Integration Tests

**Rust:**
```bash
cd src/core_rust
cargo test integration --release
```

**Python:**
```bash
cd src/core_rust
pytest tests/test_integration_python.py -v
```

---

## Troubleshooting

### Grid Not Available in FastAPI

```
⚠️ Grid routes not available: No module named 'neurograph'
```

**Solution:**
```bash
cd src/core_rust
maturin develop --release --features python
```

### Token Not Found in Grid

```python
# Check if token exists before querying
if grid.get(token_id) is not None:
    neighbors = grid.find_neighbors(token_id, ...)
else:
    print(f"Token {token_id} not in grid")
```

### Force Calculations Return Unexpected Values

```python
# Remember: forces are encoded as u8 (0-255)
force_raw = conn.calculate_force(distance)
force_normalized = force_raw / 255.0  # -1.0 to 1.0

# Positive = pull, Negative = push
```

---

## Conclusion

Token + Connection + Grid provide a powerful foundation for:
- ✅ Spatial computing in 8 dimensions
- ✅ Semantic knowledge graphs
- ✅ Physical simulations with force models
- ✅ Emotional landscapes and VAD modeling
- ✅ Fast neighbor queries (O(k) complexity)
- ✅ Field influence calculations
- ✅ Density-based clustering

**Next Steps:**
- Explore advanced patterns in `examples/`
- Review API documentation in `docs/`
- Build your own spatial computing application!

---

**NeuroGraph OS v0.15.0** - Integration Complete
*Spatial intelligence across 8 dimensions*
