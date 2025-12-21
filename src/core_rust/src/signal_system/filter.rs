use crate::signal_system::{SignalEvent, EventTypeRegistry};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Скомпилированный фильтр подписки
///
/// Фильтры предкомпилируются при создании подписки для максимальной
/// производительности проверки (target: <1μs per event)
#[derive(Debug, Clone)]
pub struct SubscriptionFilter {
    /// ID фильтра (для отладки)
    pub id: u64,

    /// Условия (скомпилированные для быстрой проверки)
    conditions: Vec<FilterCondition>,

    /// Логический оператор между условиями: AND (default) или OR
    pub logic: FilterLogic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterLogic {
    And,
    Or,
}

/// Типы условий фильтра
#[derive(Debug, Clone)]
pub enum FilterCondition {
    /// Проверка event_type (с поддержкой wildcard)
    EventType(EventTypeCondition),

    /// Проверка числового поля
    Numeric(NumericCondition),

    /// Проверка bitmap (tags)
    Bitmap(BitmapCondition),

    /// Проверка hash (sensor_id, etc.)
    Hash(HashCondition),

    /// Комбинированное условие (AND/OR/NOT)
    Combined(Box<CombinedCondition>),
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVENT TYPE CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct EventTypeCondition {
    /// Точное совпадение или wildcard
    pub pattern: String,

    /// Предкомпилированный набор ID, если это wildcard
    pub matching_ids: Option<Vec<u32>>,
}

impl EventTypeCondition {
    pub fn new(pattern: String, registry: &mut EventTypeRegistry) -> Self {
        let matching_ids = if pattern.contains('*') {
            Some(registry.compile_wildcard(&pattern))
        } else {
            None
        };

        Self {
            pattern,
            matching_ids,
        }
    }

    #[inline]
    pub fn matches(&self, event_type_id: u32, registry: &EventTypeRegistry) -> bool {
        if let Some(ref ids) = self.matching_ids {
            // Wildcard: проверяем по предкомпилированному набору
            ids.contains(&event_type_id)
        } else {
            // Точное совпадение
            registry.get_id(&self.pattern) == Some(event_type_id)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// NUMERIC CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct NumericCondition {
    /// Какое поле проверяем
    pub field: NumericField,

    /// Оператор
    pub op: CompareOp,

    /// Значение для сравнения
    pub value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericField {
    Priority,
    Confidence,
    Urgency,
    Magnitude,
    Arousal,
    Valence,
    LayerPhysical,
    LayerSpatial,
    LayerTemporal,
    LayerCausal,
    LayerEmotional,
    LayerSocial,
    LayerAbstract,
    LayerMeta,
}

impl NumericField {
    #[inline]
    pub fn extract(&self, event: &SignalEvent) -> i64 {
        match self {
            NumericField::Priority => event.priority as i64,
            NumericField::Confidence => event.confidence as i64,
            NumericField::Urgency => event.urgency as i64,
            NumericField::Magnitude => event.magnitude as i64,
            NumericField::Arousal => event.arousal as i64,
            NumericField::Valence => event.valence as i64,
            NumericField::LayerPhysical => (event.layers[0] * 255.0) as i64,
            NumericField::LayerSpatial => (event.layers[1] * 255.0) as i64,
            NumericField::LayerTemporal => (event.layers[2] * 255.0) as i64,
            NumericField::LayerCausal => (event.layers[3] * 255.0) as i64,
            NumericField::LayerEmotional => (event.layers[4] * 255.0) as i64,
            NumericField::LayerSocial => (event.layers[5] * 255.0) as i64,
            NumericField::LayerAbstract => (event.layers[6] * 255.0) as i64,
            NumericField::LayerMeta => (event.layers[7] * 255.0) as i64,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "priority" => Some(NumericField::Priority),
            "confidence" => Some(NumericField::Confidence),
            "urgency" => Some(NumericField::Urgency),
            "magnitude" => Some(NumericField::Magnitude),
            "arousal" => Some(NumericField::Arousal),
            "valence" => Some(NumericField::Valence),
            "layer.physical" => Some(NumericField::LayerPhysical),
            "layer.spatial" => Some(NumericField::LayerSpatial),
            "layer.temporal" => Some(NumericField::LayerTemporal),
            "layer.causal" => Some(NumericField::LayerCausal),
            "layer.emotional" => Some(NumericField::LayerEmotional),
            "layer.social" => Some(NumericField::LayerSocial),
            "layer.abstract" => Some(NumericField::LayerAbstract),
            "layer.meta" => Some(NumericField::LayerMeta),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOp {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}

impl CompareOp {
    #[inline]
    pub fn compare(&self, a: i64, b: i64) -> bool {
        match self {
            CompareOp::Eq => a == b,
            CompareOp::Ne => a != b,
            CompareOp::Gt => a > b,
            CompareOp::Gte => a >= b,
            CompareOp::Lt => a < b,
            CompareOp::Lte => a <= b,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "$eq" => Some(CompareOp::Eq),
            "$ne" => Some(CompareOp::Ne),
            "$gt" => Some(CompareOp::Gt),
            "$gte" => Some(CompareOp::Gte),
            "$lt" => Some(CompareOp::Lt),
            "$lte" => Some(CompareOp::Lte),
            _ => None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BITMAP CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct BitmapCondition {
    /// Какой bitmap проверяем
    pub field: BitmapField,

    /// Маска для проверки
    pub mask: u32,

    /// Режим: contains (any bit), contains_all, not_contains
    pub mode: BitmapMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapField {
    Tags,
    Flags,
}

impl BitmapField {
    #[inline]
    pub fn extract(&self, event: &SignalEvent) -> u32 {
        match self {
            BitmapField::Tags => event.tags_bitmap,
            BitmapField::Flags => event.flags as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitmapMode {
    ContainsAny,
    ContainsAll,
    NotContains,
}

impl BitmapMode {
    #[inline]
    pub fn check(&self, value: u32, mask: u32) -> bool {
        match self {
            BitmapMode::ContainsAny => (value & mask) != 0,
            BitmapMode::ContainsAll => (value & mask) == mask,
            BitmapMode::NotContains => (value & mask) == 0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HASH CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct HashCondition {
    /// Какое поле
    pub field: HashField,

    /// Хэш для сравнения (или набор хэшей)
    pub hashes: Vec<u64>,

    /// Режим: equals, in_set, not_in_set
    pub mode: HashMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashField {
    SensorId,
    SequenceId,
    TraceId,
    ParentEvent,
}

impl HashField {
    #[inline]
    pub fn extract(&self, event: &SignalEvent) -> u64 {
        match self {
            HashField::SensorId => event.sensor_id_hash,
            HashField::SequenceId => event.sequence_id_hash,
            HashField::TraceId => event.trace_id_hash,
            HashField::ParentEvent => event.parent_event_hash,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashMode {
    Equals,
    InSet,
    NotInSet,
}

impl HashMode {
    #[inline]
    pub fn check(&self, value: u64, hashes: &[u64]) -> bool {
        match self {
            HashMode::Equals => hashes.len() == 1 && hashes[0] == value,
            HashMode::InSet => hashes.contains(&value),
            HashMode::NotInSet => !hashes.contains(&value),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMBINED CONDITION
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct CombinedCondition {
    pub logic: FilterLogic,
    pub conditions: Vec<FilterCondition>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SUBSCRIPTION FILTER IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

impl SubscriptionFilter {
    /// Создаёт новый фильтр с одним условием
    pub fn new(id: u64, condition: FilterCondition) -> Self {
        Self {
            id,
            conditions: vec![condition],
            logic: FilterLogic::And,
        }
    }

    /// Создаёт фильтр с несколькими условиями
    pub fn new_multi(id: u64, conditions: Vec<FilterCondition>, logic: FilterLogic) -> Self {
        Self {
            id,
            conditions,
            logic,
        }
    }

    /// Компилирует фильтр из JSON-представления
    ///
    /// Примеры:
    /// ```json
    /// {"event_type": "signal.input.external.text.chat"}
    /// {"event_type": {"$wildcard": "signal.input.external.*"}}
    /// {"priority": {"$gte": 200}}
    /// {"$and": [{"event_type": "..."}, {"priority": {"$gt": 100}}]}
    /// ```
    pub fn compile(
        id: u64,
        json: &JsonValue,
        registry: &mut EventTypeRegistry,
    ) -> Result<Self, FilterError> {
        // Проверяем на логические операторы верхнего уровня
        if let Some(obj) = json.as_object() {
            if obj.contains_key("$and") {
                let conditions = Self::compile_array(obj.get("$and").unwrap(), registry)?;
                return Ok(Self::new_multi(id, conditions, FilterLogic::And));
            } else if obj.contains_key("$or") {
                let conditions = Self::compile_array(obj.get("$or").unwrap(), registry)?;
                return Ok(Self::new_multi(id, conditions, FilterLogic::Or));
            }
        }

        // Обычный набор условий (AND по умолчанию)
        let conditions = Self::compile_conditions(json, registry)?;
        Ok(Self::new_multi(id, conditions, FilterLogic::And))
    }

    fn compile_array(
        json: &JsonValue,
        registry: &mut EventTypeRegistry,
    ) -> Result<Vec<FilterCondition>, FilterError> {
        let array = json
            .as_array()
            .ok_or(FilterError::InvalidFormat("Expected array".to_string()))?;

        let mut conditions = Vec::new();
        for item in array {
            conditions.extend(Self::compile_conditions(item, registry)?);
        }

        Ok(conditions)
    }

    fn compile_conditions(
        json: &JsonValue,
        registry: &mut EventTypeRegistry,
    ) -> Result<Vec<FilterCondition>, FilterError> {
        let obj = json
            .as_object()
            .ok_or(FilterError::InvalidFormat("Expected object".to_string()))?;

        let mut conditions = Vec::new();

        for (key, value) in obj {
            let condition = match key.as_str() {
                "event_type" => Self::compile_event_type(value, registry)?,
                field if NumericField::from_str(field).is_some() => {
                    Self::compile_numeric(field, value)?
                }
                _ => {
                    return Err(FilterError::UnknownField(key.clone()));
                }
            };

            conditions.push(condition);
        }

        Ok(conditions)
    }

    fn compile_event_type(
        value: &JsonValue,
        registry: &mut EventTypeRegistry,
    ) -> Result<FilterCondition, FilterError> {
        let pattern = if let Some(s) = value.as_str() {
            s.to_string()
        } else if let Some(obj) = value.as_object() {
            // {"$wildcard": "pattern"}
            if let Some(wildcard) = obj.get("$wildcard") {
                wildcard
                    .as_str()
                    .ok_or(FilterError::InvalidFormat("wildcard must be string".to_string()))?
                    .to_string()
            } else {
                return Err(FilterError::InvalidFormat(
                    "event_type must be string or {$wildcard: ...}".to_string(),
                ));
            }
        } else {
            return Err(FilterError::InvalidFormat(
                "event_type must be string".to_string(),
            ));
        };

        Ok(FilterCondition::EventType(EventTypeCondition::new(
            pattern, registry,
        )))
    }

    fn compile_numeric(field: &str, value: &JsonValue) -> Result<FilterCondition, FilterError> {
        let numeric_field = NumericField::from_str(field)
            .ok_or_else(|| FilterError::UnknownField(field.to_string()))?;

        // Простое значение: {"priority": 128} → {"priority": {"$eq": 128}}
        if let Some(num) = value.as_i64() {
            return Ok(FilterCondition::Numeric(NumericCondition {
                field: numeric_field,
                op: CompareOp::Eq,
                value: num,
            }));
        }

        // Оператор: {"priority": {"$gte": 200}}
        let obj = value
            .as_object()
            .ok_or(FilterError::InvalidFormat("Expected number or object".to_string()))?;

        if obj.len() != 1 {
            return Err(FilterError::InvalidFormat(
                "Expected single operator".to_string(),
            ));
        }

        let (op_str, val) = obj.iter().next().unwrap();
        let op = CompareOp::from_str(op_str)
            .ok_or_else(|| FilterError::UnknownOperator(op_str.clone()))?;

        let num = val
            .as_i64()
            .ok_or(FilterError::InvalidFormat("Operator value must be number".to_string()))?;

        Ok(FilterCondition::Numeric(NumericCondition {
            field: numeric_field,
            op,
            value: num,
        }))
    }

    /// Быстрая проверка события
    #[inline]
    pub fn matches(&self, event: &SignalEvent, registry: &EventTypeRegistry) -> bool {
        match self.logic {
            FilterLogic::And => self
                .conditions
                .iter()
                .all(|c| c.matches(event, registry)),
            FilterLogic::Or => self
                .conditions
                .iter()
                .any(|c| c.matches(event, registry)),
        }
    }
}

impl FilterCondition {
    #[inline]
    pub fn matches(&self, event: &SignalEvent, registry: &EventTypeRegistry) -> bool {
        match self {
            FilterCondition::EventType(c) => c.matches(event.event_type_id, registry),

            FilterCondition::Numeric(c) => {
                let value = c.field.extract(event);
                c.op.compare(value, c.value)
            }

            FilterCondition::Bitmap(c) => {
                let bitmap = c.field.extract(event);
                c.mode.check(bitmap, c.mask)
            }

            FilterCondition::Hash(c) => {
                let hash = c.field.extract(event);
                c.mode.check(hash, &c.hashes)
            }

            FilterCondition::Combined(c) => {
                match c.logic {
                    FilterLogic::And => c.conditions.iter().all(|cond| cond.matches(event, registry)),
                    FilterLogic::Or => c.conditions.iter().any(|cond| cond.matches(event, registry)),
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum FilterError {
    InvalidFormat(String),
    UnknownField(String),
    UnknownOperator(String),
}

impl std::fmt::Display for FilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            FilterError::UnknownField(field) => write!(f, "Unknown field: {}", field),
            FilterError::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
        }
    }
}

impl std::error::Error for FilterError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signal_system::EventTypeRegistry;

    #[test]
    fn test_event_type_condition_exact() {
        let mut registry = EventTypeRegistry::new();
        let condition = EventTypeCondition::new(
            "signal.input.external.text.chat".to_string(),
            &mut registry,
        );

        let id = registry.get_id("signal.input.external.text.chat").unwrap();
        assert!(condition.matches(id, &registry));

        let other_id = registry.get_id("signal.input.external.text.command").unwrap();
        assert!(!condition.matches(other_id, &registry));
    }

    #[test]
    fn test_event_type_condition_wildcard() {
        let mut registry = EventTypeRegistry::new();
        let condition = EventTypeCondition::new("signal.input.external.*".to_string(), &mut registry);

        assert!(condition.matching_ids.is_some());

        let id = registry.get_id("signal.input.external.text.chat").unwrap();
        assert!(condition.matches(id, &registry));

        let activation_id = registry.get_id("signal.activation.resonance").unwrap();
        assert!(!condition.matches(activation_id, &registry));
    }

    #[test]
    fn test_numeric_condition() {
        let mut event = SignalEvent::default();
        event.priority = 200;

        let condition = NumericCondition {
            field: NumericField::Priority,
            op: CompareOp::Gte,
            value: 150,
        };

        let value = condition.field.extract(&event);
        assert!(condition.op.compare(value, condition.value));

        event.priority = 100;
        let value = condition.field.extract(&event);
        assert!(!condition.op.compare(value, condition.value));
    }

    #[test]
    fn test_bitmap_condition() {
        let mut event = SignalEvent::default();
        event.tags_bitmap = 0b0000_0101; // bits 0 and 2

        let condition = BitmapCondition {
            field: BitmapField::Tags,
            mask: 0b0000_0001, // bit 0
            mode: BitmapMode::ContainsAny,
        };

        let bitmap = condition.field.extract(&event);
        assert!(condition.mode.check(bitmap, condition.mask));

        // Check ContainsAll
        let condition2 = BitmapCondition {
            field: BitmapField::Tags,
            mask: 0b0000_0101, // bits 0 and 2
            mode: BitmapMode::ContainsAll,
        };
        let bitmap = condition2.field.extract(&event);
        assert!(condition2.mode.check(bitmap, condition2.mask));

        let condition3 = BitmapCondition {
            field: BitmapField::Tags,
            mask: 0b0000_1000, // bit 3 (not set)
            mode: BitmapMode::NotContains,
        };
        let bitmap = condition3.field.extract(&event);
        assert!(condition3.mode.check(bitmap, condition3.mask));
    }

    #[test]
    fn test_filter_compile_simple() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "event_type": "signal.input.external.text.chat"
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();
        assert_eq!(filter.conditions.len(), 1);
        assert_eq!(filter.logic, FilterLogic::And);
    }

    #[test]
    fn test_filter_compile_wildcard() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "event_type": {"$wildcard": "signal.input.*"}
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();
        assert_eq!(filter.conditions.len(), 1);
    }

    #[test]
    fn test_filter_compile_numeric() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "priority": {"$gte": 200}
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();
        assert_eq!(filter.conditions.len(), 1);
    }

    #[test]
    fn test_filter_compile_and() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "$and": [
                {"event_type": "signal.input.external.text.chat"},
                {"priority": {"$gte": 200}}
            ]
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();
        assert_eq!(filter.conditions.len(), 2);
        assert_eq!(filter.logic, FilterLogic::And);
    }

    #[test]
    fn test_filter_matches() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "priority": {"$gte": 200}
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();

        let mut event = SignalEvent::default();
        event.priority = 250;
        assert!(filter.matches(&event, &registry));

        event.priority = 150;
        assert!(!filter.matches(&event, &registry));
    }

    #[test]
    fn test_filter_matches_combined() {
        let mut registry = EventTypeRegistry::new();
        let json = serde_json::json!({
            "event_type": {"$wildcard": "signal.input.external.*"},
            "priority": {"$gte": 200}
        });

        let filter = SubscriptionFilter::compile(1, &json, &mut registry).unwrap();

        let mut event = SignalEvent::default();
        event.event_type_id = registry.get_id("signal.input.external.text.chat").unwrap();
        event.priority = 250;

        assert!(filter.matches(&event, &registry));

        // Wrong event type
        event.event_type_id = registry.get_id("signal.activation.resonance").unwrap();
        assert!(!filter.matches(&event, &registry));

        // Wrong priority
        event.event_type_id = registry.get_id("signal.input.external.text.chat").unwrap();
        event.priority = 100;
        assert!(!filter.matches(&event, &registry));
    }
}
