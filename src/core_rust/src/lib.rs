pub mod adna;
pub mod appraisers;
pub mod cdna;
pub mod connection;
pub mod experience_stream;
pub mod graph;
pub mod grid;
pub mod guardian;
pub mod learner;
/// NeuroGraph OS Core - Rust Implementation
///
/// This is the core Rust implementation of NeuroGraph OS
///
/// # Architecture
///
/// - Token V2.0: 64-byte atomic unit of information
/// - Connection V1.0: 32-byte link between tokens
/// - 8-dimensional semantic space (L1-L8)
/// - Binary-compatible format for cross-language interop
/// - Python FFI bindings via PyO3 (optional)
/// - Zero core dependencies (pure Rust)
pub mod token;

#[cfg(feature = "python")]
pub mod ffi;

pub use token::{flags as token_flags, CoordinateSpace, EntityType, Token, SCALE_FACTORS};

pub use connection::{active_levels, flags as connection_flags, Connection, ConnectionType};

pub use grid::{Grid, GridConfig};

pub use graph::{Direction, EdgeId, EdgeInfo, Graph, GraphConfig, NodeId, Path, Subgraph};

pub use cdna::{
    CDNAFlags, ProfileId, ProfileState, CDNA, CDNA_MAGIC, CDNA_VERSION_MAJOR, CDNA_VERSION_MINOR,
};

pub use guardian::{Event, EventType, Guardian, GuardianConfig, Subscription, ValidationError};

pub use experience_stream::{
    EventFlags, EventType as StreamEventType, ExperienceEvent, ExperienceStream, HotBuffer,
    SamplingStrategy, StreamError,
};

pub use adna::{
    ADNAError, ADNAHeader, ADNAParameters, ADNAProfile, EvolutionMetrics, PolicyPointer,
    PolicyType, ADNA, ADNA_MAGIC, ADNA_VERSION_MAJOR, ADNA_VERSION_MINOR,
};

pub use appraisers::{
    Appraiser, AppraisersManager, CuriosityAppraiser, EfficiencyAppraiser, GoalDirectedAppraiser,
    HomeostasisAppraiser,
};

pub use learner::{
    EdgeId as LearnerEdgeId, // Alias to avoid conflict with graph::EdgeId
    HebbianRule,
    Learner,
    LearnerMetrics,
    LearningConfig,
    LearningMode,
    WeightUpdate,
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
