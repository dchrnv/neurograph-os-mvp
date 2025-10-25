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
/// - Zero external dependencies (pure Rust)

pub mod token;
pub mod connection;

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

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 13;
pub const VERSION_PATCH: u8 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.13.0");
    }
}
