/// NeuroGraph OS Core - Rust Implementation
///
/// This is the core Rust implementation of NeuroGraph OS
///
/// # Architecture
///
/// - Token V2.0: 64-byte atomic unit of information
/// - Connection V1.0: 32-byte link between tokens
/// - 8-dimensional semantic space (L1-L8)
/// - ADNA v3.0: 256-byte Policy Engine
/// - ExperienceStream v2.1: Event-based memory system (128-byte events)
/// - Archive: Long-term compressed storage (ExperienceToken 128-byte)
/// - Binary-compatible format for cross-language interop
/// - Python FFI bindings via PyO3 (optional)
/// - Zero core dependencies (pure Rust)

pub mod token;
pub mod connection;
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
pub mod executors;
pub mod persistence;

#[cfg(feature = "python")]
pub mod ffi;

pub use token::{
    Token,
    CoordinateSpace,
    EntityType,
    flags as token_flags,
    SCALE_FACTORS,
};

pub use connection::{
    Connection,
    ConnectionType,
    active_levels,
    flags as connection_flags,
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
    IntuitionConfig,
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

pub use action_controller::{
    ActionController,
    ActionControllerConfig,
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
