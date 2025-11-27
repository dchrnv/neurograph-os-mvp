// NeuroGraph OS - REST API Module v0.39.0
//
// HTTP REST API for external access to NeuroGraph OS

pub mod models;
pub mod state;
pub mod handlers;
pub mod router;
pub mod websocket;

// Re-export key types
pub use models::{
    QueryRequest, QueryResponse, QueryMetadata,
    FeedbackRequest, FeedbackResponse, FeedbackType,
    StatusResponse, StatsResponse,
    HealthResponse, ErrorResponse,
};

pub use state::{ApiState, ApiConfig};
pub use router::create_router;
pub use websocket::handle_websocket;
