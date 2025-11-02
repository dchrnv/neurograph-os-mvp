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
pub mod connection;
pub mod grid;
pub mod graph;
pub mod cdna;
pub mod guardian;
pub mod experience_stream;
pub mod adna;

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

pub use experience_stream::{
    ExperienceStream,
    ExperienceEvent,
    EventType as StreamEventType,
    EventFlags,
    HotBuffer,
    SamplingStrategy,
    StreamError,
};

pub use adna::{
    ADNA,
    ADNAHeader,
    ADNAParameters,
    EvolutionMetrics,
    PolicyPointer,
    PolicyType,
    ADNAProfile,
    ADNAError,
    ADNA_MAGIC,
    ADNA_VERSION_MAJOR,
    ADNA_VERSION_MINOR,
};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 23;
pub const VERSION_PATCH: u8 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.23.0");
    }
}
