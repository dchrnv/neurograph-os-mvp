// PyO3 bindings for SignalSystem v1.1
//
// Этот модуль компилируется только когда feature "python-bindings" включен

#![cfg(feature = "python-bindings")]

use crate::signal_system::{
    SignalSystem, SignalEvent, ProcessingResult, SubscriptionFilter,
    Subscriber,
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::{PyValueError, PyRuntimeError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Python wrapper для SignalSystem
#[pyclass(name = "SignalSystem")]
pub struct PySignalSystem {
    /// Внутренний Rust SignalSystem
    inner: Arc<SignalSystem>,

    /// Зарегистрированные Python callbacks
    /// Mutex для thread-safe доступа из Rust threads
    callbacks: Arc<Mutex<HashMap<u64, PyObject>>>,
}

#[pymethods]
impl PySignalSystem {
    /// Создаёт новый SignalSystem
    ///
    /// Example:
    ///     system = SignalSystem()
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SignalSystem::new()),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Эмитит сигнал в систему
    ///
    /// Args:
    ///     event_type: str - тип события (например "signal.input.text")
    ///     vector: List[float] - 8D вектор
    ///     priority: int - приоритет (0-255)
    ///     **kwargs: дополнительные поля события
    ///
    /// Returns:
    ///     dict - результат обработки
    ///
    /// Example:
    ///     result = system.emit(
    ///         event_type="signal.input.text",
    ///         vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    ///         priority=200
    ///     )
    #[pyo3(signature = (event_type, vector, priority=128, **kwargs))]
    pub fn emit(
        &self,
        py: Python<'_>,
        event_type: &str,
        vector: Vec<f32>,
        priority: u8,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<PyObject> {
        // Валидация вектора
        if vector.len() != 8 {
            return Err(PyValueError::new_err(format!(
                "Vector must have 8 dimensions, got {}",
                vector.len()
            )));
        }

        // Получаем event_type_id
        let event_type_id = {
            let mut registry = self.inner.event_registry().write();
            registry.register(event_type)
        };

        // Создаём SignalEvent
        let mut event = SignalEvent::default();
        event.event_type_id = event_type_id;
        event.vector = vector.try_into().unwrap();
        event.priority = priority;

        // Применяем дополнительные поля из kwargs
        if let Some(kw) = kwargs {
            self.apply_kwargs_to_event(&mut event, kw)?;
        }

        // Отпускаем GIL на время обработки
        let result = py.allow_threads(|| self.inner.emit(event));

        // Конвертируем результат в Python dict
        self.result_to_py(py, &result)
    }

    /// Подписаться на события
    ///
    /// Args:
    ///     name: str - имя подписчика
    ///     filter_dict: dict - фильтр событий в JSON формате
    ///     callback: Optional[Callable] - Python функция для обработки событий
    ///
    /// Returns:
    ///     int - subscriber_id
    ///
    /// Example:
    ///     def handler(event):
    ///         print(f"Got event: {event}")
    ///
    ///     sub_id = system.subscribe(
    ///         name="my_handler",
    ///         filter_dict={"event_type": {"$wildcard": "signal.input.*"}},
    ///         callback=handler
    ///     )
    #[pyo3(signature = (name, filter_dict, callback=None))]
    pub fn subscribe(
        &self,
        _py: Python<'_>,
        name: &str,
        filter_dict: &Bound<'_, PyDict>,
        callback: Option<PyObject>,
    ) -> PyResult<u64> {
        // Компилируем фильтр из Python dict
        let filter = self.compile_filter_from_dict(filter_dict)?;

        let subscriber_id = self.inner.next_subscriber_id();

        let subscriber = if let Some(cb) = callback {
            // Регистрируем Python callback
            let callback_id = subscriber_id; // Используем subscriber_id как callback_id
            self.callbacks.lock().unwrap().insert(callback_id, cb);

            Subscriber::new_python_callback(subscriber_id, name.to_string(), filter, callback_id)
        } else {
            // Polling mode (TODO: implement poll() method)
            let (subscriber, _receiver) = Subscriber::new_polling(subscriber_id, name.to_string(), filter);
            subscriber
        };

        self.inner
            .subscribe(subscriber)
            .map_err(|e| PyRuntimeError::new_err(format!("Subscribe failed: {}", e)))?;

        Ok(subscriber_id)
    }

    /// Отписаться от событий
    ///
    /// Args:
    ///     subscriber_id: int
    ///
    /// Example:
    ///     system.unsubscribe(sub_id)
    pub fn unsubscribe(&self, subscriber_id: u64) -> PyResult<()> {
        // Удаляем callback если был
        self.callbacks.lock().unwrap().remove(&subscriber_id);

        self.inner
            .unsubscribe(subscriber_id)
            .map_err(|e| PyRuntimeError::new_err(format!("Unsubscribe failed: {}", e)))?;

        Ok(())
    }

    /// Получить статистику работы системы
    ///
    /// Returns:
    ///     dict - статистика
    ///
    /// Example:
    ///     stats = system.get_stats()
    ///     print(f"Total events: {stats['total_events']}")
    pub fn get_stats(&self, py: Python<'_>) -> PyResult<PyObject> {
        let stats = self.inner.get_stats();

        let dict = PyDict::new_bound(py);
        dict.set_item("total_events", stats.total_events)?;
        dict.set_item("avg_processing_time_us", stats.avg_processing_time_us)?;
        dict.set_item("subscriber_notifications", stats.subscriber_notifications)?;
        dict.set_item("filter_matches", stats.filter_matches)?;
        dict.set_item("filter_misses", stats.filter_misses)?;

        // events_by_type как dict
        let events_by_type = PyDict::new_bound(py);
        for (type_id, count) in &stats.events_by_type {
            events_by_type.set_item(type_id, count)?;
        }
        dict.set_item("events_by_type", events_by_type)?;

        Ok(dict.into())
    }

    /// Сбросить статистику
    pub fn reset_stats(&self) {
        self.inner.reset_stats();
    }

    /// Получить количество активных подписчиков
    pub fn subscriber_count(&self) -> usize {
        self.inner.subscriber_count()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER METHODS
// ═══════════════════════════════════════════════════════════════════════════════

impl PySignalSystem {
    /// Применить kwargs к SignalEvent
    fn apply_kwargs_to_event(&self, event: &mut SignalEvent, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        // confidence
        if let Some(v) = kwargs.get_item("confidence")? {
            event.confidence = v.extract::<u8>()?;
        }

        // urgency
        if let Some(v) = kwargs.get_item("urgency")? {
            event.urgency = v.extract::<u8>()?;
        }

        // magnitude
        if let Some(v) = kwargs.get_item("magnitude")? {
            event.magnitude = v.extract::<i16>()?;
        }

        // layers (8D)
        if let Some(v) = kwargs.get_item("layers")? {
            let layers: Vec<f32> = v.extract()?;
            if layers.len() == 8 {
                event.layers = layers.try_into().unwrap();
            }
        }

        Ok(())
    }

    /// Конвертировать ProcessingResult в Python dict
    fn result_to_py(&self, py: Python<'_>, result: &ProcessingResult) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);

        dict.set_item("token_id", result.token_id)?;
        dict.set_item("energy_delta", result.energy_delta)?;
        dict.set_item("activation_spread", result.activation_spread)?;
        dict.set_item("is_novel", result.is_novel)?;
        dict.set_item("anomaly_score", result.anomaly_score)?;
        dict.set_item("processing_time_us", result.processing_time_us)?;

        // neighbors
        let neighbors = PyList::empty_bound(py);
        for n in &result.neighbors {
            let neighbor_dict = PyDict::new_bound(py);
            neighbor_dict.set_item("token_id", n.token_id)?;
            neighbor_dict.set_item("distance", n.distance)?;
            neighbor_dict.set_item("resonance", n.resonance)?;
            neighbor_dict.set_item("token_type", n.token_type)?;
            neighbor_dict.set_item("layer_affinity", n.layer_affinity)?;
            neighbors.append(neighbor_dict)?;
        }
        dict.set_item("neighbors", neighbors)?;

        // triggered_actions
        let actions = PyList::new_bound(py, &result.triggered_actions);
        dict.set_item("triggered_actions", actions)?;

        Ok(dict.into())
    }

    /// Компилировать фильтр из Python dict
    fn compile_filter_from_dict(&self, filter_dict: &Bound<'_, PyDict>) -> PyResult<SubscriptionFilter> {
        // Конвертируем PyDict в serde_json::Value через repr
        use pyo3::types::PyAnyMethods;
        let repr_str = filter_dict.repr()?.to_string();

        // Parse Python dict repr as JSON (rough conversion)
        // Better: use pythonize crate, but for now simple approach
        let json_str = repr_str.replace("'", "\"");
        let json_value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| PyValueError::new_err(format!("Invalid filter JSON: {}. Dict was: {}", e, json_str)))?;

        // Компилируем фильтр
        let mut registry = self.inner.event_registry().write();
        let filter = SubscriptionFilter::compile(1, &json_value, &mut registry)
            .map_err(|e| PyValueError::new_err(format!("Filter compilation failed: {}", e)))?;

        Ok(filter)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_signal_system_new() {
        let system = PySignalSystem::new();
        assert_eq!(system.subscriber_count(), 0);
    }

    // Note: Full Python integration tests require Python runtime
    // These will be tested in integration tests with pytest
}
