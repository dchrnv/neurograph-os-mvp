"""
Sensor Registry - dynamic sensor registration and management

The registry maintains a catalog of all active sensors,
their types, encoders, and metadata.
"""

from typing import Dict, Optional, Callable, Any, List
from dataclasses import dataclass, field
from datetime import datetime
import threading

from ..encoders import EncoderType


@dataclass
class SensorConfig:
    """
    Конфигурация одного сенсора.

    Определяет тип сенсора, энкодер, метаданные и опциональный
    кастомный обработчик для препроцессинга данных.
    """

    sensor_id: str
    """Уникальный ID сенсора"""

    sensor_type: str
    """Тип сенсора (text_chat, system_monitor, timer, etc.)"""

    domain: str
    """Домен: external | internal | system"""

    modality: str
    """Модальность: text | audio | vision | numeric | multimodal"""

    encoder_type: EncoderType
    """Тип энкодера для преобразования данных"""

    description: str = ""
    """Описание сенсора"""

    default_priority: int = 128
    """Приоритет по умолчанию (0-255)"""

    default_confidence: float = 1.0
    """Уверенность по умолчанию (0.0-1.0)"""

    metadata: Dict[str, Any] = field(default_factory=dict)
    """Дополнительные метаданные"""

    preprocessor: Optional[Callable[[Any], Any]] = None
    """Опциональная функция препроцессинга перед энкодированием"""

    registered_at: datetime = field(default_factory=datetime.now)
    """Время регистрации"""

    enabled: bool = True
    """Активен ли сенсор"""


class SensorRegistry:
    """
    Реестр сенсоров для SignalGateway.

    Thread-safe хранилище конфигураций сенсоров с возможностью
    динамической регистрации, обновления и удаления.

    Usage:
        registry = SensorRegistry()
        registry.register_sensor(
            sensor_id="telegram_bot_001",
            sensor_type="text_chat",
            domain="external",
            modality="text",
            encoder_type=EncoderType.TEXT_TFIDF
        )
    """

    def __init__(self):
        self._sensors: Dict[str, SensorConfig] = {}
        self._lock = threading.RLock()

    # ═══════════════════════════════════════════════════════════════════════════════
    # REGISTRATION API
    # ═══════════════════════════════════════════════════════════════════════════════

    def register_sensor(
        self,
        sensor_id: str,
        sensor_type: str,
        domain: str,
        modality: str,
        encoder_type: EncoderType,
        description: str = "",
        default_priority: int = 128,
        default_confidence: float = 1.0,
        metadata: Optional[Dict[str, Any]] = None,
        preprocessor: Optional[Callable[[Any], Any]] = None,
    ) -> SensorConfig:
        """
        Зарегистрировать новый сенсор.

        Args:
            sensor_id: Уникальный ID сенсора
            sensor_type: Тип сенсора (text_chat, etc.)
            domain: external | internal | system
            modality: text | audio | vision | numeric | multimodal
            encoder_type: Тип энкодера
            description: Описание сенсора
            default_priority: Приоритет по умолчанию (0-255)
            default_confidence: Уверенность по умолчанию (0.0-1.0)
            metadata: Дополнительные метаданные
            preprocessor: Функция препроцессинга

        Returns:
            SensorConfig созданного сенсора

        Raises:
            ValueError: Если sensor_id уже зарегистрирован
        """
        with self._lock:
            if sensor_id in self._sensors:
                raise ValueError(f"Sensor '{sensor_id}' already registered")

            config = SensorConfig(
                sensor_id=sensor_id,
                sensor_type=sensor_type,
                domain=domain,
                modality=modality,
                encoder_type=encoder_type,
                description=description,
                default_priority=default_priority,
                default_confidence=default_confidence,
                metadata=metadata or {},
                preprocessor=preprocessor,
            )

            self._sensors[sensor_id] = config
            return config

    def unregister_sensor(self, sensor_id: str) -> bool:
        """
        Удалить сенсор из реестра.

        Args:
            sensor_id: ID сенсора

        Returns:
            True если сенсор был удалён, False если не найден
        """
        with self._lock:
            if sensor_id in self._sensors:
                del self._sensors[sensor_id]
                return True
            return False

    def update_sensor(
        self,
        sensor_id: str,
        **updates
    ) -> SensorConfig:
        """
        Обновить конфигурацию сенсора.

        Args:
            sensor_id: ID сенсора
            **updates: Поля для обновления

        Returns:
            Обновлённый SensorConfig

        Raises:
            KeyError: Если сенсор не найден
        """
        with self._lock:
            if sensor_id not in self._sensors:
                raise KeyError(f"Sensor '{sensor_id}' not found")

            config = self._sensors[sensor_id]
            for key, value in updates.items():
                if hasattr(config, key):
                    setattr(config, key, value)

            return config

    # ═══════════════════════════════════════════════════════════════════════════════
    # QUERY API
    # ═══════════════════════════════════════════════════════════════════════════════

    def get_sensor(self, sensor_id: str) -> Optional[SensorConfig]:
        """
        Получить конфигурацию сенсора по ID.

        Args:
            sensor_id: ID сенсора

        Returns:
            SensorConfig или None если не найден
        """
        with self._lock:
            return self._sensors.get(sensor_id)

    def list_sensors(
        self,
        domain: Optional[str] = None,
        modality: Optional[str] = None,
        sensor_type: Optional[str] = None,
        enabled_only: bool = True,
    ) -> List[SensorConfig]:
        """
        Получить список сенсоров с фильтрацией.

        Args:
            domain: Фильтр по домену (external/internal/system)
            modality: Фильтр по модальности (text/audio/etc.)
            sensor_type: Фильтр по типу сенсора
            enabled_only: Вернуть только активные сенсоры

        Returns:
            Список SensorConfig
        """
        with self._lock:
            sensors = list(self._sensors.values())

            if enabled_only:
                sensors = [s for s in sensors if s.enabled]

            if domain:
                sensors = [s for s in sensors if s.domain == domain]

            if modality:
                sensors = [s for s in sensors if s.modality == modality]

            if sensor_type:
                sensors = [s for s in sensors if s.sensor_type == sensor_type]

            return sensors

    def sensor_exists(self, sensor_id: str) -> bool:
        """Проверить, зарегистрирован ли сенсор."""
        with self._lock:
            return sensor_id in self._sensors

    def count_sensors(
        self,
        domain: Optional[str] = None,
        enabled_only: bool = True,
    ) -> int:
        """Подсчитать количество сенсоров с фильтрацией."""
        return len(self.list_sensors(domain=domain, enabled_only=enabled_only))

    # ═══════════════════════════════════════════════════════════════════════════════
    # STATE MANAGEMENT
    # ═══════════════════════════════════════════════════════════════════════════════

    def enable_sensor(self, sensor_id: str) -> bool:
        """Активировать сенсор."""
        try:
            self.update_sensor(sensor_id, enabled=True)
            return True
        except KeyError:
            return False

    def disable_sensor(self, sensor_id: str) -> bool:
        """Деактивировать сенсор."""
        try:
            self.update_sensor(sensor_id, enabled=False)
            return True
        except KeyError:
            return False

    def clear(self):
        """Очистить все сенсоры (для тестов)."""
        with self._lock:
            self._sensors.clear()

    # ═══════════════════════════════════════════════════════════════════════════════
    # BUILT-IN SENSORS
    # ═══════════════════════════════════════════════════════════════════════════════

    def register_builtin_sensors(self):
        """
        Зарегистрировать встроенные сенсоры.

        Создаёт базовый набор сенсоров для MVP:
        - text_chat: Текстовый чат (TF-IDF)
        - system_monitor: Системные метрики (numeric)
        - timer: Периодические события (passthrough)
        """
        # 1. Text Chat Sensor
        self.register_sensor(
            sensor_id="builtin.text_chat",
            sensor_type="text_chat",
            domain="external",
            modality="text",
            encoder_type=EncoderType.TEXT_TFIDF,
            description="Text chat messages from external sources",
            default_priority=200,
            default_confidence=1.0,
            metadata={
                "supports_streaming": False,
                "max_length": 4096,
                "language": "multilingual",
            }
        )

        # 2. System Monitor Sensor
        self.register_sensor(
            sensor_id="builtin.system_monitor",
            sensor_type="system_monitor",
            domain="system",
            modality="numeric",
            encoder_type=EncoderType.NUMERIC_DIRECT,
            description="System metrics (CPU, memory, etc.)",
            default_priority=100,
            default_confidence=1.0,
            metadata={
                "metrics": ["cpu_percent", "memory_percent", "disk_io", "network_io"],
                "sample_rate_ms": 1000,
            }
        )

        # 3. Timer Sensor
        self.register_sensor(
            sensor_id="builtin.timer",
            sensor_type="timer",
            domain="system",
            modality="numeric",
            encoder_type=EncoderType.PASSTHROUGH,
            description="Periodic timer events",
            default_priority=50,
            default_confidence=1.0,
            metadata={
                "interval_ms": 1000,
                "precision": "millisecond",
            }
        )


__all__ = [
    "SensorConfig",
    "SensorRegistry",
]
