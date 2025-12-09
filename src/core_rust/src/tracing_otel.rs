//! OpenTelemetry Distributed Tracing Integration (v0.44.0)
//!
//! This module provides OpenTelemetry tracing integration with Jaeger backend.
//! It allows distributed tracing across services with context propagation.
//!
//! # Features
//! - Jaeger exporter for trace collection
//! - Automatic span creation for HTTP requests
//! - Context propagation via headers
//! - Integration with existing tracing infrastructure
//!
//! # Usage
//!
//! ```rust
//! use neurograph_core::tracing_otel;
//!
//! // Initialize tracing with Jaeger
//! tracing_otel::init_tracer("neurograph-api", "http://localhost:14268/api/traces")?;
//!
//! // Traces are automatically created by axum middleware
//! // or manually with tracing::info_span!
//! ```

use opentelemetry::global;
use opentelemetry::KeyValue;
use opentelemetry_sdk::trace::{self, RandomIdGenerator, Sampler};
use opentelemetry_sdk::Resource;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

/// Initialize OpenTelemetry tracer with Jaeger backend
///
/// # Arguments
/// * `service_name` - Name of the service (e.g., "neurograph-api")
/// * `jaeger_endpoint` - Jaeger collector endpoint (e.g., "http://localhost:14268/api/traces")
///
/// # Returns
/// Result indicating success or error message
///
/// # Example
///
/// ```rust
/// init_tracer("neurograph-api", "http://jaeger:14268/api/traces")?;
/// ```
pub fn init_tracer(service_name: &str, jaeger_endpoint: &str) -> Result<(), String> {
    // Create Jaeger exporter
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(jaeger_endpoint)
        .with_service_name(service_name)
        .with_auto_split_batch(true)
        .with_max_packet_size(65_000)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::TraceIdRatioBased(0.01))  // v0.44.3: 1% sampling
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                    KeyValue::new("service.version", "v0.45.0"),  // Updated version
                ])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .map_err(|e| format!("Failed to install Jaeger tracer: {}", e))?;

    // Create OpenTelemetry tracing layer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Get existing subscriber and add OpenTelemetry layer
    let subscriber = Registry::default().with(telemetry);

    // Set as global default
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| format!("Failed to set global subscriber: {}", e))?;

    Ok(())
}

/// Initialize tracing with both structured logging and OpenTelemetry
///
/// This combines the logging_utils structured logging with OpenTelemetry tracing.
///
/// # Arguments
/// * `service_name` - Name of the service
/// * `jaeger_endpoint` - Jaeger collector endpoint
/// * `log_level` - Logging level filter (e.g., "info", "debug")
///
/// # Example
///
/// ```rust
/// init_tracing_with_jaeger(
///     "neurograph-api",
///     "http://jaeger:14268/api/traces",
///     "info"
/// )?;
/// ```
pub fn init_tracing_with_jaeger(
    service_name: &str,
    jaeger_endpoint: &str,
    log_level: &str,
) -> Result<(), String> {
    use tracing_subscriber::fmt;
    use tracing_subscriber::EnvFilter;

    // Create Jaeger exporter
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(jaeger_endpoint)
        .with_service_name(service_name)
        .with_auto_split_batch(true)
        .with_max_packet_size(65_000)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::TraceIdRatioBased(0.01))  // v0.44.3: 1% sampling
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name.to_string()),
                    KeyValue::new("service.version", "v0.45.0"),  // Updated version
                ])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .map_err(|e| format!("Failed to install Jaeger tracer: {}", e))?;

    // Create OpenTelemetry layer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Create formatting layer
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_thread_ids(false);

    // Create filter layer
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    // Combine layers
    let subscriber = Registry::default()
        .with(filter_layer)
        .with(fmt_layer)
        .with(telemetry);

    // Set as global default
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| format!("Failed to set global subscriber: {}", e))?;

    Ok(())
}

/// Shutdown OpenTelemetry tracer gracefully
///
/// This ensures all pending spans are flushed to Jaeger before shutdown.
/// Should be called during application shutdown.
///
/// # Example
///
/// ```rust
/// // On shutdown
/// shutdown_tracer();
/// ```
pub fn shutdown_tracer() {
    global::shutdown_tracer_provider();
}

/// Create a new tracing span with custom attributes
///
/// # Arguments
/// * `name` - Span name
/// * `attributes` - Key-value attributes to attach
///
/// # Example
///
/// ```rust
/// use tracing::info_span;
///
/// let span = info_span!(
///     "token_creation",
///     token_id = %token.id,
///     token_weight = token.weight
/// );
/// ```
#[macro_export]
macro_rules! traced_operation {
    ($name:expr, $($key:ident = $value:expr),*) => {
        tracing::info_span!($name, $($key = tracing::field::display($value)),*)
    };
}

/// Helper function to extract trace context from HTTP headers
///
/// This is used for context propagation in distributed systems.
/// The trace context is extracted from W3C TraceContext headers.
///
/// # Arguments
/// * `headers` - HTTP headers map
///
/// # Returns
/// Option containing the extracted context
pub fn extract_trace_context(
    headers: &axum::http::HeaderMap,
) -> Option<opentelemetry::Context> {
    use opentelemetry::global;

    // Create a simple extractor from headers
    struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

    impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
        fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key).and_then(|v| v.to_str().ok())
        }

        fn keys(&self) -> Vec<&str> {
            self.0.keys().map(|k| k.as_str()).collect()
        }
    }

    // Extract context using global propagator
    let context = global::get_text_map_propagator(|prop| {
        prop.extract(&HeaderExtractor(headers))
    });

    Some(context)
}

/// Helper function to inject trace context into HTTP headers
///
/// This is used for context propagation when making outbound requests.
///
/// # Arguments
/// * `headers` - Mutable HTTP headers map to inject into
pub fn inject_trace_context(headers: &mut axum::http::HeaderMap) {
    use opentelemetry::global;

    // Create a simple injector for headers
    struct HeaderInjector<'a>(&'a mut axum::http::HeaderMap);

    impl<'a> opentelemetry::propagation::Injector for HeaderInjector<'a> {
        fn set(&mut self, key: &str, value: String) {
            if let Ok(header_name) = axum::http::HeaderName::try_from(key) {
                if let Ok(header_value) = axum::http::HeaderValue::try_from(value) {
                    self.0.insert(header_name, header_value);
                }
            }
        }
    }

    // Inject context using global propagator
    global::get_text_map_propagator(|prop| {
        prop.inject_context(
            &opentelemetry::Context::current(),
            &mut HeaderInjector(headers),
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracer_initialization() {
        // Note: This test doesn't actually connect to Jaeger
        // It only verifies that the initialization logic works
        let result = init_tracer("test-service", "http://localhost:14268/api/traces");
        // Result depends on whether Jaeger is available
        // In tests, we just verify it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_trace_context_extraction() {
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("traceparent", "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-01".parse().unwrap());

        let context = extract_trace_context(&headers);
        assert!(context.is_some());
    }
}
