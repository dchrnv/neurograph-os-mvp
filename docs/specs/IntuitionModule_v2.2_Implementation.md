# Intuition Module v2.2 — Implementation Specification

**Version:** 2.2.0
**Date:** 2025-11-12
**Status:** Implementation Ready
**Dependencies:**
- ExperienceStream v2.1 ✅ (implemented)
- ADNA v3.0 ✅ (implemented, needs extension)
- Token v2.0 ✅ (implemented)
- Connection v1.0 ✅ (implemented)

**Language:** Rust + tokio async runtime
**Target:** Full implementation of 4 Appraisers + Coordinator

---

## 📋 TABLE OF CONTENTS

1. [Architecture Overview](#1-architecture-overview)
2. [L1-L8 Coordinate System](#2-l1-l8-coordinate-system)
3. [ADNA Extension for Appraisers](#3-adna-extension-for-appraisers)
4. [Appraiser Implementations](#4-appraiser-implementations)
5. [AppraiserSet Coordinator](#5-appraiserset-coordinator)
6. [Integration Points](#6-integration-points)
7. [Implementation Roadmap](#7-implementation-roadmap)

---

## 1. ARCHITECTURE OVERVIEW

### 1.1 What We Already Have ✅

```rust
// ✅ ExperienceStream v2.1 - READY
pub struct ExperienceStream {
    buffer: Arc<HotBuffer>,
    tx: broadcast::Sender<ExperienceEvent>,
}

// ✅ ExperienceEvent - READY (128 bytes, 4 reward components)
pub struct ExperienceEvent {
    pub event_id: u128,           // 16 bytes
    pub timestamp: u64,            // 8 bytes
    pub episode_id: u64,           // 8 bytes
    pub step_number: u32,          // 4 bytes
    pub event_type: u16,           // 2 bytes
    pub flags: u16,                // 2 bytes
    pub state: [f32; 8],           // 32 bytes - L1-L8 coordinates
    pub action: [f32; 8],          // 32 bytes - L1-L8 action space
    pub reward_homeostasis: f32,   // 4 bytes ✅
    pub reward_curiosity: f32,     // 4 bytes ✅
    pub reward_efficiency: f32,    // 4 bytes ✅
    pub reward_goal: f32,          // 4 bytes ✅
    pub adna_version_hash: u32,    // 4 bytes
    pub _reserved: [u8; 4],        // 4 bytes
}

// ✅ Lock-free reward updates - READY
stream.set_appraiser_reward(seq, AppraiserType::Homeostasis, reward)?;

// ✅ Pub-Sub system - READY
let mut rx = stream.subscribe();
```

### 1.2 What We Need to Implement 🔨

```
src/core_rust/src/
├── coordinates.rs           // 🆕 L1-L8 coordinate helpers
├── adna.rs                   // ⚡ Extend for appraiser params
├── intuition/
│   ├── mod.rs                // 🆕 Module root
│   ├── appraiser_trait.rs    // 🆕 Common Appraiser trait
│   ├── appraisers/
│   │   ├── mod.rs            // 🆕 Appraisers root
│   │   ├── homeostasis.rs    // 🆕 Homeostasis appraiser
│   │   ├── curiosity.rs      // 🆕 Curiosity appraiser
│   │   ├── efficiency.rs     // 🆕 Efficiency appraiser
│   │   └── goal_directed.rs  // 🆕 Goal-directed appraiser
│   └── appraiser_set.rs      // 🆕 Coordinator
└── lib.rs                    // ⚡ Add intuition module exports
```

---

## 2. L1-L8 COORDINATE SYSTEM

### 2.1 Semantic Mapping

Our `ExperienceEvent` has:
- `state: [f32; 8]` - current state coordinates
- `action: [f32; 8]` - action taken

**Mapping to L1-L8:**

```rust
// src/core_rust/src/coordinates.rs

/// L1-L8 Coordinate indices in state[]/action[] arrays
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateIndex {
    L1_Existence = 0,     // Physical presence/activation
    L2_Novelty = 1,       // Sensory novelty (0.0 = known, 1.0 = new)
    L3_Velocity = 2,      // Motor activity/speed
    L4_Attention = 3,     // Focus/attention level
    L5_CognitiveLoad = 4, // Mental effort/complexity
    L6_Certainty = 5,     // Confidence/predictability
    L7_Valence = 6,       // Emotional value (positive/negative)
    L8_Coherence = 7,     // Logical consistency
}

impl CoordinateIndex {
    pub fn as_usize(self) -> usize {
        self as usize
    }
}

/// Helper functions to extract coordinates from ExperienceEvent
impl ExperienceEvent {
    // L1: Existence (базовая активация)
    pub fn l1_existence(&self) -> f32 {
        self.state[CoordinateIndex::L1_Existence.as_usize()]
    }

    // L2: Novelty (новизна стимула)
    pub fn l2_novelty(&self) -> f32 {
        self.state[CoordinateIndex::L2_Novelty.as_usize()]
    }

    // L3: Velocity (скорость движения/изменения)
    pub fn l3_velocity(&self) -> f32 {
        self.state[CoordinateIndex::L3_Velocity.as_usize()]
    }

    // L3: Acceleration (из action[])
    pub fn l3_acceleration(&self) -> f32 {
        self.action[CoordinateIndex::L3_Velocity.as_usize()]
    }

    // L4: Attention (уровень внимания)
    pub fn l4_attention(&self) -> f32 {
        self.state[CoordinateIndex::L4_Attention.as_usize()]
    }

    // L5: Cognitive Load (когнитивная нагрузка)
    pub fn l5_cognitive_load(&self) -> f32 {
        self.state[CoordinateIndex::L5_CognitiveLoad.as_usize()]
    }

    // L6: Certainty (уверенность/предсказуемость)
    pub fn l6_certainty(&self) -> f32 {
        self.state[CoordinateIndex::L6_Certainty.as_usize()]
    }

    // L7: Valence (эмоциональная окраска)
    pub fn l7_valence(&self) -> f32 {
        self.state[CoordinateIndex::L7_Valence.as_usize()]
    }

    // L8: Coherence (логическая согласованность)
    pub fn l8_coherence(&self) -> f32 {
        self.state[CoordinateIndex::L8_Coherence.as_usize()]
    }
}
```

**Ranges and Normalization:**
- All coordinates: `[0.0, 1.0]` (normalized)
- L7 Valence: `[-1.0, 1.0]` (exception for positive/negative)

---

## 3. ADNA EXTENSION FOR APPRAISERS

### 3.1 Current ADNA Structure (v3.0)

```rust
// Existing in src/core_rust/src/adna.rs
pub struct ADNA {
    pub header: ADNAHeader,
    pub evolution_metrics: EvolutionMetrics,
    pub policy_pointers: [PolicyPointer; 8],  // L1-L8 policies
    pub state_mappings: [StateMapping; 8],     // State → Policy
    // ... existing fields ...
}
```

### 3.2 Extension to v3.1 - Appraiser Parameters

```rust
// 🆕 ADD TO src/core_rust/src/adna.rs

/// Homeostasis Appraiser configuration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HomeostasisParams {
    /// Weight (importance) of this appraiser
    pub weight: f32,

    /// Target ranges for key coordinates
    pub cognitive_load_range: (f32, f32),  // (min, max) e.g. (0.2, 0.7)
    pub certainty_range: (f32, f32),       // e.g. (0.5, 0.9)
    pub coherence_range: (f32, f32),       // e.g. (0.6, 1.0)

    /// Penalty multiplier for deviations
    pub penalty_multiplier: f32,           // default: 1.0
}

impl Default for HomeostasisParams {
    fn default() -> Self {
        Self {
            weight: 0.25,  // 25% of total reward
            cognitive_load_range: (0.2, 0.7),
            certainty_range: (0.5, 0.9),
            coherence_range: (0.6, 1.0),
            penalty_multiplier: 1.0,
        }
    }
}

/// Curiosity Appraiser configuration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CuriosityParams {
    pub weight: f32,
    pub novelty_threshold: f32,   // Minimum novelty to trigger reward
    pub reward_scale: f32,        // Multiplier for novelty reward
}

impl Default for CuriosityParams {
    fn default() -> Self {
        Self {
            weight: 0.25,
            novelty_threshold: 0.3,
            reward_scale: 1.0,
        }
    }
}

/// Efficiency Appraiser configuration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfficiencyParams {
    pub weight: f32,
    pub motor_cost_factor: f32,      // Penalty for L3 velocity/acceleration
    pub cognitive_cost_factor: f32,  // Penalty for L5 cognitive load
    pub creation_cost_base: f32,     // Base cost for creating tokens/connections
}

impl Default for EfficiencyParams {
    fn default() -> Self {
        Self {
            weight: 0.20,
            motor_cost_factor: 0.1,
            cognitive_cost_factor: 0.15,
            creation_cost_base: 0.05,
        }
    }
}

/// Goal-Directed Appraiser configuration
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GoalDirectedParams {
    pub weight: f32,
    pub gamma: f32,  // Temporal discounting factor (0.0 < gamma <= 1.0)

    // MVP: Simple goal state matching
    pub goal_state: [f32; 8],  // Target state in L1-L8 space
    pub goal_threshold: f32,   // Distance threshold to consider goal reached
}

impl Default for GoalDirectedParams {
    fn default() -> Self {
        Self {
            weight: 0.30,
            gamma: 0.95,
            goal_state: [0.0; 8],  // Will be set dynamically
            goal_threshold: 0.2,
        }
    }
}

/// Complete appraiser configuration (64 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AppraiserConfig {
    pub homeostasis: HomeostasisParams,
    pub curiosity: CuriosityParams,
    pub efficiency: EfficiencyParams,
    pub goal_directed: GoalDirectedParams,
}

impl Default for AppraiserConfig {
    fn default() -> Self {
        Self {
            homeostasis: HomeostasisParams::default(),
            curiosity: CuriosityParams::default(),
            efficiency: EfficiencyParams::default(),
            goal_directed: GoalDirectedParams::default(),
        }
    }
}

// 🆕 ADD THIS FIELD TO ADNA STRUCT
pub struct ADNA {
    // ... existing fields ...
    pub appraiser_config: AppraiserConfig,  // 🆕 NEW
}
```

### 3.3 ADNAReader Trait

```rust
// 🆕 ADD TO src/core_rust/src/adna.rs

/// Trait for reading ADNA configuration
/// Allows appraisers to access their parameters
pub trait ADNAReader: Send + Sync {
    fn get_homeostasis_params(&self) -> HomeostasisParams;
    fn get_curiosity_params(&self) -> CuriosityParams;
    fn get_efficiency_params(&self) -> EfficiencyParams;
    fn get_goal_directed_params(&self) -> GoalDirectedParams;
}

impl ADNAReader for ADNA {
    fn get_homeostasis_params(&self) -> HomeostasisParams {
        self.appraiser_config.homeostasis
    }

    fn get_curiosity_params(&self) -> CuriosityParams {
        self.appraiser_config.curiosity
    }

    fn get_efficiency_params(&self) -> EfficiencyParams {
        self.appraiser_config.efficiency
    }

    fn get_goal_directed_params(&self) -> GoalDirectedParams {
        self.appraiser_config.goal_directed
    }
}
```

---

## 4. APPRAISER IMPLEMENTATIONS

### 4.1 Common Appraiser Trait

```rust
// src/core_rust/src/intuition/appraiser_trait.rs

use crate::experience_stream::ExperienceEvent;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Common trait for all appraisers
#[async_trait::async_trait]
pub trait Appraiser: Send + Sync {
    /// Get appraiser name for logging
    fn name(&self) -> &'static str;

    /// Get appraiser type
    fn appraiser_type(&self) -> AppraiserType;

    /// Main processing loop - subscribes to event stream
    async fn run(&mut self);

    /// Calculate reward for a single event (synchronous core logic)
    fn calculate_reward(&self, event: &ExperienceEvent) -> f32;
}

/// Error type for appraiser operations
#[derive(Debug)]
pub enum AppraiserError {
    StreamClosed,
    ADNAReadError(String),
    RewardUpdateError(String),
}
```

### 4.2 HomeostasisAppraiser Implementation

```rust
// src/core_rust/src/intuition/appraisers/homeostasis.rs

use crate::experience_stream::{ExperienceEvent, ExperienceStream, AppraiserType};
use crate::adna::{ADNAReader, HomeostasisParams};
use crate::intuition::appraiser_trait::{Appraiser, AppraiserError};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct HomeostasisAppraiser {
    stream: Arc<ExperienceStream>,
    adna: Arc<dyn ADNAReader>,
    params: HomeostasisParams,
}

impl HomeostasisAppraiser {
    pub fn new(stream: Arc<ExperienceStream>, adna: Arc<dyn ADNAReader>) -> Self {
        let params = adna.get_homeostasis_params();
        Self { stream, adna, params }
    }

    /// Reload parameters from ADNA (for live updates)
    pub fn reload_params(&mut self) {
        self.params = self.adna.get_homeostasis_params();
    }
}

#[async_trait::async_trait]
impl Appraiser for HomeostasisAppraiser {
    fn name(&self) -> &'static str {
        "HomeostasisAppraiser"
    }

    fn appraiser_type(&self) -> AppraiserType {
        AppraiserType::Homeostasis
    }

    async fn run(&mut self) {
        let mut rx = self.stream.subscribe();
        let mut processed_count = 0u64;

        loop {
            match rx.recv().await {
                Ok(event) => {
                    // Calculate reward
                    let reward = self.calculate_reward(&event);

                    // Update reward if significant
                    if reward.abs() > 1e-6 {
                        let seq = self.stream.total_written() - 1;
                        if let Err(e) = self.stream.set_appraiser_reward(
                            seq,
                            AppraiserType::Homeostasis,
                            reward
                        ) {
                            eprintln!("[{}] Error setting reward: {}", self.name(), e);
                        }
                    }

                    processed_count += 1;
                }
                Err(_) => {
                    // Channel closed, exit gracefully
                    break;
                }
            }
        }
    }

    fn calculate_reward(&self, event: &ExperienceEvent) -> f32 {
        let mut total_penalty = 0.0;

        // Penalty for cognitive load outside target range
        let cognitive_load = event.l5_cognitive_load();
        let (cl_min, cl_max) = self.params.cognitive_load_range;
        let cl_deviation = if cognitive_load < cl_min {
            cl_min - cognitive_load
        } else if cognitive_load > cl_max {
            cognitive_load - cl_max
        } else {
            0.0
        };
        total_penalty += cl_deviation * cl_deviation;

        // Penalty for certainty outside target range
        let certainty = event.l6_certainty();
        let (cert_min, cert_max) = self.params.certainty_range;
        let cert_deviation = if certainty < cert_min {
            cert_min - certainty
        } else if certainty > cert_max {
            certainty - cert_max
        } else {
            0.0
        };
        total_penalty += cert_deviation * cert_deviation;

        // Penalty for coherence outside target range
        let coherence = event.l8_coherence();
        let (coh_min, coh_max) = self.params.coherence_range;
        let coh_deviation = if coherence < coh_min {
            coh_min - coherence
        } else if coherence > coh_max {
            coherence - coh_max
        } else {
            0.0
        };
        total_penalty += coh_deviation * coh_deviation;

        // Return negative weighted penalty
        -self.params.weight * self.params.penalty_multiplier * total_penalty
    }
}
```

### 4.3 CuriosityAppraiser Implementation

```rust
// src/core_rust/src/intuition/appraisers/curiosity.rs

pub struct CuriosityAppraiser {
    stream: Arc<ExperienceStream>,
    adna: Arc<dyn ADNAReader>,
    params: CuriosityParams,
}

impl CuriosityAppraiser {
    pub fn new(stream: Arc<ExperienceStream>, adna: Arc<dyn ADNAReader>) -> Self {
        let params = adna.get_curiosity_params();
        Self { stream, adna, params }
    }
}

#[async_trait::async_trait]
impl Appraiser for CuriosityAppraiser {
    fn name(&self) -> &'static str {
        "CuriosityAppraiser"
    }

    fn appraiser_type(&self) -> AppraiserType {
        AppraiserType::Curiosity
    }

    async fn run(&mut self) {
        // Similar structure to HomeostasisAppraiser
        // ... (implementation follows same pattern)
    }

    fn calculate_reward(&self, event: &ExperienceEvent) -> f32 {
        let novelty = event.l2_novelty();

        // Only reward if novelty exceeds threshold
        if novelty >= self.params.novelty_threshold {
            self.params.weight * self.params.reward_scale * novelty
        } else {
            0.0
        }
    }
}
```

### 4.4 EfficiencyAppraiser Implementation

```rust
// src/core_rust/src/intuition/appraisers/efficiency.rs

pub struct EfficiencyAppraiser {
    stream: Arc<ExperienceStream>,
    adna: Arc<dyn ADNAReader>,
    params: EfficiencyParams,
}

#[async_trait::async_trait]
impl Appraiser for EfficiencyAppraiser {
    fn name(&self) -> &'static str {
        "EfficiencyAppraiser"
    }

    fn appraiser_type(&self) -> AppraiserType {
        AppraiserType::Efficiency
    }

    async fn run(&mut self) {
        // ... (similar pattern)
    }

    fn calculate_reward(&self, event: &ExperienceEvent) -> f32 {
        let mut total_cost = 0.0;

        // Motor cost (L3 velocity + acceleration)
        let velocity = event.l3_velocity();
        let acceleration = event.l3_acceleration();
        total_cost += self.params.motor_cost_factor * (velocity.powi(2) + acceleration.powi(2));

        // Cognitive cost (L5 cognitive load)
        let cognitive_load = event.l5_cognitive_load();
        total_cost += self.params.cognitive_cost_factor * cognitive_load;

        // Creation cost (if event type is creation)
        if event.event_type == EventType::TokenCreated as u16
            || event.event_type == EventType::ConnectionCreated as u16
        {
            total_cost += self.params.creation_cost_base;
        }

        // Return negative weighted cost
        -self.params.weight * total_cost
    }
}
```

### 4.5 GoalDirectedAppraiser Implementation (MVP)

```rust
// src/core_rust/src/intuition/appraisers/goal_directed.rs

pub struct GoalDirectedAppraiser {
    stream: Arc<ExperienceStream>,
    adna: Arc<dyn ADNAReader>,
    params: GoalDirectedParams,
}

#[async_trait::async_trait]
impl Appraiser for GoalDirectedAppraiser {
    fn name(&self) -> &'static str {
        "GoalDirectedAppraiser"
    }

    fn appraiser_type(&self) -> AppraiserType {
        AppraiserType::Goal
    }

    async fn run(&mut self) {
        // MVP: Simple goal state matching
        // TODO: Add trajectory tracing in future versions
        // ... (similar pattern)
    }

    fn calculate_reward(&self, event: &ExperienceEvent) -> f32 {
        // MVP: Calculate distance to goal state in L1-L8 space
        let mut distance_squared = 0.0;

        for i in 0..8 {
            let diff = event.state[i] - self.params.goal_state[i];
            distance_squared += diff * diff;
        }

        let distance = distance_squared.sqrt();

        // Reward proximity to goal
        if distance < self.params.goal_threshold {
            // Close to goal - high reward
            self.params.weight * (1.0 - distance / self.params.goal_threshold)
        } else {
            // Far from goal - small penalty
            -self.params.weight * 0.1
        }
    }
}
```

---

## 5. APPRAISERSET COORDINATOR

```rust
// src/core_rust/src/intuition/appraiser_set.rs

use crate::experience_stream::ExperienceStream;
use crate::adna::ADNAReader;
use crate::intuition::appraisers::*;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct AppraiserSet {
    homeostasis: HomeostasisAppraiser,
    curiosity: CuriosityAppraiser,
    efficiency: EfficiencyAppraiser,
    goal_directed: GoalDirectedAppraiser,

    handles: Vec<JoinHandle<()>>,
}

impl AppraiserSet {
    pub fn new(stream: Arc<ExperienceStream>, adna: Arc<dyn ADNAReader>) -> Self {
        Self {
            homeostasis: HomeostasisAppraiser::new(Arc::clone(&stream), Arc::clone(&adna)),
            curiosity: CuriosityAppraiser::new(Arc::clone(&stream), Arc::clone(&adna)),
            efficiency: EfficiencyAppraiser::new(Arc::clone(&stream), Arc::clone(&adna)),
            goal_directed: GoalDirectedAppraiser::new(Arc::clone(&stream), Arc::clone(&adna)),
            handles: Vec::new(),
        }
    }

    /// Start all appraisers concurrently
    pub fn start(&mut self) {
        // Spawn each appraiser in separate tokio task
        let h1 = tokio::spawn(async move {
            self.homeostasis.run().await;
        });

        let h2 = tokio::spawn(async move {
            self.curiosity.run().await;
        });

        let h3 = tokio::spawn(async move {
            self.efficiency.run().await;
        });

        let h4 = tokio::spawn(async move {
            self.goal_directed.run().await;
        });

        self.handles = vec![h1, h2, h3, h4];
    }

    /// Gracefully shutdown all appraisers
    pub async fn shutdown(self) {
        for handle in self.handles {
            let _ = handle.await;
        }
    }
}
```

---

## 6. INTEGRATION POINTS

### 6.1 Module Exports

```rust
// src/core_rust/src/intuition/mod.rs

pub mod appraiser_trait;
pub mod appraisers;
pub mod appraiser_set;

pub use appraiser_trait::{Appraiser, AppraiserError};
pub use appraiser_set::AppraiserSet;
```

```rust
// src/core_rust/src/lib.rs

pub mod intuition;

pub use intuition::{
    Appraiser,
    AppraiserError,
    AppraiserSet,
};
```

### 6.2 Usage Example

```rust
use neurograph_core::{
    ExperienceStream,
    ADNA,
    AppraiserSet,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create stream
    let stream = Arc::new(ExperienceStream::new(10_000, 1024));

    // Load ADNA config
    let adna = Arc::new(ADNA::default());

    // Create and start all appraisers
    let mut appraisers = AppraiserSet::new(stream, adna);
    appraisers.start();

    // ... write events to stream ...

    // Shutdown
    appraisers.shutdown().await;
}
```

---

## 7. IMPLEMENTATION ROADMAP

### Phase 1: Foundation ✅
- [x] ExperienceStream v2.1
- [x] ExperienceEvent with 4 reward components
- [x] Lock-free reward updates
- [x] Pub-sub system

### Phase 2: Coordinates (Current)
- [ ] Create `coordinates.rs` module
- [ ] Define L1-L8 mapping
- [ ] Add helper methods to ExperienceEvent

### Phase 3: ADNA Extension
- [ ] Add appraiser parameter structs
- [ ] Extend ADNA to v3.1
- [ ] Implement ADNAReader trait

### Phase 4: Appraisers
- [ ] Implement HomeostasisAppraiser
- [ ] Implement CuriosityAppraiser
- [ ] Implement EfficiencyAppraiser
- [ ] Implement GoalDirectedAppraiser (MVP)

### Phase 5: Coordinator
- [ ] Implement AppraiserSet
- [ ] Add graceful shutdown
- [ ] Add monitoring/logging

### Phase 6: Integration & Testing
- [ ] Unit tests for each appraiser
- [ ] Integration demo
- [ ] Performance benchmarks

### Phase 7: Documentation Cleanup
- [ ] Update version to v2.2
- [ ] Clean up docs/arch/2,3,4
- [ ] Create final API documentation

---

## 🎯 SUCCESS CRITERIA

✅ All 4 appraisers running concurrently
✅ Lock-free reward updates working correctly
✅ Parameters loaded from ADNA v3.1
✅ L1-L8 coordinates properly mapped
✅ Demo showing complete workflow
✅ Performance: > 10K events/sec throughput

---

## 📚 REFERENCES

- ExperienceStream v2.1 spec: `docs/specs/ExperienceStream_v2.1.md`
- ADNA v3.0 spec: `docs/specs/ADNA_v3.0.md`
- Original appraiser specs: `docs/arch/2/*.md` (to be archived)

---

**Status:** Ready for implementation
**Next Step:** Create `coordinates.rs` module with L1-L8 mapping
