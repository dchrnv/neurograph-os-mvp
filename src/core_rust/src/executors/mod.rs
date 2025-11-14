//! Action Executors - concrete implementations of ActionExecutor trait
//!
//! This module contains various action executors that can be registered
//! with ActionController to perform specific tasks.

mod noop;
mod message_sender;

pub use noop::NoOpExecutor;
pub use message_sender::MessageSenderExecutor;