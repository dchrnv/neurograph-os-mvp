use std::collections::HashMap;

/// Реестр типов событий — строки маппятся в u32 ID
///
/// Обеспечивает:
/// - Эффективное хранение (ID вместо строк)
/// - Быструю проверку wildcard паттернов
/// - Динамическую регистрацию новых типов
pub struct EventTypeRegistry {
    /// Строка → ID
    type_to_id: HashMap<String, u32>,

    /// ID → Строка
    id_to_type: Vec<String>,

    /// Предкомпилированные wildcard паттерны для быстрого matching
    wildcard_cache: HashMap<String, Vec<u32>>,
}

impl EventTypeRegistry {
    /// Инициализация с базовыми типами из таксономии
    pub fn new() -> Self {
        let mut registry = Self {
            type_to_id: HashMap::new(),
            id_to_type: Vec::new(),
            wildcard_cache: HashMap::new(),
        };

        // Регистрируем базовые типы из таксономии Signal Gateway v2.0

        // Input events - External
        registry.register("signal.input.external.text.chat");
        registry.register("signal.input.external.text.command");
        registry.register("signal.input.external.text.query");
        registry.register("signal.input.external.text.transcription");

        registry.register("signal.input.external.audio.speech");
        registry.register("signal.input.external.audio.music");
        registry.register("signal.input.external.audio.ambient");
        registry.register("signal.input.external.audio.alert");

        registry.register("signal.input.external.vision.object");
        registry.register("signal.input.external.vision.face");
        registry.register("signal.input.external.vision.gesture");
        registry.register("signal.input.external.vision.scene");

        registry.register("signal.input.external.environment.temperature");
        registry.register("signal.input.external.environment.humidity");
        registry.register("signal.input.external.environment.pressure");
        registry.register("signal.input.external.environment.light");

        // Input events - Internal
        registry.register("signal.input.internal.thought");
        registry.register("signal.input.internal.memory_recall");
        registry.register("signal.input.internal.emotion");
        registry.register("signal.input.internal.prediction");
        registry.register("signal.input.internal.hypothesis");

        // Input events - System
        registry.register("signal.input.system.resource.memory_low");
        registry.register("signal.input.system.resource.memory_critical");
        registry.register("signal.input.system.resource.cpu_high");
        registry.register("signal.input.system.lifecycle.startup");
        registry.register("signal.input.system.lifecycle.shutdown");
        registry.register("signal.input.system.timer.tick");
        registry.register("signal.input.system.timer.heartbeat");

        // Activation events
        registry.register("signal.activation.resonance");
        registry.register("signal.activation.action_trigger");
        registry.register("signal.activation.memory_association");
        registry.register("signal.activation.pattern_match");
        registry.register("signal.activation.cascade");

        // Anomaly events
        registry.register("signal.anomaly.novelty");
        registry.register("signal.anomaly.contradiction");
        registry.register("signal.anomaly.outlier");
        registry.register("signal.anomaly.unexpected");

        // Meta events
        registry.register("signal.meta.subscription.added");
        registry.register("signal.meta.subscription.removed");
        registry.register("signal.meta.sensor.registered");
        registry.register("signal.meta.sensor.unregistered");

        registry
    }

    /// Регистрирует новый тип события, возвращает ID
    pub fn register(&mut self, event_type: &str) -> u32 {
        // Если уже зарегистрирован - возвращаем существующий ID
        if let Some(&id) = self.type_to_id.get(event_type) {
            return id;
        }

        // Создаём новый ID
        let id = self.id_to_type.len() as u32;
        self.type_to_id.insert(event_type.to_string(), id);
        self.id_to_type.push(event_type.to_string());

        id
    }

    /// Получает ID по строке
    pub fn get_id(&self, event_type: &str) -> Option<u32> {
        self.type_to_id.get(event_type).copied()
    }

    /// Получает строку по ID
    pub fn get_type(&self, id: u32) -> Option<&str> {
        self.id_to_type.get(id as usize).map(|s| s.as_str())
    }

    /// Проверяет совпадение с wildcard паттерном
    ///
    /// Поддерживаемые паттерны:
    /// - "exact.match" - точное совпадение
    /// - "prefix.*" - все типы начинающиеся с prefix.
    /// - "prefix.*.suffix" - префикс + любое в середине + суффикс
    pub fn matches_wildcard(&self, id: u32, pattern: &str) -> bool {
        if let Some(type_str) = self.get_type(id) {
            self.match_pattern(type_str, pattern)
        } else {
            false
        }
    }

    /// Предкомпилирует wildcard паттерн в набор ID
    ///
    /// Используется при создании подписки для ускорения проверок
    pub fn compile_wildcard(&mut self, pattern: &str) -> Vec<u32> {
        // Проверяем cache
        if let Some(ids) = self.wildcard_cache.get(pattern) {
            return ids.clone();
        }

        // Компилируем паттерн
        let mut matching_ids = Vec::new();

        for (id, type_str) in self.id_to_type.iter().enumerate() {
            if self.match_pattern(type_str, pattern) {
                matching_ids.push(id as u32);
            }
        }

        // Сохраняем в cache
        self.wildcard_cache
            .insert(pattern.to_string(), matching_ids.clone());

        matching_ids
    }

    /// Внутренняя логика matching паттернов
    fn match_pattern(&self, type_str: &str, pattern: &str) -> bool {
        // Точное совпадение
        if !pattern.contains('*') {
            return type_str == pattern;
        }

        // Wildcard: "prefix.*"
        if pattern.ends_with(".*") {
            let prefix = &pattern[..pattern.len() - 2];
            return type_str.starts_with(prefix);
        }

        // Wildcard: "prefix*"
        if pattern.ends_with('*') && !pattern.ends_with(".*") {
            let prefix = &pattern[..pattern.len() - 1];
            return type_str.starts_with(prefix);
        }

        // Сложные паттерны (TODO: полноценный glob matching)
        // Пока используем простую логику

        false
    }

    /// Возвращает количество зарегистрированных типов
    pub fn count(&self) -> usize {
        self.id_to_type.len()
    }

    /// Возвращает все зарегистрированные типы
    pub fn list_all(&self) -> Vec<String> {
        self.id_to_type.clone()
    }

    /// Очищает cache wildcard паттернов
    pub fn clear_wildcard_cache(&mut self) {
        self.wildcard_cache.clear();
    }
}

impl Default for EventTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_basic() {
        let mut registry = EventTypeRegistry::new();

        // Регистрация нового типа
        let id = registry.register("test.event.custom");
        assert!(id > 0);

        // Получение по ID
        assert_eq!(registry.get_type(id), Some("test.event.custom"));

        // Получение ID по строке
        assert_eq!(registry.get_id("test.event.custom"), Some(id));
    }

    #[test]
    fn test_registry_idempotent() {
        let mut registry = EventTypeRegistry::new();

        let id1 = registry.register("test.event");
        let id2 = registry.register("test.event");

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_wildcard_exact_match() {
        let registry = EventTypeRegistry::new();

        let id = registry
            .get_id("signal.input.external.text.chat")
            .unwrap();
        assert!(registry.matches_wildcard(id, "signal.input.external.text.chat"));
        assert!(!registry.matches_wildcard(id, "signal.input.external.text.command"));
    }

    #[test]
    fn test_wildcard_prefix_match() {
        let registry = EventTypeRegistry::new();

        let id = registry
            .get_id("signal.input.external.text.chat")
            .unwrap();

        assert!(registry.matches_wildcard(id, "signal.input.external.*"));
        assert!(registry.matches_wildcard(id, "signal.input.*"));
        assert!(registry.matches_wildcard(id, "signal.*"));
        assert!(!registry.matches_wildcard(id, "signal.activation.*"));
    }

    #[test]
    fn test_compile_wildcard() {
        let mut registry = EventTypeRegistry::new();

        let ids = registry.compile_wildcard("signal.input.external.text.*");
        assert!(ids.len() >= 4); // chat, command, query, transcription

        // Проверяем что все ID соответствуют паттерну
        for id in ids {
            let type_str = registry.get_type(id).unwrap();
            assert!(type_str.starts_with("signal.input.external.text."));
        }
    }

    #[test]
    fn test_wildcard_cache() {
        let mut registry = EventTypeRegistry::new();

        // Первый раз - компиляция
        let ids1 = registry.compile_wildcard("signal.input.*");

        // Второй раз - из cache
        let ids2 = registry.compile_wildcard("signal.input.*");

        assert_eq!(ids1, ids2);
    }

    #[test]
    fn test_registry_count() {
        let registry = EventTypeRegistry::new();
        assert!(registry.count() > 30); // Базовые типы
    }

    #[test]
    fn test_list_all() {
        let registry = EventTypeRegistry::new();
        let all_types = registry.list_all();

        assert!(all_types.contains(&"signal.activation.resonance".to_string()));
        assert!(all_types.contains(&"signal.anomaly.novelty".to_string()));
    }
}
