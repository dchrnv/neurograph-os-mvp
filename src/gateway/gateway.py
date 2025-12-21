"""
SignalGateway - main sensory interface for NeuroGraph OS

The Gateway is the entry point for all external signals.
It normalizes, encodes, and routes signals to the Core.
"""

import time
from typing import Optional, Dict, Any, List, Callable
import uuid

from .models import (
    SignalEvent,
    SignalSource,
    SemanticCore,
    EnergyProfile,
    TemporalBinding,
    RawPayload,
    RoutingInfo,
)
from .registry import SensorRegistry, SensorConfig
from .encoders import (
    EncoderType,
    BaseEncoder,
    PassthroughEncoder,
    NumericDirectEncoder,
    TextTfidfEncoder,
    SentimentSimpleEncoder,
)


class SignalGateway:
    """
    Главный интерфейс Gateway v2.0.

    Отвечает за:
    1. Приём сырых данных от сенсоров
    2. Нормализацию и валидацию
    3. Кодирование в 8D векторы
    4. Упаковку в SignalEvent
    5. Передачу в Core (SignalSystem)

    Architecture:
        External Source → Gateway.push_*() → normalize → encode → SignalEvent → Core

    Usage:
        gateway = SignalGateway()
        gateway.initialize()  # Register built-in sensors

        # Push text signal
        event = gateway.push_text(
            text="Hello, world!",
            sensor_id="telegram_bot_001"
        )
    """

    def __init__(self, core_system=None):
        """
        Создать Gateway.

        Args:
            core_system: Опциональная ссылка на _core.SignalSystem
                        (для прямой передачи событий в Core)
        """
        self.registry = SensorRegistry()
        self.core_system = core_system

        # Счётчики для статистики
        self._total_events = 0
        self._neuro_tick = 0

        # Encoders
        self._encoders: Dict[EncoderType, BaseEncoder] = {}

    # ═══════════════════════════════════════════════════════════════════════════════
    # INITIALIZATION
    # ═══════════════════════════════════════════════════════════════════════════════

    def initialize(self, register_builtin: bool = True):
        """
        Инициализировать Gateway.

        Args:
            register_builtin: Зарегистрировать встроенные сенсоры
        """
        if register_builtin:
            self.registry.register_builtin_sensors()

        # Initialize encoders
        self._initialize_encoders()

    def _initialize_encoders(self):
        """Инициализировать энкодеры."""
        self._encoders = {
            EncoderType.PASSTHROUGH: PassthroughEncoder(),
            EncoderType.NUMERIC_DIRECT: NumericDirectEncoder(),
            EncoderType.TEXT_TFIDF: TextTfidfEncoder(),
            EncoderType.SENTIMENT_SIMPLE: SentimentSimpleEncoder(),
        }

    # ═══════════════════════════════════════════════════════════════════════════════
    # PUSH API (Main Entry Points)
    # ═══════════════════════════════════════════════════════════════════════════════

    def push_text(
        self,
        text: str,
        sensor_id: str = "builtin.text_chat",
        priority: int = 200,
        metadata: Optional[Dict[str, Any]] = None,
        sequence_id: Optional[str] = None,
    ) -> SignalEvent:
        """
        Отправить текстовый сигнал.

        Args:
            text: Текстовое сообщение
            sensor_id: ID сенсора (по умолчанию builtin.text_chat)
            priority: Приоритет обработки (0-255)
            metadata: Дополнительные метаданные
            sequence_id: ID последовательности (для диалогов)

        Returns:
            Созданное SignalEvent

        Raises:
            ValueError: Если sensor_id не найден или неправильного типа
        """
        return self._push_signal(
            data=text,
            data_type="text",
            sensor_id=sensor_id,
            priority=priority,
            metadata=metadata,
            sequence_id=sequence_id,
        )

    def push_audio(
        self,
        audio_data: bytes,
        sensor_id: str,
        priority: int = 180,
        sample_rate: int = 16000,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Отправить аудио сигнал.

        Args:
            audio_data: Аудио данные (bytes)
            sensor_id: ID сенсора
            priority: Приоритет обработки
            sample_rate: Частота дискретизации (Hz)
            metadata: Дополнительные метаданные

        Returns:
            Созданное SignalEvent
        """
        meta = metadata or {}
        meta["sample_rate"] = sample_rate

        return self._push_signal(
            data=audio_data,
            data_type="audio",
            sensor_id=sensor_id,
            priority=priority,
            metadata=meta,
        )

    def push_vision(
        self,
        image_data: bytes,
        sensor_id: str,
        priority: int = 150,
        width: Optional[int] = None,
        height: Optional[int] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Отправить визуальный сигнал (изображение).

        Args:
            image_data: Данные изображения (bytes)
            sensor_id: ID сенсора
            priority: Приоритет обработки
            width: Ширина изображения (px)
            height: Высота изображения (px)
            metadata: Дополнительные метаданные

        Returns:
            Созданное SignalEvent
        """
        meta = metadata or {}
        if width:
            meta["width"] = width
        if height:
            meta["height"] = height

        return self._push_signal(
            data=image_data,
            data_type="image",
            sensor_id=sensor_id,
            priority=priority,
            metadata=meta,
        )

    def push_system(
        self,
        metric_name: str,
        metric_value: float,
        sensor_id: str = "builtin.system_monitor",
        priority: int = 100,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> SignalEvent:
        """
        Отправить системную метрику.

        Args:
            metric_name: Название метрики (cpu_percent, memory_percent, etc.)
            metric_value: Значение метрики
            sensor_id: ID сенсора (по умолчанию builtin.system_monitor)
            priority: Приоритет обработки
            metadata: Дополнительные метаданные

        Returns:
            Созданное SignalEvent
        """
        data = {
            "metric": metric_name,
            "value": metric_value,
        }

        return self._push_signal(
            data=data,
            data_type="json",
            sensor_id=sensor_id,
            priority=priority,
            metadata=metadata,
        )

    # ═══════════════════════════════════════════════════════════════════════════════
    # INTERNAL PROCESSING
    # ═══════════════════════════════════════════════════════════════════════════════

    def _push_signal(
        self,
        data: Any,
        data_type: str,
        sensor_id: str,
        priority: int,
        metadata: Optional[Dict[str, Any]] = None,
        sequence_id: Optional[str] = None,
    ) -> SignalEvent:
        """
        Внутренний метод обработки сигнала.

        Выполняет полный цикл:
        1. Валидация сенсора
        2. Нормализация данных
        3. Кодирование в 8D
        4. Создание SignalEvent
        5. Передача в Core (если подключен)

        Args:
            data: Сырые данные
            data_type: Тип данных (text/audio/image/json)
            sensor_id: ID сенсора
            priority: Приоритет
            metadata: Метаданные
            sequence_id: ID последовательности

        Returns:
            SignalEvent
        """
        # 1. Get sensor config
        sensor_config = self.registry.get_sensor(sensor_id)
        if sensor_config is None:
            raise ValueError(f"Sensor '{sensor_id}' not found in registry")

        if not sensor_config.enabled:
            raise ValueError(f"Sensor '{sensor_id}' is disabled")

        # 2. Increment tick and stats FIRST (before event creation)
        self._neuro_tick += 1
        self._total_events += 1

        # 3. Normalize data (apply preprocessor if exists)
        normalized_data = self._normalize_data(data, sensor_config)

        # 4. Encode to 8D vector
        vector_8d = self._encode_to_vector(normalized_data, sensor_config)

        # 5. Build SignalEvent (will use current neuro_tick)
        event = self._build_event(
            data=data,
            data_type=data_type,
            normalized_data=normalized_data,
            vector_8d=vector_8d,
            sensor_config=sensor_config,
            priority=priority,
            metadata=metadata,
            sequence_id=sequence_id,
        )

        # 6. Send to Core (if connected)
        if self.core_system is not None:
            self._send_to_core(event)

        return event

    def _normalize_data(self, data: Any, sensor_config: SensorConfig) -> Any:
        """
        Нормализовать данные перед кодированием.

        Применяет preprocessor если он определён в конфигурации сенсора.
        """
        if sensor_config.preprocessor is not None:
            return sensor_config.preprocessor(data)
        return data

    def _encode_to_vector(self, data: Any, sensor_config: SensorConfig) -> List[float]:
        """
        Закодировать данные в 8D вектор используя энкодер сенсора.

        Args:
            data: Нормализованные данные
            sensor_config: Конфигурация сенсора

        Returns:
            8D вектор [0, 1]
        """
        encoder_type = sensor_config.encoder_type
        encoder = self._encoders.get(encoder_type)

        if encoder is None:
            # Fallback: zero vector
            return [0.0] * 8

        try:
            vector = encoder.encode(data)
            return vector
        except Exception as e:
            # Fallback on encoding error
            print(f"Warning: encoder {encoder_type} failed: {e}")
            return [0.0] * 8

    def _build_event(
        self,
        data: Any,
        data_type: str,
        normalized_data: Any,
        vector_8d: List[float],
        sensor_config: SensorConfig,
        priority: int,
        metadata: Optional[Dict[str, Any]],
        sequence_id: Optional[str],
    ) -> SignalEvent:
        """
        Собрать финальное SignalEvent.

        Создаёт все вложенные структуры и упаковывает в SignalEvent.
        """
        timestamp_us = int(time.time() * 1_000_000)

        # Build nested structures
        source = SignalSource(
            domain=sensor_config.domain,
            modality=sensor_config.modality,
            sensor_id=sensor_config.sensor_id,
            sensor_type=sensor_config.sensor_type,
            sensor_meta=sensor_config.metadata,
            confidence=sensor_config.default_confidence,
        )

        semantic = SemanticCore(
            vector=vector_8d,
            encoding_method=sensor_config.encoder_type.value,
        )

        energy = EnergyProfile(
            magnitude=1.0,  # TODO: compute from data
            valence=0.0,    # TODO: sentiment analysis
            arousal=0.5,    # TODO: compute from urgency
            urgency=priority / 255.0,  # Normalize priority to [0, 1]
        )

        temporal = TemporalBinding(
            timestamp_us=timestamp_us,
            neuro_tick=self._neuro_tick,
            sequence_id=sequence_id,
        )

        # Determine size
        if isinstance(data, (bytes, bytearray)):
            size_bytes = len(data)
        elif isinstance(data, str):
            size_bytes = len(data.encode('utf-8'))
        else:
            size_bytes = 0

        # Determine MIME type
        mime_type = {
            "text": "text/plain",
            "audio": "audio/wav",
            "image": "image/png",
            "json": "application/json",
            "binary": "application/octet-stream",
        }.get(data_type, "application/octet-stream")

        payload = RawPayload(
            data_type=data_type,
            data=data,
            mime_type=mime_type,
            size_bytes=size_bytes,
            metadata=metadata or {},
        )

        routing = RoutingInfo(
            priority=priority,
            tags=[sensor_config.sensor_type, sensor_config.domain],
        )

        # Determine event_type based on sensor config
        event_type = f"signal.input.{sensor_config.domain}.{sensor_config.modality}.{sensor_config.sensor_type}"

        # Build final event
        event = SignalEvent(
            event_type=event_type,
            source=source,
            semantic=semantic,
            energy=energy,
            temporal=temporal,
            payload=payload,
            routing=routing,
        )

        return event

    def _send_to_core(self, event: SignalEvent):
        """
        Отправить событие в Rust Core.

        TODO: Преобразовать SignalEvent в формат _core.SignalSystem
        """
        # Placeholder: будет реализовано в Phase 5
        # Нужно будет вызвать:
        # result = self.core_system.emit(
        #     event_type=event.event_type,
        #     vector=event.semantic.vector,
        #     priority=event.routing.priority
        # )
        # И обновить event.result = ProcessingResult(...)
        pass

    # ═══════════════════════════════════════════════════════════════════════════════
    # SENSOR MANAGEMENT
    # ═══════════════════════════════════════════════════════════════════════════════

    def register_sensor(self, **kwargs) -> SensorConfig:
        """Proxy to registry.register_sensor()."""
        return self.registry.register_sensor(**kwargs)

    def unregister_sensor(self, sensor_id: str) -> bool:
        """Proxy to registry.unregister_sensor()."""
        return self.registry.unregister_sensor(sensor_id)

    def list_sensors(self, **kwargs) -> List[SensorConfig]:
        """Proxy to registry.list_sensors()."""
        return self.registry.list_sensors(**kwargs)

    # ═══════════════════════════════════════════════════════════════════════════════
    # STATS
    # ═══════════════════════════════════════════════════════════════════════════════

    def get_stats(self) -> Dict[str, Any]:
        """
        Получить статистику Gateway.

        Returns:
            Dict с метриками
        """
        return {
            "total_events": self._total_events,
            "neuro_tick": self._neuro_tick,
            "registered_sensors": self.registry.count_sensors(),
            "enabled_sensors": self.registry.count_sensors(enabled_only=True),
        }


__all__ = [
    "SignalGateway",
]
