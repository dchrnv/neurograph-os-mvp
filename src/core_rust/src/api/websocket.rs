// NeuroGraph OS - WebSocket Support v0.39.0
//
// Real-time bidirectional communication

use super::models::*;
use super::state::ApiState;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

// ============================================================================
// WebSocket Message Types
// ============================================================================

/// Client -> Server messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Subscribe to events
    Subscribe { topics: Vec<String> },

    /// Unsubscribe from events
    Unsubscribe { topics: Vec<String> },

    /// Send query
    Query { query: QueryRequest },

    /// Send feedback
    Feedback { feedback: FeedbackRequest },

    /// Ping
    Ping,
}

/// Server -> Client messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Query response
    QueryResponse { data: QueryResponse },

    /// Feedback response
    FeedbackResponse { data: FeedbackResponse },

    /// Event notification
    Event { topic: String, data: serde_json::Value },

    /// Error
    Error { error: ErrorResponse },

    /// Pong
    Pong,
}

// ============================================================================
// WebSocket Handler
// ============================================================================

/// WebSocket upgrade handler
pub async fn handle_websocket(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

/// Handle WebSocket connection
async fn websocket_connection(socket: WebSocket, state: ApiState) {
    let (mut sender, mut receiver) = socket.split();

    // Create broadcast channel for events
    let (tx, mut rx) = broadcast::channel::<ServerMessage>(100);

    // Spawn task to forward broadcast messages to WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Parse client message
                    let client_msg: Result<ClientMessage, _> = serde_json::from_str(&text);

                    match client_msg {
                        Ok(ClientMessage::Query { query }) => {
                            // Handle query (simplified - full implementation would need async handling)
                            let response = ServerMessage::QueryResponse {
                                data: QueryResponse {
                                    signal_id: 0, // Would come from actual processing
                                    state: [0.0; 8],
                                    signal_type: "query".to_string(),
                                    response: Some("WebSocket query received".to_string()),
                                    metadata: QueryMetadata {
                                        processing_time_us: 0,
                                        matched_tokens: 0,
                                        unknown_words: 0,
                                        decision_source: None,
                                        confidence: None,
                                    },
                                },
                            };

                            let _ = tx.send(response);
                        }
                        Ok(ClientMessage::Ping) => {
                            let _ = tx.send(ServerMessage::Pong);
                        }
                        Ok(ClientMessage::Subscribe { topics: _ }) => {
                            // TODO: Implement subscription logic
                        }
                        Ok(ClientMessage::Unsubscribe { topics: _ }) => {
                            // TODO: Implement unsubscription logic
                        }
                        Ok(ClientMessage::Feedback { feedback: _ }) => {
                            // TODO: Handle feedback
                        }
                        Err(e) => {
                            let error_msg = ServerMessage::Error {
                                error: ErrorResponse::new(
                                    "parse_error",
                                    format!("Failed to parse message: {}", e),
                                ),
                            };
                            let _ = tx.send(error_msg);
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

// ============================================================================
// Event Broadcasting (for future use)
// ============================================================================

/// Event topics
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventTopic {
    /// Curiosity exploration events
    Exploration,
    /// Feedback events
    Feedback,
    /// System status changes
    Status,
}

impl EventTopic {
    pub fn as_str(&self) -> &str {
        match self {
            EventTopic::Exploration => "exploration",
            EventTopic::Feedback => "feedback",
            EventTopic::Status => "status",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_serialization() {
        let msg = ClientMessage::Ping;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"ping\""));

        let query_msg = ClientMessage::Query {
            query: QueryRequest {
                query: "test".to_string(),
                context: std::collections::HashMap::new(),
                timeout_ms: None,
            },
        };
        let json = serde_json::to_string(&query_msg).unwrap();
        assert!(json.contains("\"type\":\"query\""));
    }

    #[test]
    fn test_server_message_serialization() {
        let msg = ServerMessage::Pong;
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"pong\""));
    }
}
