/// Archive module - long-term storage and compression
///
/// This module contains structures and utilities for archiving
/// experiences in a compressed format for long-term storage
/// and later replay/analysis.

pub mod experience_token;

pub use experience_token::{
    ExperienceToken,
    ExperienceFlags,
    InfoFlags,
    EXPERIENCE_TOKEN_MAGIC,
};
