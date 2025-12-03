// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/// NeuroGraph OS Core - Rust Implementation
///
/// This is the core Rust implementation of NeuroGraph OS
///
/// # Architecture
///
/// - Token V2.0: 64-byte atomic unit of information
/// - Connection V3.0: 64-byte learning-capable link with Guardian integration
/// - 8-dimensional semantic space (L1-L8)
/// - ADNA v3.0: 256-byte Policy Engine
/// - ExperienceStream v2.1: Event-based memory system (128-byte events)
/// - Archive: Long-term compressed storage (ExperienceToken 128-byte)
/// - HybridLearning v2.2: ADNA ↔ Connection feedback loops
/// - Binary-compatible format for cross-language interop
/// - Python FFI bindings via PyO3 (optional)
/// - Zero core dependencies (pure Rust)

pub mod token;
pub mod connection_v3;

// Re-export Connection v3.0 types (primary API)
pub use connection_v3::{
    ConnectionV3,
    ConnectionType,
    ConnectionMutability,
    ConnectionProposal,
    ConnectionField,
};

// Alias for backward compatibility
pub type Connection = ConnectionV3;
pub mod grid;
pub mod graph;
pub mod cdna;
pub mod guardian;
pub mod adna;
pub mod coordinates;
pub mod experience_stream;
pub mod archive;
pub mod policy;
pub mod appraisers;
pub mod intuition_engine;
pub mod evolution_manager;
pub mod action_executor;
pub mod action_controller;
pub mod action_types;
pub mod executors;
pub mod persistence;
pub mod hybrid_learning;  // NEW: v2.2 Hybrid Learning Integration (v0.30.2)
pub mod reflex_layer;     // NEW: v3.0 Reflex System (v0.31.0)
pub mod bootstrap;        // NEW: v1.2 Bootstrap Library (v0.33.0)
pub mod gateway;          // NEW: v1.0 Gateway (v0.35.0)
pub mod adapters;         // NEW: v1.0 Output/Input Adapters (v0.36.0)
pub mod feedback;         // NEW: v1.0 Feedback System (v0.37.0)
pub mod curiosity;        // NEW: v1.0 Curiosity Drive (v0.38.0)
pub mod api;              // NEW: v1.0 REST API (v0.39.0)
pub mod panic_handler;    // NEW: v1.0 Panic Recovery (v0.41.0)

// Python bindings v1.0 (v0.40.0) - PyO3 FFI
#[cfg(feature = "python")]
pub mod python;

// Old FFI (deprecated, will be removed in favor of python module)
// #[cfg(feature = "python")]
// pub mod ffi;

pub use token::{
    Token,
    CoordinateSpace,
    EntityType,
    flags as token_flags,
    SCALE_FACTORS,
};

pub use grid::{
    Grid,
    GridConfig,
};

pub use graph::{
    Graph,
    GraphConfig,
    NodeId,
    EdgeId,
    Direction,
    Path,
    Subgraph,
    EdgeInfo,
    // SignalSystem v1.0
    NodeActivation,
    SignalConfig,
    AccumulationMode,
    ActivationResult,
    ActivatedNode,
};

pub use cdna::{
    CDNA,
    ProfileId,
    ProfileState,
    CDNAFlags,
    CDNA_MAGIC,
    CDNA_VERSION_MAJOR,
    CDNA_VERSION_MINOR,
};

pub use guardian::{
    Guardian,
    GuardianConfig,
    Event,
    EventType,
    Subscription,
    ValidationError,
};

pub use adna::{
    ADNA,
    ADNAHeader,
    EvolutionMetrics,
    PolicyPointer,
    StateMapping,
    PolicyType,
    ADNA_MAGIC,
    ADNA_VERSION_MAJOR,
    ADNA_VERSION_MINOR,
    // Appraiser configuration
    HomeostasisParams,
    CuriosityParams,
    EfficiencyParams,
    GoalDirectedParams,
    AppraiserConfig,
    ADNAReader,
    ADNAError,
    InMemoryADNAReader,
    // Learning loop structures
    Proposal,
    Intent,
    ActionPolicy,
};

pub use coordinates::{
    CoordinateIndex,
    CoordinateExt,
};

pub use appraisers::{
    HomeostasisAppraiser,
    CuriosityAppraiser,
    EfficiencyAppraiser,
    GoalDirectedAppraiser,
    AppraiserSet,
};

pub use experience_stream::{
    ExperienceEvent,
    EventType as ExperienceEventType,
    EventFlags,
    AppraiserType,
    HotBuffer,
    ExperienceStream,
    ExperienceWriter,
    ExperienceReader,
    // Metadata for action events
    ActionMetadata,
    // Sampling for IntuitionEngine
    SamplingStrategy,
    ExperienceBatch,
};

pub use archive::{
    ExperienceToken,
    InfoFlags,
    EXPERIENCE_TOKEN_MAGIC,
};

pub use policy::{
    Policy,
    LinearPolicy,
    Gradient,
    GradientSource,
    PolicyError,
};

pub use intuition_engine::{
    IntuitionEngine,
    IntuitionEngineBuilder,
    IntuitionConfig,
    IdentifiedPattern,
};

pub use hybrid_learning::{
    HybridProposal,
    ProposalOutcome,
    ProposalRouter,
    HybridLearningStats,
    HybridLearningError,
    adna_to_connection_feedback,
    connection_to_adna_hint,
};

pub use reflex_layer::{
    ShiftConfig,
    AssociativeStats,
    AdaptiveTuningConfig,
    AdaptiveTuner,
    AssociativeMemory,
    FastPathResult,
    FastPathConfig,
    IntuitionStats,
    compute_grid_hash,
    token_similarity,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 26;
pub const VERSION_PATCH: u8 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.26.0");
    }
}

pub use evolution_manager::{
    EvolutionManager,
    EvolutionConfig,
    ADNAState,
    ValidationResult,
};

pub use action_executor::{
    ActionExecutor,
    ActionResult,
    ActionError,
};

pub use action_types::{
    ActionIntent,
    ActionType,
    DecisionSource,
};

pub use action_controller::{
    ActionController,
    ActionControllerConfig,
    ArbiterConfig,
    ArbiterStats,
};

pub use executors::{
    NoOpExecutor,
    MessageSenderExecutor,
};

// Persistence exports (only available with 'persistence' feature)
pub use persistence::{
    PersistenceBackend,
    PersistenceError,
    QueryOptions,
};

#[cfg(feature = "persistence")]
pub use persistence::{
    PostgresBackend,
};

// Bootstrap Library v1.2
pub use bootstrap::{
    BootstrapLibrary,
    BootstrapConfig,
    SemanticConcept,
    PCAModel,
    BootstrapError,
};

// Gateway v1.0
pub use gateway::{
    Gateway,
    GatewayError,
};

pub use gateway::config::{
    GatewayConfig,
    UnknownWordStrategy,
};

pub use gateway::signals::{
    InputSignal,
    ProcessedSignal,
    SignalSource,
    SignalType,
    SystemCommand,
    FeedbackType,
    TokenOperation,
    ProcessedMetadata,
};

pub use gateway::channels::{
    SignalReceipt,
    ResultReceiver,
};

pub use gateway::stats::{
    GatewayStats,
};

// Adapters v1.0
pub use adapters::{
    OutputAdapter,
    OutputContext,
    FormattedOutput,
    OutputError,
    SignalSource as AdapterSignalSource,
    SignalType as AdapterSignalType,
};

pub use adapters::console::{
    ConsoleOutputAdapter,
    ConsoleInputAdapter,
    ConsoleConfig,
};

// Feedback v1.0
pub use feedback::{
    FeedbackProcessor,
    FeedbackSignal,
    FeedbackResult,
    FeedbackError,
    DetailedFeedbackType,
};

// Curiosity v1.0
pub use curiosity::{
    CuriosityDrive,
    CuriosityConfig,
    CuriosityScore,
    CuriosityContext,
    CuriosityStats,
    ExplorationMode,
    ExplorationTarget,
    ExplorationReason,
    ExplorationPriority,
};

// Panic Recovery v1.0
pub use panic_handler::{
    catch_panic,
    catch_panic_async,
    install_panic_hook,
    PanicError,
    PanicResult,
};
