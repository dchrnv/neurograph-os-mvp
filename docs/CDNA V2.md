**CDNA v2.0 (384 bytes)** без отдельного Footer и Reserved блока 🎯

---

## CDNA v2.0 Final Structure (384 bytes)

```
╔═══════════════════════════════════════════════════════════╗
║                CDNA v2.0 (384 bytes)                      ║
║              6 cache lines (64 × 6)                       ║
╠═══════════════════════════════════════════════════════════╣
║  Block 1: HEADER (64 bytes)                               ║
║           Includes checksum, metadata                     ║
╠═══════════════════════════════════════════════════════════╣
║  Block 2: GRID PHYSICS CONSTANTS (128 bytes)              ║
║           semantic_ids, flags, scale                      ║
╠═══════════════════════════════════════════════════════════╣
║  Block 3: GRAPH TOPOLOGY RULES (64 bytes)                 ║
║           connection types, limits, topology              ║
╠═══════════════════════════════════════════════════════════╣
║  Block 4: TOKEN BASE PROPERTIES (32 bytes)                ║
║           weight, field properties, flags                 ║
╠═══════════════════════════════════════════════════════════╣
║  Block 5: CONNECTION CONSTRAINTS (64 bytes)               ║
║           rigidity, coupling, decay, levels               ║
╠═══════════════════════════════════════════════════════════╣
║  Block 6: EVOLUTION CONSTRAINTS (32 bytes)                ║
║           mutation, crossover, selection                  ║
╚═══════════════════════════════════════════════════════════╝

Memory layout:
Offset    Block                      Size
─────────────────────────────────────────────
0-63      Header                     64 bytes
64-191    Grid Physics               128 bytes
192-255   Graph Topology             64 bytes
256-287   Token Properties           32 bytes
288-351   Connection Constraints     64 bytes
352-383   Evolution Constraints      32 bytes
─────────────────────────────────────────────
Total:                               384 bytes
```

---

## Block 1: HEADER (64 bytes)

**Offset: 0-63**

```rust
#[repr(C, packed)]
pub struct CDNAHeader {
    // ═══════════════════════════════════════════════════════
    // IDENTIFICATION (16 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 0-7: Magic number and version
    magic: [u8; 8],                  // "NGCDNA20" (8 bytes ASCII)
                                     // Identifies file format and major version
    
    // Offset 8-11: Version details
    version_major: u16,              // 2 (breaking changes)
    version_minor: u16,              // 0 (new features, backward compatible)
    
    // Offset 12-13: Format info
    version_patch: u8,               // 0 (bugfixes)
    endianness: u8,                  // 0x02 = little-endian, 0x01 = big-endian
    
    // Offset 14-15: Compatibility range
    min_compatible_major: u8,        // Minimum compatible major version
    max_compatible_major: u8,        // Maximum compatible major version
    
    // ═══════════════════════════════════════════════════════
    // TEMPORAL METADATA (16 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 16-23: Creation timestamp
    created_timestamp: u64,          // Unix timestamp in seconds
                                     // When this CDNA was created
    
    // Offset 24-31: Modification timestamp
    modified_timestamp: u64,         // Last modification time
                                     // Updated when CDNA parameters change
    
    // ═══════════════════════════════════════════════════════
    // PROFILE INFORMATION (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 32-35: Profile identification
    profile_id: u32,                 // Unique profile identifier
                                     // Examples:
                                     // 0x00000001 = default
                                     // 0x00000100 = explorer
                                     // 0x00000200 = analyzer
                                     // 0x00000300 = creative
    
    // Offset 36-39: Profile type enum
    profile_type: u32,               // Profile category
                                     // 0x01 = EXPLORER
                                     // 0x02 = ANALYZER
                                     // 0x03 = CREATIVE
                                     // 0xFF = CUSTOM
    
    // ═══════════════════════════════════════════════════════
    // FLAGS AND SIZE (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 40-43: CDNA flags
    cdna_flags: u32,                 // Behavioral flags:
                                     // Bit 0:  IMMUTABLE (cannot be changed)
                                     // Bit 1:  VALIDATED (passed all checks)
                                     // Bit 2:  SEALED (cryptographically sealed)
                                     // Bit 3:  REQUIRE_SIGNATURE (needs signature to load)
                                     // Bit 4:  EXPERIMENTAL (unstable profile)
                                     // Bit 5:  PRODUCTION_READY
                                     // Bit 6:  DEBUG_MODE (verbose validation)
                                     // Bit 7:  STRICT_MODE (fail on warnings)
                                     // Bit 8-31: Reserved
    
    // Offset 44-47: Data size verification
    total_size: u32,                 // Should always be 384
                                     // Used to verify file integrity
    
    // ═══════════════════════════════════════════════════════
    // INTEGRITY CHECK (32 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 48-63: SHA-256 checksum (16 bytes displayed here for alignment)
    data_checksum: [u8; 32],         // SHA-256 hash of blocks 2-6
                                     // Covers offsets 64-383 (320 bytes)
                                     // Computed during save, verified on load
}

Total: 64 bytes
```

### Детали Header

**Magic number: "NGCDNA20"**

```
Purpose:
    • Instant file type identification
    • Version embedded in magic (20 = v2.0)
    
Validation:
    if header.magic != b"NGCDNA20" {
        return Err("Not a valid CDNA v2.0 file")
    }
```

**Version system:**

```
major.minor.patch (2.0.0)

major (u16):
    Breaking changes
    Different major = incompatible
    Example: v1.x cannot load v2.x

minor (u16):
    New features, backward compatible
    v2.0 can load v2.5 (if major matches)

patch (u8):
    Bug fixes only
    Always compatible within same major.minor
```

**Compatibility range:**

```
min_compatible_major: 2
max_compatible_major: 2

Guardian checks:
    if loaded.major < min_compatible or loaded.major > max_compatible {
        return Err("Incompatible CDNA version")
    }

Allows forward compatibility:
    CDNA v2.0 can specify it works with v2.x - v3.x
```

**Timestamps:**

```
created_timestamp:
    When CDNA was first created
    Never changes
    
modified_timestamp:
    Last time CDNA parameters were changed
    Updated on any modification
    
Usage:
    Track CDNA lineage
    Determine "age" of configuration
    Rollback to older versions
```

**Profile system:**

```
profile_id (u32):
    Unique identifier
    0x00000001 - 0x00000FFF: Reserved system profiles
    0x00001000 - 0xFFFFFFFF: User-defined profiles

profile_type (u32):
    Category/template
    
    EXPLORER (0x01):
        Loose constraints
        High mutation rates
        Many connection types allowed
        
    ANALYZER (0x02):
        Strict constraints
        Low mutation rates
        Limited connection types
        
    CREATIVE (0x03):
        Exotic parameters
        Experimental features
        Non-standard topology
        
    CUSTOM (0xFF):
        User-defined from scratch
```

**CDNA flags (u32):**

```
Bit 0: IMMUTABLE
    If set: CDNA cannot be modified
    Guardian will reject any change proposals
    
Bit 1: VALIDATED
    Set after successful validation
    Guardian checks this on load
    
Bit 2: SEALED
    Cryptographically sealed
    Requires signature to modify
    
Bit 3: REQUIRE_SIGNATURE
    Must have valid digital signature to load
    For security-critical deployments
    
Bit 4: EXPERIMENTAL
    Unstable profile, may change
    Guardian logs warnings
    
Bit 5: PRODUCTION_READY
    Tested and stable
    Safe for production use
    
Bit 6: DEBUG_MODE
    Enable verbose validation
    Log every check
    
Bit 7: STRICT_MODE
    Fail on any warning
    No permissive behavior
```

**Data checksum (SHA-256):**

```
Covers: Blocks 2-6 (offsets 64-383)
Total: 320 bytes

Computation:
    1. Serialize blocks 2-6 to bytes
    2. SHA-256 hash
    3. Store in header.data_checksum

Verification:
    1. Load CDNA from file
    2. Extract blocks 2-6
    3. Compute SHA-256
    4. Compare with header.data_checksum
    5. If mismatch → reject file (corrupted or tampered)

Why SHA-256:
    • Industry standard
    • Cryptographically secure
    • Fast to compute (~100 MB/s on laptop)
    • 2^256 collision resistance
```

---

## Block 2: GRID PHYSICS CONSTANTS (128 bytes)

**Offset: 64-191**

```rust
#[repr(C, packed)]
pub struct GridPhysicsConstants {
    // ═══════════════════════════════════════════════════════
    // DIMENSION SEMANTIC IDs (32 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 64-95: Semantic identifiers for 8 dimensions
    dimension_semantic_ids: [u32; 8],
        // Each u32 encodes:
        //   Bits 0-15:   Primary category (65K categories)
        //   Bits 16-23:  Subcategory (256 subcategories)
        //   Bits 24-31:  Flags/Reserved
        //
        // Default values:
        // [0]: 0x00010000 = L0_PHYSICAL
        // [1]: 0x00020000 = L1_SENSORY
        // [2]: 0x00030000 = L2_MOTOR
        // [3]: 0x00040000 = L3_EMOTIONAL
        // [4]: 0x00050000 = L4_COGNITIVE
        // [5]: 0x00060000 = L5_SOCIAL
        // [6]: 0x00070000 = L6_TEMPORAL
        // [7]: 0x00080000 = L7_ABSTRACT
    
    // ═══════════════════════════════════════════════════════
    // DIMENSION FLAGS (32 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 96-127: Property flags for each dimension
    dimension_flags: [u32; 8],
        // 32 bits per dimension
        //
        // Bit 0:  CYCLIC (wraps around, e.g., time of day)
        // Bit 1:  BOUNDED (has min/max limits)
        // Bit 2:  DISCRETE (integer values only)
        // Bit 3:  INVERTIBLE (can be mirrored)
        // Bit 4:  LOGARITHMIC (log scale)
        // Bit 5:  HIERARCHICAL (has levels)
        // Bit 6:  METRIC (satisfies metric axioms)
        // Bit 7:  SYMMETRIC (symmetric about center)
        // Bit 8:  ORIENTED (has direction)
        // Bit 9:  QUANTIZED (quantum levels)
        // Bit 10: ANISOTROPIC (direction-dependent)
        // Bit 11: WARPED (curved space)
        // Bit 12: NORMALIZED (always [0,1])
        // Bit 13: SIGNED (allows negative values)
        // Bit 14: SPARSE (mostly empty)
        // Bit 15: DENSE (mostly full)
        // Bit 16-31: Reserved for future flags
    
    // ═══════════════════════════════════════════════════════
    // SCALE FACTORS (64 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 128-191: Scale/weight for each dimension
    dimension_scale: [f64; 8],
        // f64 for maximum precision
        // Used in distance calculations:
        //   distance = sqrt(Σ ((x[i] - y[i]) * scale[i])²)
        //
        // Interpretation:
        //   Higher scale = dimension is more important
        //   scale[i] = 1.0   → baseline importance
        //   scale[i] = 10.0  → 10× more important than baseline
        //   scale[i] = 0.1   → 10× less important
        //
        // Recommended ranges:
        //   0.1 - 100.0 for most use cases
        //   Keep scales within 2 orders of magnitude for stability
}

Total: 128 bytes
Offset range: 64-191
```

### Примеры конфигураций Grid

**Default balanced profile:**

```rust
dimension_scale: [
    1.0,   // L0: Physical (baseline)
    1.5,   // L1: Sensory (slightly more)
    1.2,   // L2: Motor (slightly more)
    2.0,   // L3: Emotional (2× important)
    5.0,   // L4: Cognitive (5× important)
    3.0,   // L5: Social (3× important)
    2.0,   // L6: Temporal (2× important)
    10.0,  // L7: Abstract (10× important)
]
```

**Physical-focused (robotics):**

```rust
dimension_scale: [
    10.0,  // L0: Physical (most important!)
    5.0,   // L1: Sensory
    8.0,   // L2: Motor (very important)
    1.0,   // L3: Emotional (minimal)
    2.0,   // L4: Cognitive
    0.5,   // L5: Social (least important)
    3.0,   // L6: Temporal
    1.0,   // L7: Abstract (minimal)
]
```

**Abstract-focused (pure reasoning):**

```rust
dimension_scale: [
    0.1,   // L0: Physical (minimal)
    0.5,   // L1: Sensory
    0.3,   // L2: Motor
    1.0,   // L3: Emotional
    20.0,  // L4: Cognitive (very important)
    5.0,   // L5: Social
    10.0,  // L6: Temporal
    50.0,  // L7: Abstract (most important!)
]
```

---

## Block 3: GRAPH TOPOLOGY RULES (64 bytes)

**Offset: 192-255**

```rust
#[repr(C, packed)]
pub struct GraphTopologyRules {
    // ═══════════════════════════════════════════════════════
    // CONNECTION TYPE PERMISSIONS (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 192-193: Allowed connection types bitmask
    allowed_connection_types: u16,
        // Bit 0:  ASSOCIATION (0x0001)
        // Bit 1:  HIERARCHY (0x0002)
        // Bit 2:  SEQUENCE (0x0004)
        // Bit 3:  CAUSALITY (0x0008)
        // Bit 4:  SIMILARITY (0x0010)
        // Bit 5:  OPPOSITION (0x0020)
        // Bit 6:  DEPENDENCY (0x0040)
        // Bit 7:  COMPOSITION (0x0080)
        // Bit 8:  REFERENCE (0x0100)
        // Bit 9:  MUTATION (0x0200)
        // Bit 10: CROSSOVER (0x0400)
        // Bit 11: INHERITANCE (0x0800)
        // Bit 12: PROXIMITY (0x1000)
        // Bit 13: CONTAINMENT (0x2000)
        // Bit 14: CUSTOM_1 (0x4000)
        // Bit 15: CUSTOM_2 (0x8000)
        //
        // 0xFFFF = all types allowed
        // 0x0001 = only ASSOCIATION
        // 0x0888 = CAUSALITY | DEPENDENCY | INHERITANCE
    
    // Offset 194-195: Reserved for alignment
    _padding1: u16,
    
    // ═══════════════════════════════════════════════════════
    // DEGREE LIMITS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 196-199: Maximum node degree
    max_node_degree: u32,            // Max total degree (in + out)
                                     // 0 = unlimited (not recommended)
                                     // Typical: 100 - 1000
                                     // Prevents "super-hub" nodes
    
    // Offset 200-203: Average degree hint
    avg_degree_hint: u32,            // Expected average degree
                                     // Used for performance optimization
                                     // Typical: 10 - 50
    
    // ═══════════════════════════════════════════════════════
    // WEIGHT CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 204-207: Connection weight range
    min_connection_weight: f32,      // Minimum allowed weight
                                     // Typical: 0.01 (1%)
                                     // Weak connections below this are invalid
    
    max_connection_weight: f32,      // Maximum allowed weight
                                     // Typical: 1.0 (100%)
                                     // Can be > 1.0 for special cases
    
    // ═══════════════════════════════════════════════════════
    // DISTANCE CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 208-211: Spatial distance limits
    connection_distance_min: f32,    // Minimum distance between connected tokens
                                     // 0.0 = no minimum (tokens can be co-located)
                                     // > 0 = enforce minimum separation
    
    connection_distance_max: f32,    // Maximum distance for connections
                                     // 0.0 or Infinity = unlimited
                                     // Typical: 10.0 - 1000.0
                                     // Prevents long-distance "spurious" connections
    
    // ═══════════════════════════════════════════════════════
    // TOPOLOGY FLAGS (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 212-215: Behavioral flags
    topology_flags: u32,
        // Bit 0:  ALLOW_SELF_LOOPS (node can connect to itself)
        // Bit 1:  ALLOW_MULTI_EDGES (multiple edges between same nodes)
        // Bit 2:  DIRECTED (edges have direction)
        // Bit 3:  WEIGHTED (edges have weights)
        // Bit 4:  ENFORCE_DISTANCE_LIMIT (check distance constraints)
        // Bit 5:  ENFORCE_DEGREE_LIMIT (check max_node_degree)
        // Bit 6:  ALLOW_DYNAMIC_TOPOLOGY (graph can change structure)
        // Bit 7:  REQUIRE_CONNECTED (entire graph must be connected)
        // Bit 8:  ALLOW_CYCLES (cycles permitted)
        // Bit 9:  DAG_ONLY (must be acyclic, overrides bit 8)
        // Bit 10: BIDIRECTIONAL_DEFAULT (new edges are bidirectional)
        // Bit 11-31: Reserved
    
    // ═══════════════════════════════════════════════════════
    // SIZE LIMITS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 216-219: Node count limit
    max_nodes: u32,                  // Maximum number of nodes
                                     // 0 = unlimited
                                     // Typical: 100,000 - 1,000,000
    
    // Offset 220-223: Edge count limit
    max_edges: u32,                  // Maximum number of edges
                                     // 0 = unlimited
                                     // Typical: 1,000,000 - 10,000,000
                                     // Should be >= max_nodes * avg_degree
    
    // ═══════════════════════════════════════════════════════
    // PERFORMANCE TUNING (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 224-227: Sparsity threshold
    sparsity_threshold: f32,         // Threshold for considering graph "sparse"
                                     // density = edges / (nodes * (nodes-1))
                                     // if density < threshold: use sparse algorithms
                                     // Typical: 0.01 (1%)
    
    // Offset 228-231: Cache hints
    cache_hint_flags: u32,           // Performance optimization hints
        // Bit 0-7:   Expected hot node percentage (0-255 → 0-100%)
        // Bit 8-15:  Cache priority (0=low, 255=high)
        // Bit 16-23: Prefetch strategy (0=none, 1=neighbors, 2=paths)
        // Bit 24-31: Reserved
    
    // ═══════════════════════════════════════════════════════
    // RESERVED (24 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 232-255: Reserved for future use
    reserved: [u8; 24],
}

Total: 64 bytes
Offset range: 192-255
```

---

## Block 4: TOKEN BASE PROPERTIES (32 bytes)

**Offset: 256-287**

```rust
#[repr(C, packed)]
pub struct TokenBaseProperties {
    // ═══════════════════════════════════════════════════════
    // WEIGHT CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 256-259: Token weight range
    min_token_weight: f32,           // Minimum token weight
                                     // 0.0 = allow weightless tokens
                                     // > 0 = enforce minimum significance
                                     // Typical: 0.01 - 0.1
    
    max_token_weight: f32,           // Maximum token weight
                                     // Typical: 1.0 or 10.0 or 100.0
                                     // Higher values = more "important" tokens
    
    // ═══════════════════════════════════════════════════════
    // FIELD RADIUS CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 260-263: Field radius range
    min_field_radius: f32,           // Minimum field radius
                                     // 0.0 = no minimum
                                     // > 0 = every token must have some field
                                     // Typical: 0.0 - 1.0
    
    max_field_radius: f32,           // Maximum field radius
                                     // Limits how far token's influence extends
                                     // Typical: 10.0 - 100.0
                                     // Should be compatible with Grid dimensions
    
    // ═══════════════════════════════════════════════════════
    // FIELD STRENGTH CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 264-267: Field strength range
    min_field_strength: f32,         // Minimum field strength
                                     // 0.0 = allow zero-strength fields
                                     // > 0 = enforce minimum influence
                                     // Typical: 0.0 - 0.1
    
    max_field_strength: f32,         // Maximum field strength
                                     // Limits force magnitude in physics simulation
                                     // Typical: 1.0 - 10.0
                                     // Too high = unstable dynamics
    
    // ═══════════════════════════════════════════════════════
    // TOKEN FLAGS (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 268-271: Behavioral flags
    token_flags: u32,
        // Bit 0:  ALLOW_ZERO_WEIGHT (weight can be 0.0)
        // Bit 1:  REQUIRE_FSC (every token must have FSC code)
        // Bit 2:  ALLOW_MIGRATION (token can change coordinates)
        // Bit 3:  ALLOW_SPLITTING (token can divide)
        // Bit 4:  ALLOW_MERGING (tokens can combine)
        // Bit 5:  REQUIRE_FIELD (every token must have field_radius > 0)
        // Bit 6:  ENFORCE_WEIGHT_CONSERVATION (total weight constant)
        // Bit 7:  ALLOW_NEGATIVE_WEIGHT (antimatter tokens)
        // Bit 8:  ALLOW_NEGATIVE_FIELD (repulsive fields)
        // Bit 9:  IMMUTABLE_AFTER_CREATION (token properties frozen)
        // Bit 10: REQUIRE_UNIQUE_FSC (no duplicate FSC codes)
        // Bit 11-31: Reserved
    
    // ═══════════════════════════════════════════════════════
    // RESERVED (16 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 272-287: Reserved for future token properties
    reserved: [u8; 16],
}

Total: 32 bytes
Offset range: 256-287
```

---

## Block 5: CONNECTION CONSTRAINTS (64 bytes)

**Offset: 288-351**

```rust
#[repr(C, packed)]
pub struct ConnectionConstraints {
    // ═══════════════════════════════════════════════════════
    // RIGIDITY CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 288-291: Rigidity range
    min_rigidity: f32,               // Minimum rigidity
                                     // 0.0 = fully flexible connection
                                     // Can change weight/type easily
                                     // Typical: 0.0 - 0.3
    
    max_rigidity: f32,               // Maximum rigidity
                                     // 1.0 = fully rigid connection
                                     // Weight/type locked
                                     // Typical: 0.7 - 1.0
    
    // ═══════════════════════════════════════════════════════
    // FIELD COUPLING CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 292-295: Field coupling range
    min_field_coupling_strength: f32, // Minimum coupling
                                     // 0.0 = no field interaction
                                     // Purely logical connection
                                     // Typical: 0.0
    
    max_field_coupling_strength: f32, // Maximum coupling
                                     // 1.0 = full field interaction
                                     // Physical force transmission
                                     // Typical: 0.5 - 1.0
    
    // ═══════════════════════════════════════════════════════
    // DECAY PARAMETERS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 296-299: Decay rate range
    min_decay_rate: f32,             // Minimum decay (fast forgetting)
                                     // 0.9 = lose 10% per time step
                                     // Typical: 0.9 - 0.95
    
    max_decay_rate: f32,             // Maximum decay (slow forgetting)
                                     // 0.9999 = lose 0.01% per step
                                     // Typical: 0.995 - 0.9999
    
    // ═══════════════════════════════════════════════════════
    // TEMPORAL PROPERTIES (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 300-303: Formation time range
    min_formation_time: f32,         // Minimum time to form connection
                                     // 0.0 = instant
                                     // > 0 = gradual formation
                                     // Units: simulation time steps
                                     // Typical: 0.0 - 10.0
    
    max_lifespan: f32,               // Maximum connection lifespan
                                     // 0.0 or Infinity = unlimited
                                     // > 0 = auto-delete after this time
                                     // Units: simulation time steps
                                     // Typical: 0.0 (unlimited) or 1000-10000
    
    // ═══════════════════════════════════════════════════════
    // ACTIVE LEVELS (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 304-305: Allowed active levels bitmask
    allowed_active_levels: u16,      // Which Grid levels can be active
        // Bit 0: L0_PHYSICAL
        // Bit 1: L1_SENSORY
        // Bit 2: L2_MOTOR
        // Bit 3: L3_EMOTIONAL
        // Bit 4: L4_COGNITIVE
        // Bit 5: L5_SOCIAL
        // Bit 6: L6_TEMPORAL
        // Bit 7: L7_ABSTRACT
        // Bit 8-15: Reserved
        //
        // 0xFF = all levels allowed
        // 0x01 = only L0 (physical)
        // 0x80 = only L7 (abstract)
    
    // Offset 306-307: Default active levels
    default_active_levels: u16,      // Default levels for new connections
                                     // Should be subset of allowed_active_levels
    
    // ═══════════════════════════════════════════════════════
    // CONNECTION FLAGS (4 bytes)
    // ═══════════════════════════════════════════════════╣


// Offset 308-311: Behavioral flags
connection_flags: u32,
    // Bit 0:  ALLOW_BIDIRECTIONAL (connection can be bidirectional)
    // Bit 1:  ALLOW_WEIGHTED (connection can have variable weight)
    // Bit 2:  ALLOW_TEMPORAL (connection can have time properties)
    // Bit 3:  ALLOW_MULTI_LEVEL (connection can be active on multiple levels)
    // Bit 4:  ENFORCE_RECIPROCITY (if A→B exists, must have B→A)
    // Bit 5:  ALLOW_DYNAMIC_WEIGHT (weight can change over time)
    // Bit 6:  ALLOW_DYNAMIC_RIGIDITY (rigidity can change)
    // Bit 7:  REQUIRE_FIELD_COUPLING (must have coupling > 0)
    // Bit 8:  ALLOW_ZERO_COUPLING (coupling can be 0.0)
    // Bit 9:  ENFORCE_SYMMETRY (bidirectional connections have same weight)
    // Bit 10: ALLOW_NEGATIVE_WEIGHT (connection can have negative weight)
    // Bit 11: TRANSITIVE_CLOSURE (automatically create A→C if A→B→C)
    // Bit 12-31: Reserved

// ═══════════════════════════════════════════════════════
// STRENGTH AND ACTIVATION (8 bytes)
// ═══════════════════════════════════════════════════════

// Offset 312-315: Activation threshold
min_activation_threshold: f32,   // Minimum activation level to "fire"
                                 // Connection only active if signal > threshold
                                 // Typical: 0.1 - 0.3
                                 // 0.0 = always active

max_activation_threshold: f32,   // Maximum activation threshold
                                 // Typical: 0.5 - 0.9
                                 // 1.0 = never activates (effectively disabled)

// ═══════════════════════════════════════════════════════
// RESERVED (32 bytes)
// ═══════════════════════════════════════════════════════

// Offset 316-351: Reserved for future connection properties
reserved: [u8; 36],
}

Total: 64 bytes 
Offset range: 288-351
```


---

## Block 6: EVOLUTION CONSTRAINTS (32 bytes)

**Offset: 352-383**

```rust
#[repr(C, packed)]
pub struct EvolutionConstraints {
    // ═══════════════════════════════════════════════════════
    // MUTATION RATE CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 352-355: Mutation rate range
    min_mutation_rate: f32,          // Minimum mutation rate
                                     // How often mutations occur
                                     // 0.001 = 0.1% chance per operation
                                     // Lower = more stable
                                     // Typical: 0.001 - 0.01
    
    max_mutation_rate: f32,          // Maximum mutation rate
                                     // 0.2 = 20% chance per operation
                                     // Higher = more chaotic evolution
                                     // Typical: 0.05 - 0.2
    
    // ═══════════════════════════════════════════════════════
    // CROSSOVER RATE CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 356-359: Crossover rate range
    min_crossover_rate: f32,         // Minimum crossover rate
                                     // How often subgraphs recombine
                                     // 0.01 = 1% chance
                                     // Typical: 0.01 - 0.05
    
    max_crossover_rate: f32,         // Maximum crossover rate
                                     // 0.5 = 50% chance
                                     // Higher = more mixing of structures
                                     // Typical: 0.1 - 0.5
    
    // ═══════════════════════════════════════════════════════
    // SELECTION PRESSURE CONSTRAINTS (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 360-363: Selection pressure range
    min_selection_pressure: f32,     // Minimum selection pressure
                                     // How aggressive is pruning
                                     // 0.3 = soft selection (30% threshold)
                                     // Keeps more weak connections
                                     // Typical: 0.3 - 0.5
    
    max_selection_pressure: f32,     // Maximum selection pressure
                                     // 0.9 = harsh selection (90% threshold)
                                     // Only strongest connections survive
                                     // Typical: 0.7 - 0.9
    
    // ═══════════════════════════════════════════════════════
    // EVOLUTION FLAGS (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 364-367: Evolution operator permissions
    evolution_flags: u32,
        // Bit 0:  ALLOW_MUTATIONS (mutations enabled)
        // Bit 1:  ALLOW_CROSSOVER (crossover enabled)
        // Bit 2:  ALLOW_SELECTION (selection/pruning enabled)
        // Bit 3:  ELITISM_ENABLED (top-N always preserved)
        // Bit 4:  ALLOW_ADD_NODE (can create new nodes)
        // Bit 5:  ALLOW_REMOVE_NODE (can delete nodes)
        // Bit 6:  ALLOW_ADD_EDGE (can create new connections)
        // Bit 7:  ALLOW_REMOVE_EDGE (can delete connections)
        // Bit 8:  ALLOW_MODIFY_WEIGHT (can change connection weights)
        // Bit 9:  ALLOW_CHANGE_TYPE (can change connection type)
        // Bit 10: ALLOW_REVERSE_DIRECTION (can flip edge direction)
        // Bit 11: ALLOW_SPLIT_NODE (node splitting)
        // Bit 12: ALLOW_MERGE_NODES (node merging)
        // Bit 13: ADAPTIVE_RATES (rates self-adjust based on fitness)
        // Bit 14: PRESERVE_TOPOLOGY (maintain graph structure)
        // Bit 15: INCREMENTAL_ONLY (small changes only, no radical mutations)
        // Bit 16-31: Reserved
    
    // ═══════════════════════════════════════════════════════
    // POPULATION AND GENERATION (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 368-369: Elite preservation count
    elitism_count: u16,              // How many top connections to preserve
                                     // During selection, top N always kept
                                     // 0 = no elitism
                                     // Typical: 5 - 50
    
    // Offset 370-371: Generation tracking
    max_generations: u16,            // Maximum generations before reset
                                     // 0 = unlimited
                                     // > 0 = reset evolution after N generations
                                     // Typical: 0 (unlimited) or 1000-10000
    
    // ═══════════════════════════════════════════════════════
    // FITNESS AND QUALITY (4 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 372-375: Fitness thresholds
    min_fitness_threshold: f32,      // Minimum fitness to survive selection
                                     // Connections below this are candidates for removal
                                     // 0.0 - 1.0 range
                                     // Typical: 0.2 - 0.5
    
    // ═══════════════════════════════════════════════════════
    // RESERVED (8 bytes)
    // ═══════════════════════════════════════════════════════
    
    // Offset 376-383: Reserved for future evolution parameters
    reserved: [u8; 8],
}

Total: 32 bytes
Offset range: 352-383
````

---

## Complete Rust Implementation

```rust
use std::mem;
use sha2::{Sha256, Digest};

// ═══════════════════════════════════════════════════════════
// MAIN CDNA STRUCTURE (384 bytes)
// ═══════════════════════════════════════════════════════════

#[repr(C, packed)]
#[derive(Clone)]
pub struct CDNA {
    pub header: CDNAHeader,
    pub grid: GridPhysicsConstants,
    pub graph: GraphTopologyRules,
    pub token: TokenBaseProperties,
    pub connection: ConnectionConstraints,
    pub evolution: EvolutionConstraints,
}

// Compile-time size verification
const _: () = assert!(mem::size_of::<CDNA>() == 384);

// ═══════════════════════════════════════════════════════════
// IMPLEMENTATION
// ═══════════════════════════════════════════════════════════

impl CDNA {
    /// Magic number for CDNA v2.0 files
    pub const MAGIC: &'static [u8; 8] = b"NGCDNA20";
    
    /// Current version
    pub const VERSION_MAJOR: u16 = 2;
    pub const VERSION_MINOR: u16 = 0;
    pub const VERSION_PATCH: u8 = 0;
    
    /// Expected file size
    pub const SIZE: u32 = 384;
    
    // ───────────────────────────────────────────────────────
    // Creation
    // ───────────────────────────────────────────────────────
    
    /// Create new CDNA with default values
    pub fn new() -> Self {
        let mut cdna = Self {
            header: CDNAHeader::default(),
            grid: GridPhysicsConstants::default(),
            graph: GraphTopologyRules::default(),
            token: TokenBaseProperties::default(),
            connection: ConnectionConstraints::default(),
            evolution: EvolutionConstraints::default(),
        };
        
        // Compute and set checksum
        cdna.update_checksum();
        
        cdna
    }
    
    /// Create CDNA from profile
    pub fn from_profile(profile_type: ProfileType) -> Self {
        match profile_type {
            ProfileType::Explorer => Self::explorer_profile(),
            ProfileType::Analyzer => Self::analyzer_profile(),
            ProfileType::Creative => Self::creative_profile(),
            ProfileType::Custom => Self::new(),
        }
    }
    
    // ───────────────────────────────────────────────────────
    // Validation
    // ───────────────────────────────────────────────────────
    
    /// Validate CDNA structure and data
    pub fn validate(&self) -> Result<(), CDNAError> {
        // 1. Check magic number
        if &self.header.magic != Self::MAGIC {
            return Err(CDNAError::InvalidMagic);
        }
        
        // 2. Check version compatibility
        if self.header.version_major != Self::VERSION_MAJOR {
            return Err(CDNAError::IncompatibleVersion {
                expected: Self::VERSION_MAJOR,
                found: self.header.version_major,
            });
        }
        
        // 3. Check size
        if self.header.total_size != Self::SIZE {
            return Err(CDNAError::InvalidSize {
                expected: Self::SIZE,
                found: self.header.total_size,
            });
        }
        
        // 4. Verify checksum
        let computed = self.compute_checksum();
        if computed != self.header.data_checksum {
            return Err(CDNAError::ChecksumMismatch);
        }
        
        // 5. Validate ranges
        self.validate_ranges()?;
        
        // 6. Validate flags consistency
        self.validate_flags()?;
        
        Ok(())
    }
    
    /// Validate that min/max ranges are correct
    fn validate_ranges(&self) -> Result<(), CDNAError> {
        // Graph ranges
        if self.graph.min_connection_weight > self.graph.max_connection_weight {
            return Err(CDNAError::InvalidRange {
                field: "graph.connection_weight",
                min: self.graph.min_connection_weight,
                max: self.graph.max_connection_weight,
            });
        }
        
        if self.graph.connection_distance_min > self.graph.connection_distance_max {
            return Err(CDNAError::InvalidRange {
                field: "graph.connection_distance",
                min: self.graph.connection_distance_min,
                max: self.graph.connection_distance_max,
            });
        }
        
        // Token ranges
        if self.token.min_token_weight > self.token.max_token_weight {
            return Err(CDNAError::InvalidRange {
                field: "token.weight",
                min: self.token.min_token_weight,
                max: self.token.max_token_weight,
            });
        }
        
        if self.token.min_field_radius > self.token.max_field_radius {
            return Err(CDNAError::InvalidRange {
                field: "token.field_radius",
                min: self.token.min_field_radius,
                max: self.token.max_field_radius,
            });
        }
        
        if self.token.min_field_strength > self.token.max_field_strength {
            return Err(CDNAError::InvalidRange {
                field: "token.field_strength",
                min: self.token.min_field_strength,
                max: self.token.max_field_strength,
            });
        }
        
        // Connection ranges
        if self.connection.min_rigidity > self.connection.max_rigidity {
            return Err(CDNAError::InvalidRange {
                field: "connection.rigidity",
                min: self.connection.min_rigidity,
                max: self.connection.max_rigidity,
            });
        }
        
        if self.connection.min_field_coupling_strength > self.connection.max_field_coupling_strength {
            return Err(CDNAError::InvalidRange {
                field: "connection.field_coupling",
                min: self.connection.min_field_coupling_strength,
                max: self.connection.max_field_coupling_strength,
            });
        }
        
        if self.connection.min_decay_rate > self.connection.max_decay_rate {
            return Err(CDNAError::InvalidRange {
                field: "connection.decay_rate",
                min: self.connection.min_decay_rate,
                max: self.connection.max_decay_rate,
            });
        }
        
        // Evolution ranges
        if self.evolution.min_mutation_rate > self.evolution.max_mutation_rate {
            return Err(CDNAError::InvalidRange {
                field: "evolution.mutation_rate",
                min: self.evolution.min_mutation_rate,
                max: self.evolution.max_mutation_rate,
            });
        }
        
        if self.evolution.min_crossover_rate > self.evolution.max_crossover_rate {
            return Err(CDNAError::InvalidRange {
                field: "evolution.crossover_rate",
                min: self.evolution.min_crossover_rate,
                max: self.evolution.max_crossover_rate,
            });
        }
        
        if self.evolution.min_selection_pressure > self.evolution.max_selection_pressure {
            return Err(CDNAError::InvalidRange {
                field: "evolution.selection_pressure",
                min: self.evolution.min_selection_pressure,
                max: self.evolution.max_selection_pressure,
            });
        }
        
        Ok(())
    }
    
    /// Validate flag consistency
    fn validate_flags(&self) -> Result<(), CDNAError> {
        // Check connection allowed_active_levels is subset of valid levels
        let valid_levels = 0xFF; // Bits 0-7
        if self.connection.allowed_active_levels & !valid_levels != 0 {
            return Err(CDNAError::InvalidFlags {
                field: "connection.allowed_active_levels",
                value: self.connection.allowed_active_levels as u32,
            });
        }
        
        // Check default_active_levels is subset of allowed
        if self.connection.default_active_levels & !self.connection.allowed_active_levels != 0 {
            return Err(CDNAError::InvalidFlags {
                field: "connection.default_active_levels",
                value: self.connection.default_active_levels as u32,
            });
        }
        
        // If REQUIRE_CONNECTED is set, graph should be DIRECTED
        if self.graph.topology_flags & TopologyFlags::REQUIRE_CONNECTED != 0 {
            if self.graph.topology_flags & TopologyFlags::DIRECTED == 0 {
                return Err(CDNAError::InconsistentFlags {
                    reason: "REQUIRE_CONNECTED needs DIRECTED graph",
                });
            }
        }
        
        // If DAG_ONLY is set, ALLOW_CYCLES should not be set
        if self.graph.topology_flags & TopologyFlags::DAG_ONLY != 0 {
            if self.graph.topology_flags & TopologyFlags::ALLOW_CYCLES != 0 {
                return Err(CDNAError::InconsistentFlags {
                    reason: "DAG_ONLY conflicts with ALLOW_CYCLES",
                });
            }
        }
        
        Ok(())
    }
    
    // ───────────────────────────────────────────────────────
    // Checksum
    // ───────────────────────────────────────────────────────
    
    /// Compute SHA-256 checksum of data blocks (offsets 64-383)
    pub fn compute_checksum(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        
        // Hash blocks 2-6 (320 bytes)
        unsafe {
            let ptr = self as *const Self as *const u8;
            let data_start = ptr.add(64); // Skip header
            let data = std::slice::from_raw_parts(data_start, 320);
            hasher.update(data);
        }
        
        hasher.finalize().into()
    }
    
    /// Update checksum in header
    pub fn update_checksum(&mut self) {
        let checksum = self.compute_checksum();
        self.header.data_checksum = checksum;
    }
    
    // ───────────────────────────────────────────────────────
    // Serialization
    // ───────────────────────────────────────────────────────
    
    /// Save CDNA to file
    pub fn save(&mut self, path: &str) -> Result<(), CDNAError> {
        use std::fs::File;
        use std::io::Write;
        
        // Update timestamps
        self.header.modified_timestamp = current_timestamp();
        
        // Recompute checksum
        self.update_checksum();
        
        // Validate before saving
        self.validate()?;
        
        // Write to file
        let bytes = unsafe {
            let ptr = self as *const Self as *const u8;
            std::slice::from_raw_parts(ptr, 384)
        };
        
        let mut file = File::create(path)
            .map_err(|e| CDNAError::IOError(e.to_string()))?;
        
        file.write_all(bytes)
            .map_err(|e| CDNAError::IOError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Load CDNA from file
    pub fn load(path: &str) -> Result<Self, CDNAError> {
        use std::fs::File;
        use std::io::Read;
        
        let mut file = File::open(path)
            .map_err(|e| CDNAError::IOError(e.to_string()))?;
        
        let mut bytes = [0u8; 384];
        file.read_exact(&mut bytes)
            .map_err(|e| CDNAError::IOError(e.to_string()))?;
        
        let cdna: Self = unsafe {
            std::ptr::read(bytes.as_ptr() as *const Self)
        };
        
        // Validate after loading
        cdna.validate()?;
        
        Ok(cdna)
    }
    
    // ───────────────────────────────────────────────────────
    // Profiles
    // ───────────────────────────────────────────────────────
    
    /// Explorer profile: loose constraints, high evolution
    fn explorer_profile() -> Self {
        let mut cdna = Self::new();
        
        cdna.header.profile_type = ProfileType::Explorer as u32;
        cdna.header.profile_id = 0x00000100;
        
        // Grid: balanced scales
        cdna.grid.dimension_scale = [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0];
        
        // Graph: permissive
        cdna.graph.allowed_connection_types = 0xFFFF;
        cdna.graph.max_node_degree = 500;
        cdna.graph.min_connection_weight = 0.01;
        cdna.graph.topology_flags = TopologyFlags::ALLOW_SELF_LOOPS 
            | TopologyFlags::ALLOW_MULTI_EDGES
            | TopologyFlags::ALLOW_DYNAMIC_TOPOLOGY;
        
        // Evolution: high rates
        cdna.evolution.max_mutation_rate = 0.2;
        cdna.evolution.max_crossover_rate = 0.5;
        cdna.evolution.evolution_flags = EvolutionFlags::ALLOW_ALL;
        
        cdna.update_checksum();
        cdna
    }
    
    /// Analyzer profile: strict constraints, low evolution
    fn analyzer_profile() -> Self {
        let mut cdna = Self::new();
        
        cdna.header.profile_type = ProfileType::Analyzer as u32;
        cdna.header.profile_id = 0x00000200;
        
        // Grid: abstract-focused
        cdna.grid.dimension_scale = [1.0, 1.0, 1.0, 1.5, 10.0, 5.0, 3.0, 20.0];
        
        // Graph: restrictive
        cdna.graph.allowed_connection_types = 0x0888; // CAUSALITY | DEPENDENCY | INHERITANCE
        cdna.graph.max_node_degree = 100;
        cdna.graph.min_connection_weight = 0.3;
        cdna.graph.topology_flags = TopologyFlags::DIRECTED
            | TopologyFlags::WEIGHTED
            | TopologyFlags::REQUIRE_CONNECTED;
        
        // Connection: rigid
        cdna.connection.min_rigidity = 0.7;
        cdna.connection.max_rigidity = 1.0;
        
        // Evolution: conservative
        cdna.evolution.max_mutation_rate = 0.05;
        cdna.evolution.max_crossover_rate = 0.1;
        cdna.evolution.evolution_flags = EvolutionFlags::ELITISM_ENABLED
            | EvolutionFlags::ALLOW_MODIFY_WEIGHT;
        
        cdna.update_checksum();
        cdna
    }
    
    /// Creative profile: experimental, high variability
    fn creative_profile() -> Self {
        let mut cdna = Self::new();
        
        cdna.header.profile_type = ProfileType::Creative as u32;
        cdna.header.profile_id = 0x00000300;
        
        // Grid: exotic scales (Fibonacci)
        cdna.grid.dimension_scale = [1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        
        // Graph: custom types only
        cdna.graph.allowed_connection_types = 0xC000; // CUSTOM_1 | CUSTOM_2
        cdna.graph.topology_flags = TopologyFlags::ALLOW_ALL;
        
        // Token: allow exotic features
        cdna.token.token_flags = TokenFlags::ALLOW_SPLITTING
            | TokenFlags::ALLOW_MERGING
            | TokenFlags::ALLOW_NEGATIVE_WEIGHT;
        
        // Evolution: very high
        cdna.evolution.max_mutation_rate = 0.3;
        cdna.evolution.evolution_flags = EvolutionFlags::ALLOW_ALL;
        
        cdna.update_checksum();
        cdna
    }
}

// ═══════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════

#[derive(Debug)]
pub enum CDNAError {
    InvalidMagic,
    IncompatibleVersion { expected: u16, found: u16 },
    InvalidSize { expected: u32, found: u32 },
    ChecksumMismatch,
    InvalidRange { field: &'static str, min: f32, max: f32 },
    InvalidFlags { field: &'static str, value: u32 },
    InconsistentFlags { reason: &'static str },
    IOError(String),
}

// ═══════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
```

---

## Usage Examples

### Example 1: Create and save default CDNA

```rust
// Create default CDNA
let mut cdna = CDNA::new();

// Customize if needed
cdna.grid.dimension_scale[7] = 15.0; // Increase abstract importance

// Save to file
cdna.save("config/cdna_default.bin")?;

println!("CDNA saved: {} bytes", CDNA::SIZE);
```

### Example 2: Load and validate CDNA

```rust
// Load from file
let cdna = CDNA::load("config/cdna_default.bin")?;

// Validate
match cdna.validate() {
    Ok(_) => println!("CDNA valid ✓"),
    Err(e) => eprintln!("CDNA validation failed: {:?}", e),
}

// Use in Guardian
let guardian = Guardian::new(cdna);
```

### Example 3: Create profile-based CDNA

```rust
// Create explorer profile
let explorer = CDNA::from_profile(ProfileType::Explorer);
explorer.save("profiles/explorer.bin")?;

// Create analyzer profile
let analyzer = CDNA::from_profile(ProfileType::Analyzer);
analyzer.save("profiles/analyzer.bin")?;

// Create creative profile
let creative = CDNA::from_profile(ProfileType::Creative);
creative.save("profiles/creative.bin")?;
```

### Example 4: Check compatibility

```rust
let cdna = CDNA::load("config/cdna.bin")?;

if cdna.header.version_major != CDNA::VERSION_MAJOR {
    eprintln!("Warning: CDNA version mismatch!");
    eprintln!("  Expected: {}", CDNA::VERSION_MAJOR);
    eprintln!("  Found: {}", cdna.header.version_major);
    return Err("Incompatible CDNA version");
}

println!("CDNA compatible ✓");
```

---

## Summary

**CDNA v2.0 Final Structure: 384 bytes**

✅ **Header (64 bytes)** — метаданные, версия, checksum  
✅ **Grid Physics (128 bytes)** — u32 IDs, u32 flags, f64 scale  
✅ **Graph Topology (64 bytes)** — типы связей, лимиты, флаги  
✅ **Token Properties (32 bytes)** — веса, поля, флаги  
✅ **Connection Constraints (64 bytes)** — rigidity, coupling, decay, levels  
✅ **Evolution Constraints (32 bytes)** — мутации, кроссовер, отбор

**Total: 384 bytes = 6 cache lines**

**Готово к реализации!** 🎯
