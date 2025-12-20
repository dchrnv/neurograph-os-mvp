# Signal Gateway v2.0 — Сенсорный интерфейс NeuroGraph OS

**Версия:** 2.0.0  
**Статус:** Спецификация для реализации  
**Дата:** 2025-12-20  
**Роль:** Единая точка входа для всех внешних и внутренних сигналов  
**Зависимости:** SignalSystem v1.0, Guardian v2.1, Grid v2.0, Python Bindings (PyO3)  
**Язык реализации:** Python (высокоуровневый интерфейс) + Rust Core (через FFI)

---

## 1. Философия и архитектурная концепция

### 1.1 Роль Gateway в системе

Gateway — это **сенсорный нерв** NeuroGraph OS. Его единственная задача — переводить сигналы из внешнего мира в язык, понятный ядру системы.

**Ключевые принципы:**

1. **Тонкий слой** — Gateway не содержит бизнес-логики, только трансляция
2. **Stateless** — никакого внутреннего состояния, контекст живёт в Grid
3. **Emit-only** — Gateway только эмитит события, не подписывается на ответы
4. **Native bindings** — прямой доступ к Rust Core через PyO3, минуя REST API

### 1.2 Архитектурная позиция

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ВНЕШНИЙ МИР                                    │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │
│  │ Telegram │ │  Аудио   │ │  Камера  │ │ Сенсоры  │ │  Таймер  │   ...    │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘          │
└───────┼────────────┼────────────┼────────────┼────────────┼─────────────────┘
        │            │            │            │            │
        ▼            ▼            ▼            ▼            ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SIGNAL GATEWAY v2.0                               │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Input Adapters                                │   │
│  │  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐        │   │
│  │  │TextAdapter │ │AudioAdapter│ │VisionAdapt │ │SystemAdapt │  ...   │   │
│  │  └─────┬──────┘ └─────┬──────┘ └─────┬──────┘ └─────┬──────┘        │   │
│  └────────┼──────────────┼──────────────┼──────────────┼───────────────┘   │
│           │              │              │              │                    │
│           ▼              ▼              ▼              ▼                    │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                      Normalization Pipeline                          │   │
│  │  [Raw Data] → [Encode 8D] → [Energy Profile] → [Validate] → [Emit]  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                    │                                        │
│                                    ▼                                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        Sensor Registry                               │   │
│  │  Динамическая регистрация сенсоров без изменения кода               │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ emit(SignalEvent)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                          RUST CORE (neurograph_core)                        │
│  ┌───────────────┐    ┌───────────────┐    ┌───────────────┐               │
│  │ SignalSystem  │───▶│     Grid      │───▶│   Guardian    │               │
│  │  (10K+ ev/s)  │    │  (8D Space)   │    │    (CDNA)     │               │
│  └───────────────┘    └───────────────┘    └───────────────┘               │
│           │                   │                    │                        │
│           ▼                   ▼                    ▼                        │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    Spreading Activation                              │   │
│  │                 Резонанс → Соседи → Паттерны                        │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
                    ▼               ▼               ▼
            ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
            │ TelegramBot │ │  Dashboard  │ │ActionSelect │
            │ (subscriber)│ │ (subscriber)│ │ (subscriber)│
            └─────────────┘ └─────────────┘ └─────────────┘
```

### 1.3 Почему не REST API?

| Аспект | REST API | Python Bindings (PyO3) |
|--------|----------|------------------------|
| Латентность | ~5ms | <0.1ms |
| Сериализация | JSON encode/decode | Zero-copy |
| Накладные расходы | HTTP stack | Прямой вызов |
| Для 100 сигналов/сек | 500ms overhead | 10ms overhead |

**Вывод:** Для «организма» с множеством сенсоров REST создаёт «тормозящее желе». Bindings дают 50× выигрыш в латентности.

---

## 2. Структуры данных

### 2.1 SignalEvent — Универсальный атом события

```python
from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Set, Any, Union, Tuple
from uuid import UUID, uuid4
from enum import Enum
import time


class SignalEvent(BaseModel):
    """
    Универсальный атом события в SignalSystem.
    Самодостаточный — содержит всё необходимое для обработки любым подписчиком.
    """
    
    # ═══════════════════════════════════════════════════════════════════════
    # ИДЕНТИФИКАЦИЯ
    # ═══════════════════════════════════════════════════════════════════════
    
    event_id: UUID = Field(default_factory=uuid4, description="Уникальный ID события")
    event_type: str = Field(..., description="Иерархический тип (см. таксономию)")
    schema_version: str = Field(default="2.0", description="Версия схемы для миграций")
    
    # ═══════════════════════════════════════════════════════════════════════
    # ИСТОЧНИК (Sensor Description)
    # ═══════════════════════════════════════════════════════════════════════
    
    source: "SignalSource" = Field(..., description="Кто породил сигнал")
    
    # ═══════════════════════════════════════════════════════════════════════
    # СЕМАНТИЧЕСКОЕ ЯДРО (8D Space)
    # ═══════════════════════════════════════════════════════════════════════
    
    semantic: "SemanticCore" = Field(..., description="8D координаты и семантика")
    
    # ═══════════════════════════════════════════════════════════════════════
    # ЭНЕРГЕТИЧЕСКИЙ ПРОФИЛЬ
    # ═══════════════════════════════════════════════════════════════════════
    
    energy: "EnergyProfile" = Field(..., description="Энергетические характеристики")
    
    # ═══════════════════════════════════════════════════════════════════════
    # ТЕМПОРАЛЬНАЯ ПРИВЯЗКА
    # ═══════════════════════════════════════════════════════════════════════
    
    temporal: "TemporalBinding" = Field(..., description="Временные координаты")
    
    # ═══════════════════════════════════════════════════════════════════════
    # СЫРЫЕ ДАННЫЕ
    # ═══════════════════════════════════════════════════════════════════════
    
    raw: "RawPayload" = Field(..., description="Оригинальные данные от сенсора")
    
    # ═══════════════════════════════════════════════════════════════════════
    # РЕЗУЛЬТАТ ОБРАБОТКИ (заполняется Rust Core)
    # ═══════════════════════════════════════════════════════════════════════
    
    result: Optional["ProcessingResult"] = Field(
        default=None, 
        description="Результат обработки ядром (Grid activation)"
    )
    
    # ═══════════════════════════════════════════════════════════════════════
    # МАРШРУТИЗАЦИЯ И ТРАССИРОВКА
    # ═══════════════════════════════════════════════════════════════════════
    
    routing: "RoutingInfo" = Field(..., description="Метаданные маршрутизации")


# ═══════════════════════════════════════════════════════════════════════════════
# ВЛОЖЕННЫЕ СТРУКТУРЫ
# ═══════════════════════════════════════════════════════════════════════════════


class SignalSource(BaseModel):
    """Описание источника сигнала (сенсора)."""
    
    # Классификация
    domain: str = Field(..., description="external | internal | system")
    modality: str = Field(..., description="text | audio | vision | haptic | proprioception | chemical | environment | state")
    
    # Идентификация сенсора
    sensor_id: str = Field(..., description="Уникальный ID: telegram_bot_01, camera_front, mic_array_left")
    sensor_type: str = Field(..., description="Тип: chat_interface | microphone | camera | thermometer | custom")
    sensor_meta: Dict[str, Any] = Field(default_factory=dict, description="Произвольные метаданные")
    
    # Качество сигнала
    confidence: float = Field(default=1.0, ge=0.0, le=1.0, description="Уверенность сенсора в данных")
    noise_level: float = Field(default=0.0, ge=0.0, le=1.0, description="Уровень шума")
    calibration_state: str = Field(default="calibrated", description="calibrated | uncalibrated | degraded")


class SemanticCore(BaseModel):
    """Семантическое ядро сигнала в 8D пространстве."""
    
    # Основной вектор
    vector: List[float] = Field(..., min_length=8, max_length=8, description="8D координаты")
    
    # Разложение по семантическим слоям
    layer_decomposition: Optional["LayerDecomposition"] = Field(
        default=None, 
        description="Вклад каждого из 8 слоёв"
    )
    
    # Неопределённость
    uncertainty: Optional[List[float]] = Field(
        default=None, 
        min_length=8, 
        max_length=8,
        description="Погрешность по каждой оси"
    )
    
    # Метод кодирования
    encoding_method: str = Field(
        default="pca_projection", 
        description="pca_projection | transformer | direct | learned | passthrough"
    )


class LayerDecomposition(BaseModel):
    """
    Разложение семантики по 8 когнитивным слоям.
    Значения показывают вклад каждого слоя в общий смысл сигнала.
    """
    physical: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 0: физические свойства")
    spatial: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 1: пространственные отношения")
    temporal: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 2: временные отношения")
    causal: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 3: причинно-следственные связи")
    emotional: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 4: эмоциональная окраска")
    social: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 5: социальный контекст")
    abstract: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 6: абстрактные концепции")
    meta: float = Field(default=0.0, ge=0.0, le=1.0, description="Слой 7: метакогнитивный уровень")


class EnergyProfile(BaseModel):
    """Энергетический профиль сигнала."""
    
    # Базовые характеристики
    magnitude: int = Field(default=0, ge=-32768, le=32767, description="i16: сила/интенсивность")
    valence: float = Field(default=0.0, ge=-1.0, le=1.0, description="Эмоциональная окраска (-негатив, +позитив)")
    arousal: float = Field(default=0.5, ge=0.0, le=1.0, description="Уровень возбуждения")
    urgency: float = Field(default=0.5, ge=0.0, le=1.0, description="Срочность обработки")
    
    # ADSR-огибающая (как в синтезаторах)
    attack: float = Field(default=0.5, ge=0.0, le=1.0, description="Скорость нарастания")
    decay: float = Field(default=0.3, ge=0.0, le=1.0, description="Скорость затухания")
    sustain: float = Field(default=0.5, ge=0.0, le=1.0, description="Уровень удержания")


class TemporalBinding(BaseModel):
    """Темпоральная привязка сигнала."""
    
    # Внешнее время
    timestamp: float = Field(default_factory=time.time, description="Unix timestamp создания")
    duration: Optional[float] = Field(default=None, description="Длительность события (для аудио/видео)")
    
    # Внутреннее время (NeuroTime)
    neuro_tick: Optional[int] = Field(default=None, description="Внутренний тик системы")
    subjective_now: Optional[float] = Field(default=None, description="Позиция в субъективном 'сейчас'")
    
    # Контекст последовательности
    sequence_id: Optional[UUID] = Field(default=None, description="ID последовательности (диалог, сессия)")
    sequence_position: Optional[int] = Field(default=None, description="Позиция в последовательности")


class RawPayload(BaseModel):
    """Сырые данные от сенсора."""
    
    # Тип и содержимое
    data_type: str = Field(..., description="text | float_array | image | blob | structured")
    data: Union[str, List[float], bytes, Dict[str, Any]] = Field(..., description="Данные")
    encoding: Optional[str] = Field(default=None, description="utf-8 | base64 | float32")
    compression: Optional[str] = Field(default=None, description="none | gzip | lz4")
    checksum: Optional[str] = Field(default=None, description="Для верификации целостности")
    
    # Медиа-специфичные поля
    sample_rate: Optional[int] = Field(default=None, description="Для аудио (Hz)")
    resolution: Optional[Tuple[int, int]] = Field(default=None, description="Для изображений (W, H)")
    channels: Optional[int] = Field(default=None, description="Количество каналов")


class ProcessingResult(BaseModel):
    """Результат обработки сигнала ядром (заполняется Rust Core)."""
    
    # Созданный токен
    token_id: int = Field(..., description="ID токена в Grid")
    
    # Активация
    neighbors: List["NeighborInfo"] = Field(default_factory=list, description="Активированные соседи")
    
    # Энергетические изменения
    energy_delta: float = Field(default=0.0, description="Изменение общей энергии системы")
    activation_spread: int = Field(default=0, description="Количество затронутых токенов")
    
    # Флаги
    is_novel: bool = Field(default=False, description="Сигнал не имел близких соседей")
    triggered_actions: List[int] = Field(default_factory=list, description="ID токенов-действий")
    anomaly_score: float = Field(default=0.0, ge=0.0, le=1.0, description="Степень аномальности")
    
    # Производительность
    processing_time_us: int = Field(default=0, description="Время обработки (микросекунды)")


class NeighborInfo(BaseModel):
    """Информация об активированном соседе."""
    
    token_id: int = Field(..., description="ID токена")
    distance: float = Field(..., description="Расстояние в 8D пространстве")
    resonance: float = Field(..., description="Сила резонанса")
    token_type: str = Field(..., description="concept | action | memory | emotion | sensor | system")
    layer_affinity: int = Field(default=0, ge=0, le=7, description="Доминирующий семантический слой")


class RoutingInfo(BaseModel):
    """Метаданные маршрутизации и трассировки."""
    
    # Приоритет и TTL
    priority: int = Field(default=128, ge=0, le=255, description="Приоритет обработки")
    ttl: int = Field(default=10, ge=0, description="Time-to-live (хопов)")
    
    # Трассировка
    trace_id: Optional[UUID] = Field(default=None, description="ID сквозной трассировки")
    parent_event_id: Optional[UUID] = Field(default=None, description="Родительское событие")
    hop_count: int = Field(default=0, description="Количество обработавших модулей")
    
    # Теги для фильтрации
    tags: Set[str] = Field(default_factory=set, description="Произвольные теги")
    
    # Целевая маршрутизация
    target_subscribers: Optional[List[str]] = Field(default=None, description="Конкретные подписчики")
    exclude_subscribers: Optional[List[str]] = Field(default=None, description="Исключить подписчиков")


# Обновляем forward references
SignalEvent.model_rebuild()
```

---

## 3. Таксономия событий (Event Type Hierarchy)

```
signal.
├── input.                              # Входящие сигналы
│   ├── external.                       # Из внешнего мира
│   │   ├── text.                       # Текстовый ввод
│   │   │   ├── chat                    # Сообщение в чате
│   │   │   ├── command                 # Команда
│   │   │   ├── query                   # Вопрос
│   │   │   └── transcription           # Расшифровка речи
│   │   │
│   │   ├── audio.                      # Аудио сигналы
│   │   │   ├── speech                  # Распознанная речь
│   │   │   ├── music                   # Музыка
│   │   │   ├── ambient                 # Окружающие звуки
│   │   │   ├── alert                   # Звуковой сигнал/будильник
│   │   │   └── noise                   # Шум (для анализа)
│   │   │
│   │   ├── vision.                     # Визуальные сигналы
│   │   │   ├── object                  # Распознанный объект
│   │   │   ├── face                    # Лицо
│   │   │   ├── gesture                 # Жест
│   │   │   ├── scene                   # Сцена целиком
│   │   │   ├── text_ocr                # Текст с изображения
│   │   │   ├── qr_code                 # QR/штрих-код
│   │   │   └── motion                  # Детекция движения
│   │   │
│   │   ├── haptic.                     # Тактильные сигналы
│   │   │   ├── touch                   # Прикосновение
│   │   │   ├── pressure                # Давление
│   │   │   ├── vibration               # Вибрация
│   │   │   ├── texture                 # Текстура
│   │   │   └── temperature             # Температура поверхности
│   │   │
│   │   ├── proprioception.             # Положение в пространстве
│   │   │   ├── position                # Позиция
│   │   │   ├── orientation             # Ориентация
│   │   │   ├── motion                  # Движение
│   │   │   ├── acceleration            # Ускорение
│   │   │   └── balance                 # Баланс/равновесие
│   │   │
│   │   ├── chemical.                   # Химические сигналы
│   │   │   ├── smell                   # Запах
│   │   │   ├── taste                   # Вкус
│   │   │   ├── air_quality             # Качество воздуха
│   │   │   └── gas_detection           # Детекция газов
│   │   │
│   │   └── environment.                # Параметры среды
│   │       ├── temperature             # Температура воздуха
│   │       ├── humidity                # Влажность
│   │       ├── pressure                # Атмосферное давление
│   │       ├── light                   # Освещённость
│   │       ├── uv                      # UV-индекс
│   │       ├── radiation               # Радиация
│   │       ├── magnetic                # Магнитное поле
│   │       └── gps                     # GPS координаты
│   │
│   ├── internal.                       # Внутренние сигналы
│   │   ├── thought                     # Мысль от другого модуля
│   │   ├── memory_recall               # Всплывшее воспоминание
│   │   ├── emotion                     # Эмоциональный сигнал
│   │   ├── prediction                  # Предсказание
│   │   ├── hypothesis                  # Гипотеза
│   │   ├── question                    # Внутренний вопрос
│   │   ├── goal                        # Цель/намерение
│   │   ├── conflict                    # Когнитивный конфликт
│   │   └── dream                       # Сигнал из «сновидения»
│   │
│   └── system.                         # Системные сигналы
│       ├── resource.                   # Состояние ресурсов
│       │   ├── memory_low              # Мало памяти
│       │   ├── memory_critical         # Критически мало памяти
│       │   ├── cpu_high                # Высокая нагрузка CPU
│       │   ├── disk_low                # Мало места на диске
│       │   ├── energy_low              # Низкий заряд батареи
│       │   └── network_degraded        # Проблемы с сетью
│       │
│       ├── lifecycle.                  # Жизненный цикл
│       │   ├── startup                 # Запуск системы
│       │   ├── shutdown                # Завершение работы
│       │   ├── checkpoint              # Контрольная точка
│       │   ├── sleep                   # Переход в спящий режим
│       │   ├── wake                    # Пробуждение
│       │   └── maintenance             # Режим обслуживания
│       │
│       ├── error.                      # Ошибки
│       │   ├── sensor_failure          # Сбой сенсора
│       │   ├── sensor_timeout          # Таймаут сенсора
│       │   ├── encoding_error          # Ошибка кодирования
│       │   ├── validation_error        # Ошибка валидации
│       │   ├── core_error              # Ошибка ядра
│       │   └── recovery                # Восстановление после ошибки
│       │
│       └── timer.                      # Таймеры
│           ├── tick                    # Регулярный тик
│           ├── heartbeat               # Heartbeat системы
│           ├── scheduled               # Запланированное событие
│           ├── reminder                # Напоминание
│           └── timeout                 # Таймаут операции
│
├── activation.                         # Результаты активации Grid
│   ├── resonance                       # Найден резонанс с соседями
│   ├── action_trigger                  # Сработал токен-действие
│   ├── memory_association              # Ассоциация с памятью
│   ├── pattern_match                   # Совпадение с паттерном
│   ├── cascade                         # Каскадная активация
│   └── convergence                     # Схождение нескольких путей
│
├── anomaly.                            # Аномалии
│   ├── novelty                         # Новый сигнал без соседей
│   ├── contradiction                   # Противоречие с существующим
│   ├── outlier                         # Статистический выброс
│   ├── unexpected                      # Неожиданное поведение
│   ├── drift                           # Дрейф параметров
│   └── intrusion                       # Подозрительный сигнал
│
└── meta.                               # Мета-события
    ├── subscription.                   # Подписки
    │   ├── added                       # Добавлен подписчик
    │   └── removed                     # Удалён подписчик
    │
    ├── sensor.                         # Сенсоры
    │   ├── registered                  # Зарегистрирован сенсор
    │   ├── unregistered                # Отключён сенсор
    │   ├── calibrated                  # Откалиброван сенсор
    │   └── degraded                    # Деградация сенсора
    │
    └── config.                         # Конфигурация
        ├── changed                     # Изменена конфигурация
        ├── reloaded                    # Перезагружена конфигурация
        └── validated                   # Валидирована конфигурация
```

---

## 4. Система регистрации сенсоров

### 4.1 Реестр сенсоров (SensorRegistry)

```python
from typing import Dict, Optional, Callable, Any
from dataclasses import dataclass, field
from enum import Enum
import uuid


class EncoderType(Enum):
    """Типы энкодеров для преобразования в 8D."""
    TEXT_TRANSFORMER = "text_transformer"     # Через Transformer → PCA
    TEXT_TFIDF = "text_tfidf"                 # TF-IDF → PCA
    AUDIO_MEL = "audio_mel"                   # Mel-спектрограмма → PCA
    AUDIO_MFCC = "audio_mfcc"                 # MFCC → PCA
    IMAGE_CNN = "image_cnn"                   # CNN embeddings → PCA
    IMAGE_CLIP = "image_clip"                 # CLIP embeddings → PCA
    NUMERIC_DIRECT = "numeric_direct"         # Прямое отображение
    PASSTHROUGH = "passthrough"               # Без преобразования (уже 8D)
    CUSTOM = "custom"                         # Пользовательский энкодер


@dataclass
class EncoderConfig:
    """Конфигурация энкодера для сенсора."""
    
    encoder_type: EncoderType
    model_id: Optional[str] = None              # ID модели (для transformer/cnn)
    output_dim: int = 8                         # Размерность выхода
    normalize: bool = True                      # Нормализовать выход
    
    # Для кастомных энкодеров
    custom_encoder: Optional[Callable[[Any], list]] = None
    
    # PCA-проектор (если нужен)
    pca_components: Optional[Any] = None        # sklearn.decomposition.PCA
    
    # Кэширование
    cache_embeddings: bool = True
    cache_size: int = 10000


@dataclass
class EnergyDefaults:
    """Дефолтные энергетические параметры для сенсора."""
    
    base_magnitude: int = 0
    base_valence: float = 0.0
    base_arousal: float = 0.5
    base_urgency: float = 0.5
    
    # Модификаторы на основе свойств сигнала
    magnitude_from_confidence: bool = True      # magnitude = confidence * scale
    valence_from_sentiment: bool = True         # Для текста: sentiment → valence
    urgency_from_noise: bool = False            # Шумный сигнал = срочный


@dataclass
class SensorConfig:
    """Полная конфигурация сенсора."""
    
    # Идентификация
    sensor_type: str                            # Тип из известных или "custom"
    modality: str                               # Модальность
    domain: str = "external"                    # "external" | "internal" | "system"
    
    # Энкодер
    encoder: EncoderConfig = field(default_factory=lambda: EncoderConfig(
        encoder_type=EncoderType.PASSTHROUGH
    ))
    
    # Энергия
    energy_defaults: EnergyDefaults = field(default_factory=EnergyDefaults)
    
    # Метаданные
    name: str = ""                              # Человекочитаемое имя
    description: str = ""
    version: str = "1.0"
    tags: set = field(default_factory=set)
    
    # Ограничения
    max_rate_hz: float = 100.0                  # Максимальная частота сигналов
    min_confidence: float = 0.0                 # Минимальная уверенность для эмита
    require_calibration: bool = False           # Требовать калибровку


class SensorRegistry:
    """
    Реестр сенсоров.
    Позволяет динамически регистрировать новые сенсоры без изменения кода Gateway.
    """
    
    def __init__(self):
        self._sensors: Dict[str, SensorConfig] = {}
        self._encoders: Dict[str, Callable] = {}
        self._stats: Dict[str, dict] = {}
        
        # Регистрируем встроенные сенсоры
        self._register_builtin_sensors()
    
    def register(self, config: SensorConfig, sensor_id: Optional[str] = None) -> str:
        """
        Регистрирует новый сенсор.
        
        Args:
            config: Конфигурация сенсора
            sensor_id: Опциональный ID (генерируется если не указан)
            
        Returns:
            sensor_id: Уникальный идентификатор сенсора
        """
        if sensor_id is None:
            sensor_id = f"{config.sensor_type}_{uuid.uuid4().hex[:8]}"
        
        if sensor_id in self._sensors:
            raise ValueError(f"Sensor {sensor_id} already registered")
        
        self._sensors[sensor_id] = config
        self._stats[sensor_id] = {
            "signals_emitted": 0,
            "last_signal_at": None,
            "avg_processing_time_us": 0,
            "errors": 0
        }
        
        # Инициализируем энкодер
        self._init_encoder(sensor_id, config.encoder)
        
        return sensor_id
    
    def unregister(self, sensor_id: str) -> bool:
        """Отключает сенсор."""
        if sensor_id not in self._sensors:
            return False
        
        del self._sensors[sensor_id]
        del self._encoders[sensor_id]
        del self._stats[sensor_id]
        return True
    
    def get_config(self, sensor_id: str) -> Optional[SensorConfig]:
        """Возвращает конфигурацию сенсора."""
        return self._sensors.get(sensor_id)
    
    def get_encoder(self, sensor_id: str) -> Optional[Callable]:
        """Возвращает энкодер для сенсора."""
        return self._encoders.get(sensor_id)
    
    def list_sensors(self, 
                     domain: Optional[str] = None,
                     modality: Optional[str] = None) -> list:
        """Список зарегистрированных сенсоров с фильтрацией."""
        result = []
        for sensor_id, config in self._sensors.items():
            if domain and config.domain != domain:
                continue
            if modality and config.modality != modality:
                continue
            result.append({
                "sensor_id": sensor_id,
                "config": config,
                "stats": self._stats[sensor_id]
            })
        return result
    
    def _init_encoder(self, sensor_id: str, config: EncoderConfig):
        """Инициализирует энкодер для сенсора."""
        if config.encoder_type == EncoderType.CUSTOM:
            if config.custom_encoder is None:
                raise ValueError("Custom encoder requires custom_encoder function")
            self._encoders[sensor_id] = config.custom_encoder
        elif config.encoder_type == EncoderType.PASSTHROUGH:
            self._encoders[sensor_id] = lambda x: x if len(x) == 8 else [0.0] * 8
        else:
            # Lazy loading для тяжёлых моделей
            self._encoders[sensor_id] = self._create_encoder(config)
    
    def _create_encoder(self, config: EncoderConfig) -> Callable:
        """Создаёт энкодер на основе конфигурации."""
        # Placeholder — реализация зависит от доступных моделей
        def encoder(data) -> list:
            # TODO: Реализовать для каждого типа энкодера
            return [0.0] * 8
        return encoder
    
    def _register_builtin_sensors(self):
        """Регистрирует встроенные типы сенсоров."""
        
        # Текстовый чат
        self.register(
            SensorConfig(
                sensor_type="chat_interface",
                modality="text",
                domain="external",
                encoder=EncoderConfig(
                    encoder_type=EncoderType.TEXT_TRANSFORMER,
                    model_id="sentence-transformers/all-MiniLM-L6-v2",
                    output_dim=8,
                    normalize=True
                ),
                energy_defaults=EnergyDefaults(
                    base_magnitude=100,
                    valence_from_sentiment=True
                ),
                name="Text Chat Interface",
                description="Standard text input from chat interfaces",
                tags={"user_facing", "interactive"}
            ),
            sensor_id="builtin_text_chat"
        )
        
        # Системный монитор
        self.register(
            SensorConfig(
                sensor_type="system_monitor",
                modality="state",
                domain="system",
                encoder=EncoderConfig(
                    encoder_type=EncoderType.NUMERIC_DIRECT,
                    output_dim=8
                ),
                energy_defaults=EnergyDefaults(
                    base_magnitude=50,
                    base_urgency=0.3
                ),
                name="System Resource Monitor",
                description="CPU, memory, disk monitoring",
                tags={"system", "monitoring"},
                max_rate_hz=1.0  # Раз в секунду достаточно
            ),
            sensor_id="builtin_system_monitor"
        )
        
        # Таймер
        self.register(
            SensorConfig(
                sensor_type="timer",
                modality="state",
                domain="system",
                encoder=EncoderConfig(
                    encoder_type=EncoderType.PASSTHROUGH
                ),
                energy_defaults=EnergyDefaults(
                    base_magnitude=10,
                    base_urgency=0.1
                ),
                name="System Timer",
                description="Periodic ticks and scheduled events",
                tags={"system", "timer"}
            ),
            sensor_id="builtin_timer"
        )
```

---

## 5. Система фильтров для подписчиков

### 5.1 Язык фильтров

```python
from typing import Any, Dict, List, Union
from dataclasses import dataclass
import re


@dataclass
class FilterExpression:
    """
    Выражение фильтра для подписки на события.
    
    Поддерживаемые операторы:
        $eq      - равенство (по умолчанию)
        $ne      - не равно
        $gt      - больше
        $gte     - больше или равно
        $lt      - меньше
        $lte     - меньше или равно
        $in      - входит в список
        $nin     - не входит в список
        $contains - содержит (для строк и множеств)
        $regex   - регулярное выражение
        $exists  - поле существует
        $wildcard - glob-паттерн (*, ?)
    
    Логические операторы:
        $and     - логическое И
        $or      - логическое ИЛИ
        $not     - логическое НЕ
    """
    
    field: str
    operator: str
    value: Any


class SubscriptionFilter:
    """
    Фильтр подписки на события SignalSystem.
    """
    
    def __init__(self, filter_dict: Dict[str, Any]):
        """
        Args:
            filter_dict: Словарь с условиями фильтрации
            
        Examples:
            # Простой фильтр по типу
            {"event_type": "signal.input.external.text.chat"}
            
            # С wildcard
            {"event_type": {"$wildcard": "signal.input.external.*"}}
            
            # Комбинированный
            {
                "event_type": {"$wildcard": "signal.input.*"},
                "source.confidence": {"$gte": 0.8},
                "routing.tags": {"$contains": "user_facing"}
            }
            
            # С логическими операторами
            {
                "$or": [
                    {"event_type": "signal.activation.action_trigger"},
                    {"result.is_novel": True}
                ]
            }
        """
        self._filter = filter_dict
        self._compiled = self._compile(filter_dict)
    
    def matches(self, event: "SignalEvent") -> bool:
        """Проверяет, соответствует ли событие фильтру."""
        return self._evaluate(self._compiled, event)
    
    def _compile(self, filter_dict: Dict) -> Any:
        """Компилирует фильтр для быстрого выполнения."""
        # Предкомпиляция регулярных выражений и wildcard-паттернов
        compiled = {}
        for key, value in filter_dict.items():
            if key.startswith("$"):
                # Логический оператор
                if key in ("$and", "$or"):
                    compiled[key] = [self._compile(v) for v in value]
                elif key == "$not":
                    compiled[key] = self._compile(value)
            elif isinstance(value, dict):
                # Оператор сравнения
                for op, val in value.items():
                    if op == "$regex":
                        compiled[key] = {"$regex": re.compile(val)}
                    elif op == "$wildcard":
                        # Конвертируем glob в regex
                        regex = val.replace(".", r"\.").replace("*", ".*").replace("?", ".")
                        compiled[key] = {"$wildcard": re.compile(f"^{regex}$")}
                    else:
                        compiled[key] = {op: val}
            else:
                # Простое равенство
                compiled[key] = {"$eq": value}
        return compiled
    
    def _evaluate(self, compiled: Dict, event: "SignalEvent") -> bool:
        """Вычисляет скомпилированный фильтр."""
        for key, condition in compiled.items():
            if key == "$and":
                if not all(self._evaluate(c, event) for c in condition):
                    return False
            elif key == "$or":
                if not any(self._evaluate(c, event) for c in condition):
                    return False
            elif key == "$not":
                if self._evaluate(condition, event):
                    return False
            else:
                # Получаем значение поля
                field_value = self._get_field(event, key)
                if not self._match_condition(field_value, condition):
                    return False
        return True
    
    def _get_field(self, event: "SignalEvent", path: str) -> Any:
        """Получает значение поля по пути (поддержка вложенности через точку)."""
        obj = event
        for part in path.split("."):
            if hasattr(obj, part):
                obj = getattr(obj, part)
            elif isinstance(obj, dict) and part in obj:
                obj = obj[part]
            else:
                return None
        return obj
    
    def _match_condition(self, value: Any, condition: Dict) -> bool:
        """Проверяет условие для значения."""
        for op, expected in condition.items():
            if op == "$eq":
                if value != expected:
                    return False
            elif op == "$ne":
                if value == expected:
                    return False
            elif op == "$gt":
                if not (value is not None and value > expected):
                    return False
            elif op == "$gte":
                if not (value is not None and value >= expected):
                    return False
            elif op == "$lt":
                if not (value is not None and value < expected):
                    return False
            elif op == "$lte":
                if not (value is not None and value <= expected):
                    return False
            elif op == "$in":
                if value not in expected:
                    return False
            elif op == "$nin":
                if value in expected:
                    return False
            elif op == "$contains":
                if isinstance(value, (set, list)):
                    if expected not in value:
                        return False
                elif isinstance(value, str):
                    if expected not in value:
                        return False
                else:
                    return False
            elif op == "$regex":
                if not (isinstance(value, str) and expected.match(value)):
                    return False
            elif op == "$wildcard":
                if not (isinstance(value, str) and expected.match(value)):
                    return False
            elif op == "$exists":
                if expected and value is None:
                    return False
                if not expected and value is not None:
                    return False
        return True


# Примеры фильтров
FILTER_EXAMPLES = {
    # Telegram бот — только текст из чата
    "telegram_bot": {
        "event_type": {"$wildcard": "signal.input.external.text.*"},
        "source.sensor_id": {"$wildcard": "telegram_*"}
    },
    
    # Dashboard — все входящие с высоким приоритетом
    "dashboard_priority": {
        "event_type": {"$wildcard": "signal.input.*"},
        "routing.priority": {"$gte": 200}
    },
    
    # ActionSelector — активации с триггерами действий
    "action_selector": {
        "event_type": "signal.activation.action_trigger"
    },
    
    # Anomaly detector — все аномалии
    "anomaly_detector": {
        "event_type": {"$wildcard": "signal.anomaly.*"}
    },
    
    # По модальности — всё аудио
    "audio_processor": {
        "source.modality": "audio"
    },
    
    # По слою семантики — эмоционально окрашенные
    "emotion_tracker": {
        "semantic.layer_decomposition.emotional": {"$gte": 0.7}
    },
    
    # Комбинированный — срочные внешние сигналы от надёжных сенсоров
    "urgent_external": {
        "event_type": {"$wildcard": "signal.input.external.*"},
        "source.confidence": {"$gte": 0.8},
        "energy.urgency": {"$gte": 0.7},
        "routing.tags": {"$contains": "user_facing"}
    },
    
    # Логический OR — действия или новизна
    "action_or_novelty": {
        "$or": [
            {"event_type": "signal.activation.action_trigger"},
            {"result.is_novel": True}
        ]
    }
}
```

---

## 6. Основной класс Gateway

```python
import time
from typing import Optional, Dict, Any, Callable
from uuid import UUID
import logging

# Rust bindings (PyO3)
import neurograph_core


class SignalGateway:
    """
    Signal Gateway v2.0 — Единая точка входа для всех сигналов NeuroGraph OS.
    
    Принципы:
        - Тонкий слой: только трансляция, без бизнес-логики
        - Stateless: контекст живёт в Grid, не в Gateway
        - Emit-only: Gateway эмитит события, не подписывается на ответы
        - Native bindings: прямой доступ к Rust Core через PyO3
    
    Usage:
        gateway = SignalGateway()
        
        # Регистрация сенсора
        sensor_id = gateway.register_sensor(SensorConfig(...))
        
        # Отправка сигнала
        event = gateway.push_text("Привет, Оли!", source="telegram")
        
        # Отправка от конкретного сенсора  
        event = gateway.push(
            sensor_id="camera_front",
            data=image_vector,
            event_type="signal.input.external.vision.scene"
        )
    """
    
    def __init__(self, 
                 runtime: Optional[neurograph_core.Runtime] = None,
                 config: Optional[Dict] = None):
        """
        Args:
            runtime: Инстанс Rust Runtime (создаётся если не передан)
            config: Конфигурация Gateway
        """
        self._runtime = runtime or neurograph_core.Runtime()
        self._config = config or {}
        self._registry = SensorRegistry()
        self._logger = logging.getLogger("neurograph.gateway")
        
        # Счётчики для метрик
        self._stats = {
            "total_signals": 0,
            "signals_by_type": {},
            "avg_processing_time_us": 0,
            "errors": 0
        }
    
    # ═══════════════════════════════════════════════════════════════════════
    # ПУБЛИЧНЫЙ API
    # ═══════════════════════════════════════════════════════════════════════
    
    def push_text(self, 
                  text: str, 
                  source: str = "user_chat",
                  **kwargs) -> SignalEvent:
        """
        Отправляет текстовый сигнал в систему.
        
        Args:
            text: Текст сообщения
            source: Идентификатор источника (для routing)
            **kwargs: Дополнительные параметры (priority, tags, etc.)
            
        Returns:
            SignalEvent с заполненным result после обработки ядром
        """
        sensor_id = self._resolve_sensor(source, "text")
        return self._process_signal(
            sensor_id=sensor_id,
            raw_data=text,
            data_type="text",
            event_type="signal.input.external.text.chat",
            **kwargs
        )
    
    def push_audio(self,
                   audio_vector: list,
                   source: str = "microphone",
                   **kwargs) -> SignalEvent:
        """
        Отправляет аудио-сигнал (уже обработанный вектор).
        
        Args:
            audio_vector: Вектор признаков аудио
            source: Идентификатор источника
            **kwargs: Дополнительные параметры
            
        Returns:
            SignalEvent с результатом обработки
        """
        sensor_id = self._resolve_sensor(source, "audio")
        return self._process_signal(
            sensor_id=sensor_id,
            raw_data=audio_vector,
            data_type="float_array",
            event_type="signal.input.external.audio.ambient",
            **kwargs
        )
    
    def push_vision(self,
                    image_vector: list,
                    source: str = "camera",
                    vision_type: str = "scene",
                    **kwargs) -> SignalEvent:
        """
        Отправляет визуальный сигнал.
        
        Args:
            image_vector: Вектор признаков изображения
            source: Идентификатор камеры/источника
            vision_type: Тип (scene, object, face, gesture, etc.)
            **kwargs: Дополнительные параметры
            
        Returns:
            SignalEvent с результатом обработки
        """
        sensor_id = self._resolve_sensor(source, "vision")
        return self._process_signal(
            sensor_id=sensor_id,
            raw_data=image_vector,
            data_type="float_array",
            event_type=f"signal.input.external.vision.{vision_type}",
            **kwargs
        )
    
    def push_system(self,
                    event_subtype: str,
                    data: Dict[str, Any],
                    **kwargs) -> SignalEvent:
        """
        Отправляет системный сигнал.
        
        Args:
            event_subtype: Подтип события (resource.memory_low, lifecycle.startup, etc.)
            data: Данные события
            **kwargs: Дополнительные параметры
            
        Returns:
            SignalEvent с результатом обработки
        """
        return self._process_signal(
            sensor_id="builtin_system_monitor",
            raw_data=data,
            data_type="structured",
            event_type=f"signal.input.system.{event_subtype}",
            **kwargs
        )
    
    def push(self,
             sensor_id: str,
             data: Any,
             event_type: str,
             **kwargs) -> SignalEvent:
        """
        Универсальный метод отправки сигнала от любого сенсора.
        
        Args:
            sensor_id: ID зарегистрированного сенсора
            data: Сырые данные
            event_type: Полный тип события
            **kwargs: Дополнительные параметры
            
        Returns:
            SignalEvent с результатом обработки
        """
        config = self._registry.get_config(sensor_id)
        if config is None:
            raise ValueError(f"Unknown sensor: {sensor_id}")
        
        data_type = self._infer_data_type(data)
        return self._process_signal(
            sensor_id=sensor_id,
            raw_data=data,
            data_type=data_type,
            event_type=event_type,
            **kwargs
        )
    
    # ═══════════════════════════════════════════════════════════════════════
    # УПРАВЛЕНИЕ СЕНСОРАМИ
    # ═══════════════════════════════════════════════════════════════════════
    
    def register_sensor(self, 
                        config: SensorConfig,
                        sensor_id: Optional[str] = None) -> str:
        """
        Регистрирует новый сенсор.
        
        Args:
            config: Конфигурация сенсора
            sensor_id: Опциональный ID
            
        Returns:
            sensor_id: Присвоенный идентификатор
        """
        sensor_id = self._registry.register(config, sensor_id)
        
        # Эмитим мета-событие
        self._emit_meta_event("signal.meta.sensor.registered", {
            "sensor_id": sensor_id,
            "config": config.__dict__
        })
        
        return sensor_id
    
    def unregister_sensor(self, sensor_id: str) -> bool:
        """Отключает сенсор."""
        success = self._registry.unregister(sensor_id)
        if success:
            self._emit_meta_event("signal.meta.sensor.unregistered", {
                "sensor_id": sensor_id
            })
        return success
    
    def list_sensors(self, **filters) -> list:
        """Возвращает список сенсоров с фильтрацией."""
        return self._registry.list_sensors(**filters)
    
    # ═══════════════════════════════════════════════════════════════════════
    # СТАТИСТИКА
    # ═══════════════════════════════════════════════════════════════════════
    
    def get_stats(self) -> Dict:
        """Возвращает статистику работы Gateway."""
        return {
            **self._stats,
            "sensors": self._registry.list_sensors()
        }
    
    # ═══════════════════════════════════════════════════════════════════════
    # ВНУТРЕННЯЯ ЛОГИКА
    # ═══════════════════════════════════════════════════════════════════════
    
    def _process_signal(self,
                        sensor_id: str,
                        raw_data: Any,
                        data_type: str,
                        event_type: str,
                        **kwargs) -> SignalEvent:
        """
        Основной pipeline обработки сигнала.
        
        Pipeline:
            1. Получить конфиг сенсора
            2. Нормализовать данные
            3. Закодировать в 8D
            4. Вычислить энергию
            5. Собрать SignalEvent
            6. Валидировать через Guardian
            7. Эмитить в SignalSystem (Rust Core)
            8. Получить результат активации
            9. Вернуть полный SignalEvent
        """
        start_time = time.perf_counter_ns()
        
        try:
            # 1. Получить конфиг
            config = self._registry.get_config(sensor_id)
            if config is None:
                raise ValueError(f"Unknown sensor: {sensor_id}")
            
            # 2-3. Нормализовать и закодировать
            vector_8d = self._encode(sensor_id, raw_data, config)
            
            # 4. Вычислить энергию
            energy_profile = self._compute_energy(raw_data, config, kwargs)
            
            # 5. Собрать SignalEvent
            event = SignalEvent(
                event_type=event_type,
                source=SignalSource(
                    domain=config.domain,
                    modality=config.modality,
                    sensor_id=sensor_id,
                    sensor_type=config.sensor_type,
                    sensor_meta=config.__dict__.get("tags", {}),
                    confidence=kwargs.get("confidence", 1.0),
                    noise_level=kwargs.get("noise_level", 0.0),
                    calibration_state="calibrated"
                ),
                semantic=SemanticCore(
                    vector=vector_8d,
                    encoding_method=config.encoder.encoder_type.value
                ),
                energy=energy_profile,
                temporal=TemporalBinding(
                    timestamp=time.time(),
                    sequence_id=kwargs.get("sequence_id"),
                    sequence_position=kwargs.get("sequence_position")
                ),
                raw=RawPayload(
                    data_type=data_type,
                    data=raw_data if data_type != "blob" else "<binary>",
                    encoding="utf-8" if data_type == "text" else None
                ),
                routing=RoutingInfo(
                    priority=kwargs.get("priority", 128),
                    tags=set(kwargs.get("tags", [])),
                    trace_id=kwargs.get("trace_id"),
                    parent_event_id=kwargs.get("parent_event_id")
                )
            )
            
            # 6. Валидировать через Guardian
            validation = self._runtime.guardian.validate_signal(event.model_dump())
            if not validation.is_valid:
                self._logger.warning(f"Signal validation failed: {validation.reason}")
                self._stats["errors"] += 1
                # Продолжаем с пониженным приоритетом
                event.routing.priority = min(event.routing.priority, 50)
            
            # 7-8. Эмитить в SignalSystem и получить результат
            result = self._emit_to_core(event)
            event.result = result
            
            # 9. Обновить статистику
            processing_time = (time.perf_counter_ns() - start_time) // 1000
            self._update_stats(event_type, processing_time)
            
            return event
            
        except Exception as e:
            self._logger.error(f"Error processing signal: {e}")
            self._stats["errors"] += 1
            raise
    
    def _encode(self, 
                sensor_id: str, 
                data: Any, 
                config: SensorConfig) -> list:
        """Кодирует данные в 8D вектор."""
        encoder = self._registry.get_encoder(sensor_id)
        if encoder is None:
            raise ValueError(f"No encoder for sensor: {sensor_id}")
        
        # Получаем высокоразмерный вектор
        raw_vector = encoder(data)
        
        # Проецируем в 8D если нужно
        if len(raw_vector) != 8:
            if config.encoder.pca_components is not None:
                raw_vector = config.encoder.pca_components.transform([raw_vector])[0]
            else:
                # Fallback: берём первые 8 или дополняем нулями
                raw_vector = (raw_vector[:8] + [0.0] * 8)[:8]
        
        # Нормализуем если требуется
        if config.encoder.normalize:
            norm = sum(x**2 for x in raw_vector) ** 0.5
            if norm > 0:
                raw_vector = [x / norm for x in raw_vector]
        
        return raw_vector
    
    def _compute_energy(self,
                        data: Any,
                        config: SensorConfig,
                        kwargs: Dict) -> EnergyProfile:
        """Вычисляет энергетический профиль сигнала."""
        defaults = config.energy_defaults
        
        # Базовые значения
        magnitude = kwargs.get("magnitude", defaults.base_magnitude)
        valence = kwargs.get("valence", defaults.base_valence)
        arousal = kwargs.get("arousal", defaults.base_arousal)
        urgency = kwargs.get("urgency", defaults.base_urgency)
        
        # Модификаторы
        if defaults.magnitude_from_confidence:
            confidence = kwargs.get("confidence", 1.0)
            magnitude = int(magnitude * confidence)
        
        if defaults.valence_from_sentiment and isinstance(data, str):
            # Простой sentiment analysis (placeholder)
            # В реальности здесь будет вызов модели
            valence = self._simple_sentiment(data)
        
        return EnergyProfile(
            magnitude=magnitude,
            valence=valence,
            arousal=arousal,
            urgency=urgency
        )
    
    def _emit_to_core(self, event: SignalEvent) -> ProcessingResult:
        """
        Эмитит событие в Rust Core через SignalSystem.
        
        Цепочка в Rust:
            SignalSystem.emit() → Grid.add() → Grid.find_neighbors() → Spreading Activation
        """
        # Конвертируем в формат Rust
        rust_event = {
            "event_id": str(event.event_id),
            "event_type": event.event_type,
            "vector": event.semantic.vector,
            "energy": event.energy.magnitude,
            "valence": event.energy.valence,
            "timestamp": event.temporal.timestamp,
            "source": event.source.sensor_id,
            "priority": event.routing.priority,
            "tags": list(event.routing.tags)
        }
        
        # Вызов Rust через FFI
        # SignalSystem сам решает: добавить в Grid, запустить activation, уведомить подписчиков
        rust_result = self._runtime.signal_system.emit(rust_event)
        
        # Конвертируем результат обратно
        return ProcessingResult(
            token_id=rust_result.get("token_id", 0),
            neighbors=[
                NeighborInfo(
                    token_id=n["id"],
                    distance=n["distance"],
                    resonance=n["resonance"],
                    token_type=n.get("type", "concept"),
                    layer_affinity=n.get("layer", 0)
                )
                for n in rust_result.get("neighbors", [])
            ],
            energy_delta=rust_result.get("energy_delta", 0.0),
            activation_spread=rust_result.get("activation_spread", 0),
            is_novel=rust_result.get("is_novel", False),
            triggered_actions=rust_result.get("triggered_actions", []),
            anomaly_score=rust_result.get("anomaly_score", 0.0),
            processing_time_us=rust_result.get("processing_time_us", 0)
        )
    
    def _resolve_sensor(self, source: str, modality: str) -> str:
        """Находит подходящий сенсор по source и modality."""
        # Сначала ищем точное совпадение
        if source in [s["sensor_id"] for s in self._registry.list_sensors()]:
            return source
        
        # Ищем по модальности
        sensors = self._registry.list_sensors(modality=modality)
        if sensors:
            return sensors[0]["sensor_id"]
        
        # Fallback на встроенные
        if modality == "text":
            return "builtin_text_chat"
        elif modality == "audio":
            return "builtin_audio"
        else:
            raise ValueError(f"No sensor found for source={source}, modality={modality}")
    
    def _infer_data_type(self, data: Any) -> str:
        """Определяет тип данных."""
        if isinstance(data, str):
            return "text"
        elif isinstance(data, (list, tuple)) and all(isinstance(x, (int, float)) for x in data):
            return "float_array"
        elif isinstance(data, bytes):
            return "blob"
        elif isinstance(data, dict):
            return "structured"
        else:
            return "unknown"
    
    def _simple_sentiment(self, text: str) -> float:
        """Простой sentiment analysis (placeholder)."""
        # В реальности здесь будет вызов модели
        positive_words = {"хорошо", "отлично", "супер", "рад", "люблю", "спасибо"}
        negative_words = {"плохо", "ужасно", "грустно", "злой", "ненавижу"}
        
        words = set(text.lower().split())
        pos = len(words & positive_words)
        neg = len(words & negative_words)
        
        if pos + neg == 0:
            return 0.0
        return (pos - neg) / (pos + neg)
    
    def _emit_meta_event(self, event_type: str, data: Dict):
        """Эмитит мета-событие (регистрация сенсора и т.п.)."""
        try:
            self._runtime.signal_system.emit({
                "event_type": event_type,
                "data": data,
                "timestamp": time.time()
            })
        except Exception as e:
            self._logger.warning(f"Failed to emit meta event: {e}")
    
    def _update_stats(self, event_type: str, processing_time_us: int):
        """Обновляет статистику."""
        self._stats["total_signals"] += 1
        
        if event_type not in self._stats["signals_by_type"]:
            self._stats["signals_by_type"][event_type] = 0
        self._stats["signals_by_type"][event_type] += 1
        
        # Скользящее среднее
        n = self._stats["total_signals"]
        old_avg = self._stats["avg_processing_time_us"]
        self._stats["avg_processing_time_us"] = old_avg + (processing_time_us - old_avg) / n
```

---

## 7. Примеры использования

### 7.1 Базовое использование

```python
from neurograph.gateway import SignalGateway

# Создаём Gateway
gateway = SignalGateway()

# Простой текст
event = gateway.push_text("Привет, Оли!", source="telegram")
print(f"Token ID: {event.result.token_id}")
print(f"Neighbors: {len(event.result.neighbors)}")
print(f"Novel: {event.result.is_novel}")

# Системное событие
gateway.push_system("resource.memory_low", {
    "used_mb": 450,
    "total_mb": 512,
    "percent": 88
})
```

### 7.2 Регистрация кастомного сенсора

```python
from neurograph.gateway import (
    SignalGateway, SensorConfig, EncoderConfig, 
    EncoderType, EnergyDefaults
)

gateway = SignalGateway()

# Регистрируем датчик температуры
sensor_id = gateway.register_sensor(
    SensorConfig(
        sensor_type="thermometer",
        modality="environment",
        domain="external",
        encoder=EncoderConfig(
            encoder_type=EncoderType.NUMERIC_DIRECT,
            output_dim=8
        ),
        energy_defaults=EnergyDefaults(
            base_magnitude=30,
            base_urgency=0.2
        ),
        name="Room Temperature Sensor",
        description="DHT22 sensor in living room",
        tags={"smart_home", "environment"},
        max_rate_hz=0.1  # Раз в 10 секунд
    ),
    sensor_id="temp_sensor_living_room"
)

# Отправляем данные
event = gateway.push(
    sensor_id="temp_sensor_living_room",
    data=[22.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],  # Температура в первой координате
    event_type="signal.input.external.environment.temperature"
)
```

### 7.3 Подключение к Telegram боту

```python
from telegram.ext import Application, MessageHandler, filters
from neurograph.gateway import SignalGateway

gateway = SignalGateway()

async def handle_message(update, context):
    text = update.message.text
    user_id = update.effective_user.id
    
    # Отправляем в NeuroGraph
    event = gateway.push_text(
        text=text,
        source=f"telegram_{user_id}",
        tags=["telegram", "user_message"],
        sequence_id=str(update.effective_chat.id)  # ID диалога
    )
    
    # Проверяем сработавшие действия
    if event.result.triggered_actions:
        # Здесь ActionSelector решит что делать
        pass
    
    # Для отладки
    if event.result.is_novel:
        print(f"Novel input: {text}")

app = Application.builder().token("TOKEN").build()
app.add_handler(MessageHandler(filters.TEXT, handle_message))
app.run_polling()
```

### 7.4 Подписка на события (через SignalSystem)

```python
# Подписка происходит на уровне Rust Core
# Python получает callback когда срабатывает фильтр

from neurograph_core import SignalSystem

def on_action_trigger(event):
    """Вызывается когда сработал токен-действие."""
    print(f"Action triggered: {event['triggered_actions']}")
    # Здесь логика формирования ответа

def on_anomaly(event):
    """Вызывается при обнаружении аномалии."""
    print(f"Anomaly detected: {event['anomaly_score']}")

# Регистрация подписчиков
signal_system = SignalSystem()

signal_system.subscribe(
    filter={"event_type": "signal.activation.action_trigger"},
    callback=on_action_trigger
)

signal_system.subscribe(
    filter={"event_type": {"$wildcard": "signal.anomaly.*"}},
    callback=on_anomaly
)
```

---

## 8. Метрики производительности (KPI)

### 8.1 Целевые показатели

| Метрика | Target | Critical |
|---------|--------|----------|
| Латентность push_text() | < 1ms | < 5ms |
| Латентность encoding (8D) | < 0.5ms | < 2ms |
| Латентность emit (Rust) | < 0.1ms | < 0.5ms |
| Throughput | > 1000 signals/sec | > 100 signals/sec |
| Memory per sensor | < 1MB | < 10MB |

### 8.2 Бенчмарки

```python
def benchmark_gateway():
    """Бенчмарк производительности Gateway."""
    import time
    
    gateway = SignalGateway()
    iterations = 10000
    
    # Warm-up
    for _ in range(100):
        gateway.push_text("test")
    
    # Measure
    start = time.perf_counter()
    for i in range(iterations):
        gateway.push_text(f"Test message {i}")
    elapsed = time.perf_counter() - start
    
    print(f"Total: {elapsed:.2f}s")
    print(f"Per signal: {elapsed/iterations*1000:.3f}ms")
    print(f"Throughput: {iterations/elapsed:.0f} signals/sec")
    print(f"Stats: {gateway.get_stats()}")
```

---

## 9. Интеграция с существующими модулями

### 9.1 SignalSystem (Rust Core)

Gateway эмитит события через `signal_system.emit()`. SignalSystem:
- Добавляет токен в Grid
- Запускает Spreading Activation
- Уведомляет подписчиков
- Возвращает результат активации

### 9.2 Guardian (CDNA)

Каждый сигнал проходит валидацию:
- Проверка допустимости event_type
- Проверка ограничений энергии
- Проверка rate limits сенсора

### 9.3 ActionController

ActionController подписывается на `signal.activation.action_trigger` и решает:
- Hot Path (< 1μs): если паттерн знакомый
- Cold Path (1-10ms): если нужен анализ через ADNA

### 9.4 Grid

Grid хранит контекст через Temporal Decay:
- Недавние сигналы имеют высокую энергию
- Контекст диалога = активное облако токенов
- Ассоциации восстанавливаются через find_neighbors()

---

## 10. Миграция с v1.0

### 10.1 Изменения API

| v1.0 | v2.0 |
|------|------|
| `gateway.process_input(text, source)` | `gateway.push_text(text, source)` |
| `AtomicSignal` | `SignalEvent` |
| Синхронный return neighbors | Через `event.result.neighbors` |
| Без регистрации сенсоров | `gateway.register_sensor()` |

### 10.2 Обратная совместимость

```python
# Обёртка для совместимости с v1.0
class LegacyGateway:
    def __init__(self):
        self._gateway = SignalGateway()
    
    def process_input(self, text: str, source: str) -> dict:
        """API v1.0"""
        event = self._gateway.push_text(text, source=source)
        return {
            "token_id": event.result.token_id,
            "neighbors": [n.token_id for n in event.result.neighbors],
            "energy_state": event.result.energy_delta
        }
```

---

## 11. Roadmap

### v2.1 (Planned)
- [ ] Batch processing: `gateway.push_batch([signals])`
- [ ] Async API: `await gateway.push_text_async()`
- [ ] Compression для blob данных

### v2.2 (Future)
- [ ] Streaming для аудио/видео
- [ ] Auto-calibration сенсоров
- [ ] Anomaly detection на уровне Gateway

---

**Signal Gateway v2.0 — Сенсорный интерфейс NeuroGraph OS**  
*"From raw input to semantic understanding"*
