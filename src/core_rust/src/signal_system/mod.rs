// Signal System v1.1 - Event Processing with Subscriptions
//
// Компоненты:
// - event: SignalEvent структура (256 bytes)
// - registry: EventTypeRegistry (string ↔ ID mapping)
// - result: ProcessingResult (результат обработки)
// - filter: SubscriptionFilter (фильтры подписок)
// - system: SignalSystem (основная логика)
// - subscriber: Subscriber management
// - py_bindings: PyO3 bindings для Python (optional)

pub mod event;
pub mod registry;
pub mod result;
pub mod filter;
pub mod subscriber;
pub mod system;

#[cfg(feature = "python-bindings")]
pub mod py_bindings;

// Re-exports
pub use event::{SignalEvent, SignalSource, SemanticCore, EnergyProfile, TemporalBinding, RoutingInfo};
pub use registry::EventTypeRegistry;
pub use result::{ProcessingResult, NeighborInfo};
pub use filter::{SubscriptionFilter, FilterCondition, FilterLogic, FilterError};
pub use subscriber::{Subscriber, SubscriberId, CallbackType, ProcessedEvent, DeliveryMeta, SubscriberError};
pub use system::{SignalSystem, SignalSystemConfig, SignalSystemStats, SignalSystemError};

#[cfg(feature = "python-bindings")]
pub use py_bindings::PySignalSystem;
