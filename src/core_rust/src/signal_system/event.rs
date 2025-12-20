use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde::de::{self, Visitor};
use std::fmt;

/// Компактное представление сигнала для Rust Core
/// Размер: 256 bytes (оптимизировано для cache)
///
/// Структура спроектирована для:
/// - Эффективной передачи через FFI (PyO3)
/// - Минимального overhead при обработки
/// - Cache-friendly layout
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SignalEvent {
    // ═══════════════════════════════════════════════════════════════════
    // ИДЕНТИФИКАЦИЯ (32 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// UUID как два u64 (16 bytes)
    pub event_id_high: u64,
    pub event_id_low: u64,

    /// Тип события — индекс в таблице типов (4 bytes)
    pub event_type_id: u32,

    /// Версия схемы (4 bytes)
    pub schema_version: u32,

    // Padding убран для компактности

    // ═══════════════════════════════════════════════════════════════════
    // ИСТОЧНИК (32 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// ID сенсора — хэш строки (8 bytes)
    pub sensor_id_hash: u64,

    /// Domain: 0=external, 1=internal, 2=system (1 byte)
    pub domain: u8,

    /// Modality: индекс в таблице (1 byte)
    pub modality: u8,

    /// Confidence: 0-255 → 0.0-1.0 (1 byte)
    pub confidence: u8,

    /// Noise level: 0-255 → 0.0-1.0 (1 byte)
    pub noise_level: u8,

    /// Calibration: 0=calibrated, 1=uncalibrated, 2=degraded (1 byte)
    pub calibration_state: u8,

    /// Padding (3 bytes) - для выравнивания до 32 bytes
    _pad_source: [u8; 19],

    // ═══════════════════════════════════════════════════════════════════
    // СЕМАНТИЧЕСКОЕ ЯДРО (64 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// 8D вектор (32 bytes)
    pub vector: [f32; 8],

    /// Layer decomposition (32 bytes) — опционально, 0.0 если не задано
    /// [physical, spatial, temporal, causal, emotional, social, abstract, meta]
    pub layers: [f32; 8],

    // ═══════════════════════════════════════════════════════════════════
    // ЭНЕРГЕТИЧЕСКИЙ ПРОФИЛЬ (16 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// Magnitude: i16 (-32768..32767) (2 bytes)
    pub magnitude: i16,

    /// Valence: i8 → -1.0..1.0 (1 byte)
    pub valence: i8,

    /// Arousal: u8 → 0.0..1.0 (1 byte)
    pub arousal: u8,

    /// Urgency: u8 → 0.0..1.0 (1 byte)
    pub urgency: u8,

    /// Attack: u8 → 0.0..1.0 (1 byte)
    pub attack: u8,

    /// Decay: u8 → 0.0..1.0 (1 byte)
    pub decay: u8,

    /// Sustain: u8 → 0.0..1.0 (1 byte)
    pub sustain: u8,

    // ═══════════════════════════════════════════════════════════════════
    // ТЕМПОРАЛЬНАЯ ПРИВЯЗКА (32 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// Unix timestamp в микросекундах (8 bytes)
    pub timestamp_us: u64,

    /// Duration в микросекундах, 0 если мгновенный (8 bytes)
    pub duration_us: u64,

    /// NeuroTick (8 bytes)
    pub neuro_tick: u64,

    /// Sequence ID hash, 0 если нет (8 bytes)
    pub sequence_id_hash: u64,

    // ═══════════════════════════════════════════════════════════════════
    // МАРШРУТИЗАЦИЯ (32 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// Priority: 0-255 (1 byte)
    pub priority: u8,

    /// TTL: 0-255 (1 byte)
    pub ttl: u8,

    /// Hop count (1 byte)
    pub hop_count: u8,

    /// Flags: битовая маска (1 byte)
    /// bit 0: has_trace_id
    /// bit 1: has_parent
    /// bit 2: requires_response
    /// bit 3: is_broadcast
    pub flags: u8,

    /// Tags bitmap — до 32 предопределённых тегов (4 bytes)
    pub tags_bitmap: u32,

    /// Trace ID hash (8 bytes)
    pub trace_id_hash: u64,

    /// Parent event ID hash (8 bytes)
    pub parent_event_hash: u64,

    // ═══════════════════════════════════════════════════════════════════
    // RAW DATA REFERENCE (48 bytes)
    // ═══════════════════════════════════════════════════════════════════

    /// Data type: 0=none, 1=text, 2=float_array, 3=blob, 4=structured (1 byte)
    pub data_type: u8,

    /// Data location: 0=inline, 1=external (1 byte)
    pub data_location: u8,

    /// Data size in bytes (4 bytes)
    pub data_size: u32,

    /// Checksum of data (2 bytes)
    pub data_checksum: u16,

    /// Inline data: для коротких строк до 40 bytes
    /// Если data_location=1, это offset в external buffer
    pub inline_data: [u8; 40],

    /// Padding для выравнивания до 256 bytes
    _pad_final: [u8; 16],
}

impl Default for SignalEvent {
    fn default() -> Self {
        Self {
            event_id_high: 0,
            event_id_low: 0,
            event_type_id: 0,
            schema_version: 1,
            sensor_id_hash: 0,
            domain: 0,
            modality: 0,
            confidence: 255, // 1.0
            noise_level: 0,
            calibration_state: 0,
            _pad_source: [0; 19],
            vector: [0.0; 8],
            layers: [0.0; 8],
            magnitude: 0,
            valence: 0,
            arousal: 128, // 0.5
            urgency: 128, // 0.5
            attack: 128,
            decay: 77,
            sustain: 128,
            timestamp_us: 0,
            duration_us: 0,
            neuro_tick: 0,
            sequence_id_hash: 0,
            priority: 128,
            ttl: 10,
            hop_count: 0,
            flags: 0,
            tags_bitmap: 0,
            trace_id_hash: 0,
            parent_event_hash: 0,
            data_type: 0,
            data_location: 0,
            data_size: 0,
            data_checksum: 0,
            inline_data: [0; 40],
            _pad_final: [0; 16],
        }
    }
}

impl SignalEvent {
    /// Размер структуры в bytes
    pub const SIZE: usize = 256;

    /// Создаёт новый SignalEvent с минимальными данными
    pub fn new(event_type_id: u32, vector: [f32; 8]) -> Self {
        Self {
            event_type_id,
            vector,
            ..Default::default()
        }
    }

    /// Генерирует новый event_id
    pub fn generate_id(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        // Используем timestamp для high part
        self.event_id_high = now.as_secs();

        // Используем наносекунды + random для low part
        self.event_id_low = now.subsec_nanos() as u64;
    }

    /// Устанавливает timestamp в текущее время
    pub fn set_timestamp_now(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();

        self.timestamp_us = now.as_micros() as u64;
    }

    /// Сериализация в bytes
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        unsafe { std::mem::transmute_copy(self) }
    }

    /// Десериализация из bytes
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        unsafe { std::mem::transmute_copy(bytes) }
    }

    /// Проверяет корректность структуры
    pub fn validate(&self) -> bool {
        // Проверяем что вектор не содержит NaN/Inf
        for &v in &self.vector {
            if !v.is_finite() {
                return false;
            }
        }

        // Проверяем что layers не содержат NaN/Inf
        for &l in &self.layers {
            if !l.is_finite() {
                return false;
            }
        }

        true
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ВСПОМОГАТЕЛЬНЫЕ СТРУКТУРЫ (для удобства работы в Python)
// ═══════════════════════════════════════════════════════════════════════════════

/// Информация об источнике сигнала
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalSource {
    pub domain: String,      // "external" | "internal" | "system"
    pub modality: String,    // "text" | "audio" | "vision" | ...
    pub sensor_id: String,
    pub sensor_type: String,
    pub confidence: f32,
    pub noise_level: f32,
}

/// Семантическое ядро с развёрнутой информацией
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticCore {
    pub vector: Vec<f32>,
    pub layer_decomposition: Option<LayerDecomposition>,
    pub encoding_method: String,
}

/// Разложение по семантическим слоям
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerDecomposition {
    pub physical: f32,
    pub spatial: f32,
    pub temporal: f32,
    pub causal: f32,
    pub emotional: f32,
    pub social: f32,
    pub abstract_layer: f32,
    pub meta: f32,
}

/// Энергетический профиль
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyProfile {
    pub magnitude: i16,
    pub valence: f32,    // -1.0 .. 1.0
    pub arousal: f32,    // 0.0 .. 1.0
    pub urgency: f32,    // 0.0 .. 1.0
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
}

/// Темпоральная привязка
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalBinding {
    pub timestamp: f64,
    pub duration: Option<f64>,
    pub neuro_tick: Option<u64>,
    pub sequence_id: Option<String>,
}

/// Информация о маршрутизации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingInfo {
    pub priority: u8,
    pub ttl: u8,
    pub tags: Vec<String>,
    pub trace_id: Option<String>,
    pub parent_event_id: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// CUSTOM SERDE IMPLEMENTATION (для поддержки inline_data: [u8; 40])
// ═══════════════════════════════════════════════════════════════════════════════

impl Serialize for SignalEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Используем to_bytes() для сериализации всей структуры как массива байт
        serializer.serialize_bytes(&self.to_bytes())
    }
}

impl<'de> Deserialize<'de> for SignalEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignalEventVisitor;

        impl<'de> Visitor<'de> for SignalEventVisitor {
            type Value = SignalEvent;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a 256-byte SignalEvent structure")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if v.len() != SignalEvent::SIZE {
                    return Err(E::custom(format!(
                        "expected {} bytes, got {}",
                        SignalEvent::SIZE,
                        v.len()
                    )));
                }

                let mut bytes = [0u8; SignalEvent::SIZE];
                bytes.copy_from_slice(v);
                Ok(SignalEvent::from_bytes(&bytes))
            }
        }

        deserializer.deserialize_bytes(SignalEventVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_event_size() {
        assert_eq!(std::mem::size_of::<SignalEvent>(), SignalEvent::SIZE);
        assert_eq!(SignalEvent::SIZE, 256);
    }

    #[test]
    fn test_signal_event_new() {
        let event = SignalEvent::new(1, [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]);
        assert_eq!(event.event_type_id, 1);
        assert_eq!(event.vector[0], 0.1);
        assert_eq!(event.schema_version, 1);
    }

    #[test]
    fn test_signal_event_validation() {
        let mut event = SignalEvent::default();
        assert!(event.validate());

        // Невалидный вектор
        event.vector[0] = f32::NAN;
        assert!(!event.validate());
    }

    #[test]
    fn test_signal_event_serialization() {
        let event = SignalEvent::new(42, [1.0; 8]);
        let bytes = event.to_bytes();
        let restored = SignalEvent::from_bytes(&bytes);

        assert_eq!(restored.event_type_id, 42);
        assert_eq!(restored.vector, [1.0; 8]);
    }

    #[test]
    fn test_signal_event_generate_id() {
        let mut event = SignalEvent::default();
        event.generate_id();

        assert_ne!(event.event_id_high, 0);
    }
}
