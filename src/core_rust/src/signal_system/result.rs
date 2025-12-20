use serde::{Deserialize, Serialize};

/// Результат обработки сигнала SignalSystem
///
/// Возвращается после emit() и содержит:
/// - ID созданного токена
/// - Активированные соседи
/// - Метрики активации
/// - Флаги (новизна, аномалия)
/// - Триггеры действий
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    /// ID созданного токена в Grid
    pub token_id: u32,

    /// Активированные соседи (топ-N)
    pub neighbors: Vec<NeighborInfo>,

    /// Изменение энергии системы
    pub energy_delta: f32,

    /// Количество затронутых токенов spreading activation
    pub activation_spread: u32,

    /// Флаг новизны (нет близких соседей)
    pub is_novel: bool,

    /// ID токенов-действий, если сработали
    pub triggered_actions: Vec<u32>,

    /// Степень аномальности [0.0, 1.0]
    pub anomaly_score: f32,

    /// Время обработки в микросекундах
    pub processing_time_us: u64,
}

impl ProcessingResult {
    /// Создаёт результат с минимальными данными
    pub fn new(token_id: u32) -> Self {
        Self {
            token_id,
            neighbors: Vec::new(),
            energy_delta: 0.0,
            activation_spread: 0,
            is_novel: false,
            triggered_actions: Vec::new(),
            anomaly_score: 0.0,
            processing_time_us: 0,
        }
    }

    /// Создаёт результат для ошибки валидации
    pub fn validation_failed(reason: String) -> Self {
        Self {
            token_id: 0,
            neighbors: Vec::new(),
            energy_delta: 0.0,
            activation_spread: 0,
            is_novel: false,
            triggered_actions: Vec::new(),
            anomaly_score: 1.0, // Максимальная аномальность
            processing_time_us: 0,
        }
    }

    /// Добавляет соседа к результату
    pub fn add_neighbor(&mut self, neighbor: NeighborInfo) {
        self.neighbors.push(neighbor);
    }

    /// Добавляет триггер действия
    pub fn add_triggered_action(&mut self, action_id: u32) {
        self.triggered_actions.push(action_id);
    }

    /// Проверяет наличие активаций
    pub fn has_activations(&self) -> bool {
        !self.neighbors.is_empty() || self.activation_spread > 0
    }

    /// Проверяет наличие действий
    pub fn has_actions(&self) -> bool {
        !self.triggered_actions.is_empty()
    }
}

impl Default for ProcessingResult {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Информация об активированном соседе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeighborInfo {
    /// ID токена
    pub token_id: u32,

    /// Расстояние в 8D пространстве [0.0, ∞)
    pub distance: f32,

    /// Сила резонанса [0.0, 1.0]
    /// Обычно: resonance = 1.0 - distance (нормализованное)
    pub resonance: f32,

    /// Тип токена (индекс)
    /// 0 = concept, 1 = action, 2 = memory, 3 = emotion, etc.
    pub token_type: u8,

    /// Доминирующий семантический слой [0-7]
    /// 0=physical, 1=spatial, 2=temporal, 3=causal,
    /// 4=emotional, 5=social, 6=abstract, 7=meta
    pub layer_affinity: u8,
}

impl NeighborInfo {
    /// Создаёт информацию о соседе
    pub fn new(token_id: u32, distance: f32) -> Self {
        Self {
            token_id,
            distance,
            resonance: Self::compute_resonance(distance),
            token_type: 0,
            layer_affinity: 0,
        }
    }

    /// Создаёт с полной информацией
    pub fn with_details(
        token_id: u32,
        distance: f32,
        token_type: u8,
        layer_affinity: u8,
    ) -> Self {
        Self {
            token_id,
            distance,
            resonance: Self::compute_resonance(distance),
            token_type,
            layer_affinity,
        }
    }

    /// Вычисляет резонанс из расстояния
    ///
    /// Использует экспоненциальное затухание:
    /// resonance = exp(-distance)
    fn compute_resonance(distance: f32) -> f32 {
        (-distance).exp().max(0.0).min(1.0)
    }

    /// Проверяет что это сильный резонанс
    pub fn is_strong_resonance(&self, threshold: f32) -> bool {
        self.resonance >= threshold
    }
}

/// Типы токенов (enum для удобства)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TokenType {
    Concept = 0,
    Action = 1,
    Memory = 2,
    Emotion = 3,
    Sensor = 4,
    System = 5,
}

impl From<u8> for TokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => TokenType::Concept,
            1 => TokenType::Action,
            2 => TokenType::Memory,
            3 => TokenType::Emotion,
            4 => TokenType::Sensor,
            5 => TokenType::System,
            _ => TokenType::Concept,
        }
    }
}

impl From<TokenType> for u8 {
    fn from(t: TokenType) -> Self {
        t as u8
    }
}

/// Семантические слои
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SemanticLayer {
    Physical = 0,
    Spatial = 1,
    Temporal = 2,
    Causal = 3,
    Emotional = 4,
    Social = 5,
    Abstract = 6,
    Meta = 7,
}

impl From<u8> for SemanticLayer {
    fn from(value: u8) -> Self {
        match value {
            0 => SemanticLayer::Physical,
            1 => SemanticLayer::Spatial,
            2 => SemanticLayer::Temporal,
            3 => SemanticLayer::Causal,
            4 => SemanticLayer::Emotional,
            5 => SemanticLayer::Social,
            6 => SemanticLayer::Abstract,
            7 => SemanticLayer::Meta,
            _ => SemanticLayer::Physical,
        }
    }
}

impl From<SemanticLayer> for u8 {
    fn from(l: SemanticLayer) -> Self {
        l as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_result_new() {
        let result = ProcessingResult::new(42);
        assert_eq!(result.token_id, 42);
        assert_eq!(result.neighbors.len(), 0);
        assert!(!result.is_novel);
    }

    #[test]
    fn test_processing_result_validation_failed() {
        let result = ProcessingResult::validation_failed("test".to_string());
        assert_eq!(result.token_id, 0);
        assert_eq!(result.anomaly_score, 1.0);
    }

    #[test]
    fn test_add_neighbor() {
        let mut result = ProcessingResult::new(1);
        result.add_neighbor(NeighborInfo::new(2, 0.5));

        assert_eq!(result.neighbors.len(), 1);
        assert_eq!(result.neighbors[0].token_id, 2);
    }

    #[test]
    fn test_add_triggered_action() {
        let mut result = ProcessingResult::new(1);
        result.add_triggered_action(100);

        assert!(result.has_actions());
        assert_eq!(result.triggered_actions[0], 100);
    }

    #[test]
    fn test_neighbor_info_resonance() {
        let neighbor = NeighborInfo::new(1, 0.0);
        assert!(neighbor.resonance > 0.99); // distance=0 → resonance≈1

        let neighbor = NeighborInfo::new(2, 1.0);
        assert!(neighbor.resonance < 0.4); // distance=1 → resonance≈0.368

        let neighbor = NeighborInfo::new(3, 5.0);
        assert!(neighbor.resonance < 0.01); // distance=5 → resonance≈0.007
    }

    #[test]
    fn test_neighbor_with_details() {
        let neighbor = NeighborInfo::with_details(
            42,
            0.5,
            TokenType::Action as u8,
            SemanticLayer::Emotional as u8,
        );

        assert_eq!(neighbor.token_id, 42);
        assert_eq!(neighbor.token_type, 1); // Action
        assert_eq!(neighbor.layer_affinity, 4); // Emotional
    }

    #[test]
    fn test_is_strong_resonance() {
        let neighbor = NeighborInfo::new(1, 0.1);
        assert!(neighbor.is_strong_resonance(0.5)); // resonance≈0.9 > 0.5

        let neighbor = NeighborInfo::new(2, 2.0);
        assert!(!neighbor.is_strong_resonance(0.5)); // resonance≈0.135 < 0.5
    }

    #[test]
    fn test_token_type_conversion() {
        assert_eq!(TokenType::from(1), TokenType::Action);
        assert_eq!(u8::from(TokenType::Memory), 2);
    }

    #[test]
    fn test_semantic_layer_conversion() {
        assert_eq!(SemanticLayer::from(4), SemanticLayer::Emotional);
        assert_eq!(u8::from(SemanticLayer::Temporal), 2);
    }
}
