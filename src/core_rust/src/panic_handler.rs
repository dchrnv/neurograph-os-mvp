// NeuroGraph - Panic Recovery v0.41.0
// Copyright (C) 2024-2025 Chernov Denys
//
// Provides panic recovery infrastructure for production resilience.

use std::panic::{self, AssertUnwindSafe};
use tracing::{error, warn};

/// Result type for panic-recoverable operations
pub type PanicResult<T> = Result<T, PanicError>;

/// Error type for panic recovery
#[derive(Debug, Clone)]
pub struct PanicError {
    pub message: String,
    pub location: Option<String>,
}

impl std::fmt::Display for PanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref loc) = self.location {
            write!(f, "Panic at {}: {}", loc, self.message)
        } else {
            write!(f, "Panic: {}", self.message)
        }
    }
}

impl std::error::Error for PanicError {}

/// Catches panics in a closure and returns Result
///
/// # Examples
///
/// ```rust
/// use neurograph_core::panic_handler::catch_panic;
///
/// let result = catch_panic("task_name", || {
///     // Potentially panicking code
///     dangerous_operation();
///     "success"
/// });
///
/// match result {
///     Ok(value) => println!("Success: {}", value),
///     Err(e) => eprintln!("Recovered from panic: {}", e),
/// }
/// ```
pub fn catch_panic<F, T>(operation_name: &str, f: F) -> PanicResult<T>
where
    F: FnOnce() -> T + panic::UnwindSafe,
{
    match panic::catch_unwind(f) {
        Ok(result) => Ok(result),
        Err(panic_payload) => {
            let panic_message = extract_panic_message(&panic_payload);

            error!(
                operation = operation_name,
                panic_message = %panic_message,
                "Panic recovered"
            );

            Err(PanicError {
                message: format!("{}: {}", operation_name, panic_message),
                location: None,
            })
        }
    }
}

/// Async version of catch_panic
///
/// # Examples
///
/// ```rust
/// use neurograph_core::panic_handler::catch_panic_async;
///
/// let result = catch_panic_async("async_task", async {
///     // Potentially panicking async code
///     risky_async_operation().await;
///     "success"
/// }).await;
/// ```
pub async fn catch_panic_async<F, Fut, T>(operation_name: &str, f: F) -> PanicResult<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let future = AssertUnwindSafe(f());

    match panic::catch_unwind(|| {
        // Run the future in blocking context for panic catching
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(future)
        })
    }) {
        Ok(result) => Ok(result),
        Err(panic_payload) => {
            let panic_message = extract_panic_message(&panic_payload);

            error!(
                operation = operation_name,
                panic_message = %panic_message,
                "Async panic recovered"
            );

            Err(PanicError {
                message: format!("{}: {}", operation_name, panic_message),
                location: None,
            })
        }
    }
}

/// Extract human-readable message from panic payload
fn extract_panic_message(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic type".to_string()
    }
}

/// Install global panic hook for production
///
/// This should be called once at application startup.
/// It logs all panics with full backtraces.
///
/// # Examples
///
/// ```rust
/// use neurograph_core::panic_handler::install_panic_hook;
///
/// fn main() {
///     install_panic_hook();
///
///     // Application code...
/// }
/// ```
pub fn install_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let location = panic_info.location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());

        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic message".to_string()
        };

        error!(
            location = %location,
            message = %message,
            backtrace = ?std::backtrace::Backtrace::capture(),
            "PANIC OCCURRED"
        );

        eprintln!("==========================================================");
        eprintln!("PANIC at {}", location);
        eprintln!("Message: {}", message);
        eprintln!("==========================================================");
    }));
}

/// Macro for wrapping code in panic recovery
///
/// # Examples
///
/// ```rust
/// use neurograph_core::recover_panic;
///
/// let result = recover_panic!("my_operation", {
///     dangerous_function();
///     42
/// });
/// ```
#[macro_export]
macro_rules! recover_panic {
    ($name:expr, $body:expr) => {
        $crate::panic_handler::catch_panic($name, || $body)
    };
}

/// Async version of recover_panic macro
///
/// # Examples
///
/// ```rust
/// use neurograph_core::recover_panic_async;
///
/// let result = recover_panic_async!("async_op", {
///     risky_async_call().await;
///     42
/// }).await;
/// ```
#[macro_export]
macro_rules! recover_panic_async {
    ($name:expr, $body:expr) => {
        $crate::panic_handler::catch_panic_async($name, || async move { $body })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catch_panic_success() {
        let result = catch_panic("test", || 42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_catch_panic_str() {
        let result = catch_panic("test", || {
            panic!("test panic");
        });
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("test panic"));
    }

    #[test]
    fn test_catch_panic_string() {
        let result = catch_panic("test", || {
            panic!("{}", "formatted panic");
        });
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_catch_panic_async_success() {
        let result = catch_panic_async("async_test", || async { 42 }).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_catch_panic_async_error() {
        let result = catch_panic_async("async_test", || async {
            panic!("async panic");
        }).await;
        assert!(result.is_err());
    }
}
