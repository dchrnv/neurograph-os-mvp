// Signal System v1.1 - Event Processing with Subscriptions
//
// Компоненты:
// - event: SignalEvent структура (256 bytes)
// - registry: EventTypeRegistry (string ↔ ID mapping)
// - result: ProcessingResult (результат обработки)
// - filter: SubscriptionFilter (фильтры подписок)
// - system: SignalSystem (основная логика)
// - subscriber: Subscriber management

pub mod event;
pub mod registry;
pub mod result;

// Re-exports
pub use event::{SignalEvent, SignalSource, SemanticCore, EnergyProfile, TemporalBinding, RoutingInfo};
pub use registry::EventTypeRegistry;
pub use result::{ProcessingResult, NeighborInfo};
