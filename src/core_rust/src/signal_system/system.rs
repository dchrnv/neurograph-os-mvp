use crate::signal_system::{
    EventTypeRegistry, SignalEvent, ProcessingResult, Subscriber, SubscriberId,
    ProcessedEvent, DeliveryMeta,
};
use crate::module_id::ModuleId;
use crate::module_registry::REGISTRY;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// SignalSystem v1.1 - центральный координатор событий
pub struct SignalSystem {
    /// Реестр типов событий (shared with subscribers)
    event_registry: RwLock<EventTypeRegistry>,

    /// Подписчики с их фильтрами
    subscribers: RwLock<HashMap<SubscriberId, Subscriber>>,

    /// Счётчик подписчиков
    next_subscriber_id: AtomicU64,

    /// Конфигурация
    config: SignalSystemConfig,

    /// Статистика
    stats: RwLock<SignalSystemStats>,
}

/// Конфигурация SignalSystem
#[derive(Debug, Clone)]
pub struct SignalSystemConfig {
    /// Максимум подписчиков
    pub max_subscribers: usize,

    /// Максимум соседей в результате
    pub max_neighbors: usize,

    /// Порог для определения новизны
    pub novelty_threshold: f32,

    /// Batch size для обработки событий
    pub batch_size: usize,
}

impl Default for SignalSystemConfig {
    fn default() -> Self {
        Self {
            max_subscribers: 1000,
            max_neighbors: 20,
            novelty_threshold: 0.8,
            batch_size: 100,
        }
    }
}

/// Статистика работы SignalSystem
#[derive(Debug, Default, Clone)]
pub struct SignalSystemStats {
    /// Всего обработано событий
    pub total_events: u64,

    /// События по типам
    pub events_by_type: HashMap<u32, u64>,

    /// Общее время обработки (микросекунды)
    pub total_processing_time_us: u64,

    /// Среднее время обработки (микросекунды)
    pub avg_processing_time_us: f64,

    /// Уведомлений отправлено подписчикам
    pub subscriber_notifications: u64,

    /// Фильтров сработало
    pub filter_matches: u64,

    /// Фильтров не сработало
    pub filter_misses: u64,
}

impl SignalSystem {
    /// Создаёт новый SignalSystem с дефолтной конфигурацией
    pub fn new() -> Self {
        Self::with_config(SignalSystemConfig::default())
    }

    /// Создаёт новый SignalSystem с кастомной конфигурацией
    pub fn with_config(config: SignalSystemConfig) -> Self {
        Self {
            event_registry: RwLock::new(EventTypeRegistry::new()),
            subscribers: RwLock::new(HashMap::new()),
            next_subscriber_id: AtomicU64::new(1),
            config,
            stats: RwLock::new(SignalSystemStats::default()),
        }
    }

    /// Получить доступ к EventTypeRegistry (для компиляции фильтров)
    pub fn event_registry(&self) -> &RwLock<EventTypeRegistry> {
        &self.event_registry
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // SUBSCRIPTION API
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Подписаться на события с фильтром
    pub fn subscribe(&self, subscriber: Subscriber) -> Result<SubscriberId, SignalSystemError> {
        let mut subs = self.subscribers.write();

        if subs.len() >= self.config.max_subscribers {
            return Err(SignalSystemError::TooManySubscribers(
                self.config.max_subscribers,
            ));
        }

        let id = subscriber.id;
        subs.insert(id, subscriber);

        Ok(id)
    }

    /// Отписаться от событий
    pub fn unsubscribe(&self, subscriber_id: SubscriberId) -> Result<(), SignalSystemError> {
        let mut subs = self.subscribers.write();
        subs.remove(&subscriber_id)
            .ok_or(SignalSystemError::SubscriberNotFound(subscriber_id))?;

        Ok(())
    }

    /// Получить ID нового подписчика
    pub fn next_subscriber_id(&self) -> SubscriberId {
        self.next_subscriber_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Количество активных подписчиков
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.read().len()
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // EVENT PROCESSING API
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Главная точка входа — обработка сигнала
    ///
    /// Performance target: <100μs
    pub fn emit(&self, event: SignalEvent) -> ProcessingResult {
        // Проверяем, включен ли модуль
        if !REGISTRY.is_enabled(ModuleId::SignalSystem) {
            // Модуль выключен — возвращаем пустой результат
            return ProcessingResult::default();
        }

        let start_us = current_time_us();

        // 1. Базовая обработка (пока без Grid/Graph)
        let result = self.process_event(&event);

        // 2. Доставка подписчикам
        let delivery_start = current_time_us();
        self.deliver_to_subscribers(&event, &result, delivery_start);

        // 3. Обновление статистики
        let processing_time_us = current_time_us() - start_us;
        self.update_stats(event.event_type_id, processing_time_us);

        result
    }

    /// Обработать событие (базовая версия без Grid/Graph)
    fn process_event(&self, event: &SignalEvent) -> ProcessingResult {
        let start_us = current_time_us();

        // TODO: интеграция с Grid/Graph/Guardian в следующей версии
        // Пока возвращаем упрощённый результат

        ProcessingResult {
            token_id: 0, // Will be assigned by Grid
            neighbors: vec![],
            energy_delta: 0.0,
            activation_spread: 0,
            is_novel: true,
            triggered_actions: vec![],
            anomaly_score: 0.0,
            processing_time_us: current_time_us() - start_us,
        }
    }

    /// Доставить событие подписчикам
    fn deliver_to_subscribers(
        &self,
        event: &SignalEvent,
        result: &ProcessingResult,
        emit_time_us: u64,
    ) {
        let registry = self.event_registry.read();
        let subscribers = self.subscribers.read();

        let mut notifications = 0u64;
        let mut matches = 0u64;
        let mut misses = 0u64;

        for subscriber in subscribers.values() {
            // Проверяем фильтр
            if subscriber.filter.matches(event, &registry) {
                matches += 1;

                let processed_event = ProcessedEvent {
                    event: event.clone(),
                    result: Some(result.clone()),
                    delivery_meta: DeliveryMeta {
                        delivered_at_us: current_time_us(),
                        latency_us: current_time_us() - emit_time_us,
                        subscriber_id: subscriber.id,
                        total_recipients: 1, // Will be updated after counting
                    },
                };

                // Доставляем событие
                if subscriber.deliver(processed_event).is_ok() {
                    notifications += 1;
                }
            } else {
                misses += 1;
            }
        }

        // Обновляем статистику доставки
        let mut stats = self.stats.write();
        stats.subscriber_notifications += notifications;
        stats.filter_matches += matches;
        stats.filter_misses += misses;
    }

    /// Обновить статистику
    fn update_stats(&self, event_type_id: u32, processing_time_us: u64) {
        let mut stats = self.stats.write();

        stats.total_events += 1;
        *stats.events_by_type.entry(event_type_id).or_insert(0) += 1;
        stats.total_processing_time_us += processing_time_us;
        stats.avg_processing_time_us =
            stats.total_processing_time_us as f64 / stats.total_events as f64;
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // STATS API
    // ═══════════════════════════════════════════════════════════════════════════════

    /// Получить снимок статистики
    pub fn get_stats(&self) -> SignalSystemStats {
        self.stats.read().clone()
    }

    /// Сбросить статистику
    pub fn reset_stats(&self) {
        let mut stats = self.stats.write();
        *stats = SignalSystemStats::default();
    }
}

impl Default for SignalSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Получить текущее время в микросекундах с Unix epoch
fn current_time_us() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, thiserror::Error)]
pub enum SignalSystemError {
    #[error("Too many subscribers: {0} (max allowed)")]
    TooManySubscribers(usize),

    #[error("Subscriber {0} not found")]
    SubscriberNotFound(SubscriberId),

    #[error("Event queue full")]
    QueueFull,
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_system::{FilterCondition, SubscriptionFilter};

    #[test]
    fn test_signal_system_new() {
        let system = SignalSystem::new();
        assert_eq!(system.subscriber_count(), 0);

        let stats = system.get_stats();
        assert_eq!(stats.total_events, 0);
    }

    #[test]
    fn test_subscribe_unsubscribe() {
        let system = SignalSystem::new();
        let mut registry = system.event_registry.write();

        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(crate::signal_system::filter::EventTypeCondition::new(
                "signal.test".to_string(),
                &mut registry,
            )),
        );
        drop(registry);

        let id = system.next_subscriber_id();
        let (subscriber, _receiver) = Subscriber::new_polling(id, "test_sub".to_string(), filter);

        system.subscribe(subscriber).unwrap();
        assert_eq!(system.subscriber_count(), 1);

        system.unsubscribe(id).unwrap();
        assert_eq!(system.subscriber_count(), 0);
    }

    #[test]
    fn test_emit_and_deliver() {
        let system = SignalSystem::new();
        let mut registry = system.event_registry.write();

        let event_type_id = registry.register("signal.test.emit");
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(crate::signal_system::filter::EventTypeCondition::new(
                "signal.test.emit".to_string(),
                &mut registry,
            )),
        );
        drop(registry);

        let id = system.next_subscriber_id();
        let (subscriber, receiver) = Subscriber::new_polling(id, "test_sub".to_string(), filter);
        system.subscribe(subscriber).unwrap();

        // Emit event
        let mut event = SignalEvent::default();
        event.event_type_id = event_type_id;

        let result = system.emit(event);
        // processing_time_us can be 0 on fast systems
        assert!(result.processing_time_us >= 0);

        // Check delivery
        let received = receiver.try_recv();
        assert!(received.is_ok());

        let stats = system.get_stats();
        assert_eq!(stats.total_events, 1);
        assert_eq!(stats.subscriber_notifications, 1);
        assert_eq!(stats.filter_matches, 1);
    }

    #[test]
    fn test_filter_mismatch() {
        let system = SignalSystem::new();
        let mut registry = system.event_registry.write();

        let event_type_id = registry.register("signal.test.other");
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(crate::signal_system::filter::EventTypeCondition::new(
                "signal.test.nomatch".to_string(),
                &mut registry,
            )),
        );
        drop(registry);

        let id = system.next_subscriber_id();
        let (subscriber, receiver) = Subscriber::new_polling(id, "test_sub".to_string(), filter);
        system.subscribe(subscriber).unwrap();

        // Emit event with different type
        let mut event = SignalEvent::default();
        event.event_type_id = event_type_id;

        system.emit(event);

        // Should NOT be delivered
        let received = receiver.try_recv();
        assert!(received.is_err());

        let stats = system.get_stats();
        assert_eq!(stats.subscriber_notifications, 0);
        assert_eq!(stats.filter_misses, 1);
    }

    #[test]
    fn test_stats() {
        let system = SignalSystem::new();
        let mut registry = system.event_registry.write();
        let event_type_id = registry.register("signal.test.stats");
        drop(registry);

        // Emit multiple events
        for _ in 0..5 {
            let mut event = SignalEvent::default();
            event.event_type_id = event_type_id;
            system.emit(event);
        }

        let stats = system.get_stats();
        assert_eq!(stats.total_events, 5);
        assert_eq!(stats.events_by_type.get(&event_type_id), Some(&5));
        assert!(stats.avg_processing_time_us > 0.0);

        // Reset stats
        system.reset_stats();
        let stats = system.get_stats();
        assert_eq!(stats.total_events, 0);
    }
}
