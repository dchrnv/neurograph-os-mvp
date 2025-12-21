use crate::signal_system::{SubscriptionFilter, SignalEvent, ProcessingResult};
use std::sync::Arc;

/// ID подписчика
pub type SubscriberId = u64;

/// Подписчик на события SignalSystem
#[derive(Debug, Clone)]
pub struct Subscriber {
    /// Уникальный ID подписчика
    pub id: SubscriberId,

    /// Человекочитаемое имя (для отладки)
    pub name: String,

    /// Фильтр для отбора событий
    pub filter: SubscriptionFilter,

    /// Тип callback для доставки событий
    pub callback_type: CallbackType,
}

/// Способ доставки событий подписчику
#[derive(Clone)]
pub enum CallbackType {
    /// Polling: события накапливаются в очереди, подписчик забирает их сам
    Polling {
        sender: crossbeam_channel::Sender<ProcessedEvent>,
    },

    /// Push через channel: события отправляются немедленно
    Channel {
        sender: crossbeam_channel::Sender<ProcessedEvent>,
    },

    /// Python callback (для PyO3 bindings)
    /// Хранит ID зарегистрированного callback в глобальном реестре
    PythonCallback {
        callback_id: u64,
    },

    /// Rust closure (для тестов и внутреннего использования)
    /// Arc для Clone support
    RustCallback {
        callback: Arc<dyn Fn(ProcessedEvent) + Send + Sync>,
    },
}

impl std::fmt::Debug for CallbackType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallbackType::Polling { .. } => write!(f, "CallbackType::Polling"),
            CallbackType::Channel { .. } => write!(f, "CallbackType::Channel"),
            CallbackType::PythonCallback { callback_id } => {
                write!(f, "CallbackType::PythonCallback({})", callback_id)
            }
            CallbackType::RustCallback { .. } => write!(f, "CallbackType::RustCallback"),
        }
    }
}

/// Обработанное событие, готовое для доставки подписчику
#[derive(Debug, Clone)]
pub struct ProcessedEvent {
    /// Исходное событие
    pub event: SignalEvent,

    /// Результат обработки (если была обработка)
    pub result: Option<ProcessingResult>,

    /// Метаданные доставки
    pub delivery_meta: DeliveryMeta,
}

/// Метаданные доставки события
#[derive(Debug, Clone)]
pub struct DeliveryMeta {
    /// Timestamp доставки (микросекунды с Unix epoch)
    pub delivered_at_us: u64,

    /// Задержка от emit до доставки (микросекунды)
    pub latency_us: u64,

    /// ID подписчика-получателя
    pub subscriber_id: SubscriberId,

    /// Сколько подписчиков получили это событие
    pub total_recipients: u32,
}

impl Subscriber {
    /// Создаёт нового подписчика с Polling callback
    pub fn new_polling(id: SubscriberId, name: String, filter: SubscriptionFilter) -> (Self, crossbeam_channel::Receiver<ProcessedEvent>) {
        let (sender, receiver) = crossbeam_channel::unbounded();

        let subscriber = Self {
            id,
            name,
            filter,
            callback_type: CallbackType::Polling { sender },
        };

        (subscriber, receiver)
    }

    /// Создаёт нового подписчика с Channel callback
    pub fn new_channel(
        id: SubscriberId,
        name: String,
        filter: SubscriptionFilter,
        sender: crossbeam_channel::Sender<ProcessedEvent>,
    ) -> Self {
        Self {
            id,
            name,
            filter,
            callback_type: CallbackType::Channel { sender },
        }
    }

    /// Создаёт нового подписчика с Python callback
    pub fn new_python_callback(
        id: SubscriberId,
        name: String,
        filter: SubscriptionFilter,
        callback_id: u64,
    ) -> Self {
        Self {
            id,
            name,
            filter,
            callback_type: CallbackType::PythonCallback { callback_id },
        }
    }

    /// Создаёт нового подписчика с Rust closure
    pub fn new_rust_callback<F>(
        id: SubscriberId,
        name: String,
        filter: SubscriptionFilter,
        callback: F,
    ) -> Self
    where
        F: Fn(ProcessedEvent) + Send + Sync + 'static,
    {
        Self {
            id,
            name,
            filter,
            callback_type: CallbackType::RustCallback {
                callback: Arc::new(callback),
            },
        }
    }

    /// Доставляет событие подписчику
    pub fn deliver(&self, event: ProcessedEvent) -> Result<(), SubscriberError> {
        match &self.callback_type {
            CallbackType::Polling { sender } | CallbackType::Channel { sender } => {
                sender
                    .send(event)
                    .map_err(|_| SubscriberError::ChannelClosed(self.id))?;
            }

            CallbackType::PythonCallback { callback_id } => {
                // Для PyO3 - вызов будет в bindings слое
                // Здесь только placeholder для проверки типа
                return Err(SubscriberError::PythonCallbackNotImplemented(*callback_id));
            }

            CallbackType::RustCallback { callback } => {
                callback(event);
            }
        }

        Ok(())
    }
}

/// Ошибки работы с подписчиками
#[derive(Debug, Clone, thiserror::Error)]
pub enum SubscriberError {
    #[error("Subscriber {0} channel closed")]
    ChannelClosed(SubscriberId),

    #[error("Python callback {0} not implemented at core level")]
    PythonCallbackNotImplemented(u64),

    #[error("Subscriber {0} not found")]
    NotFound(SubscriberId),
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_system::{EventTypeRegistry, FilterCondition, FilterLogic};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_subscriber_new_polling() {
        let mut registry = EventTypeRegistry::new();
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(
                crate::signal_system::filter::EventTypeCondition::new(
                    "signal.test".to_string(),
                    &mut registry,
                ),
            ),
        );

        let (subscriber, _receiver) = Subscriber::new_polling(1, "test_sub".to_string(), filter);

        assert_eq!(subscriber.id, 1);
        assert_eq!(subscriber.name, "test_sub");
        assert!(matches!(subscriber.callback_type, CallbackType::Polling { .. }));
    }

    #[test]
    fn test_subscriber_deliver_polling() {
        let mut registry = EventTypeRegistry::new();
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(
                crate::signal_system::filter::EventTypeCondition::new(
                    "signal.test".to_string(),
                    &mut registry,
                ),
            ),
        );

        let (subscriber, receiver) = Subscriber::new_polling(1, "test_sub".to_string(), filter);

        let event = ProcessedEvent {
            event: SignalEvent::default(),
            result: None,
            delivery_meta: DeliveryMeta {
                delivered_at_us: 1000,
                latency_us: 100,
                subscriber_id: 1,
                total_recipients: 1,
            },
        };

        subscriber.deliver(event.clone()).unwrap();

        let received = receiver.try_recv().unwrap();
        assert_eq!(received.delivery_meta.subscriber_id, 1);
    }

    #[test]
    fn test_subscriber_rust_callback() {
        let mut registry = EventTypeRegistry::new();
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(
                crate::signal_system::filter::EventTypeCondition::new(
                    "signal.test".to_string(),
                    &mut registry,
                ),
            ),
        );

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let subscriber = Subscriber::new_rust_callback(
            1,
            "callback_sub".to_string(),
            filter,
            move |_event| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            },
        );

        let event = ProcessedEvent {
            event: SignalEvent::default(),
            result: None,
            delivery_meta: DeliveryMeta {
                delivered_at_us: 1000,
                latency_us: 100,
                subscriber_id: 1,
                total_recipients: 1,
            },
        };

        subscriber.deliver(event.clone()).unwrap();
        subscriber.deliver(event.clone()).unwrap();
        subscriber.deliver(event).unwrap();

        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn test_subscriber_channel_closed() {
        let mut registry = EventTypeRegistry::new();
        let filter = SubscriptionFilter::new(
            1,
            FilterCondition::EventType(
                crate::signal_system::filter::EventTypeCondition::new(
                    "signal.test".to_string(),
                    &mut registry,
                ),
            ),
        );

        let (sender, receiver) = crossbeam_channel::unbounded();
        drop(receiver); // Закрываем receiver

        let subscriber = Subscriber::new_channel(1, "test_sub".to_string(), filter, sender);

        let event = ProcessedEvent {
            event: SignalEvent::default(),
            result: None,
            delivery_meta: DeliveryMeta {
                delivered_at_us: 1000,
                latency_us: 100,
                subscriber_id: 1,
                total_recipients: 1,
            },
        };

        let result = subscriber.deliver(event);
        assert!(matches!(result, Err(SubscriberError::ChannelClosed(1))));
    }
}
