# Спецификация Token v2.0 — Официальная версия

**Статус:** Production Ready  
**Версия:** 2.0.0  
**Дата:** 2025-10-13  
**Источник:** Официальная спецификация NeuroGraph OS Token v2

---

## Обзор

**Token** — базовая единица информации в NeuroGraph OS, представляющая дискретный элемент данных в многомерном пространстве состояний.

### Философия дизайна

Token спроектирован как:

- **Атомарная единица** — неделимый элемент системы
- **Самодостаточная структура** — содержит всю необходимую информацию о себе
- **Компактное представление** — оптимизированное бинарное кодирование (64 байта)
- **Многомерная сущность** — существует одновременно в 8 пространствах
- **Временная единица** — имеет жизненный цикл и историю

---

## Бинарная структура (64 байта)

| Поле | Смещение | Размер | Тип | Описание |
|------|----------|--------|-----|----------|
| **coordinates** | 0-47 | 48 bytes | 8×3×int16 | 24 координаты (X,Y,Z для 8 уровней) |
| **id** | 48-51 | 4 bytes | uint32 | Уникальный идентификатор (0 до 4,294,967,295) |
| **flags** | 52-53 | 2 bytes | uint16 | 16 битов состояния и типа |
| **weight** | 54-57 | 4 bytes | float32 | Весовой коэффициент (IEEE 754) |
| **field_radius** | 58 | 1 byte | uint8 | Радиус влияния (0.0-2.55) |
| **field_strength** | 59 | 1 byte | uint8 | Сила поля (0.0-1.0) |
| **timestamp** | 60-63 | 4 bytes | uint32 | Unix timestamp |

**ИТОГО: 64 bytes**

### Python struct format

```python
import struct

PACK_FORMAT = '<24h I H f 2B I'
#              │  │ │ │ │  └─ timestamp (uint32)
#              │  │ │ │ └──── field_strength, field_radius (2× uint8)
#              │  │ │ └────── weight (float32)
#              │  │ └──────── flags (uint16)
#              │  └────────── id (uint32)
#              └──────────── coordinates (24× int16)

SIZE = 64  # bytes
```

---

## 1. COORDINATES (48 bytes)

### Многомерное позиционирование

Token существует одновременно в **8 пространствах состояний**:

```python
COORDINATE_SPACES = {
    0: "L1_PHYSICAL",    # Физическое пространство
    1: "L2_SENSORY",     # Сенсорное пространство
    2: "L3_MOTOR",       # Моторное пространство
    3: "L4_EMOTIONAL",   # Эмоциональное пространство
    4: "L5_COGNITIVE",   # Когнитивное пространство
    5: "L6_SOCIAL",      # Социальное пространство
    6: "L7_TEMPORAL",    # Темпоральное пространство
    7: "L8_ABSTRACT"     # Абстрактное пространство
}
```

### Кодирование координат

```python
def encode_coordinate(value: float, scale: int = 100) -> int:
    """
    Кодирует float координату в int16
    
    Args:
        value: Значение для кодирования (или None)
        scale: Масштаб (100, 1000, 10000)
    
    Returns:
        int16 значение
    """
    if value is None:
        return 127  # Специальное значение "отсутствует"
    
    scaled = int(value * scale)
    return max(-32767, min(32767, scaled))

def decode_coordinate(encoded: int, scale: int = 100) -> Optional[float]:
    """
    Декодирует int16 координату в float
    
    Args:
        encoded: int16 значение
        scale: Масштаб
    
    Returns:
        float значение или None
    """
    if encoded == 127:
        return None  # Координата отсутствует
    
    return encoded / scale
```

---

## Семантика 8 пространств

### L1: PHYSICAL (Физическое)

**Назначение:** Позиция в физическом 3D пространстве

```python
# Оси:
X: float  # Позиция Восток/Запад (метры)
Y: float  # Позиция Север/Юг (метры)
Z: float  # Высота (метры)

# Кодирование:
тип: int16
масштаб: 100
диапазон: -327.68 до +327.67 метров

# Пример:
encode_coordinate(100.0, scale=100)  # → 10000 (int16)
decode_coordinate(10000, scale=100)  # → 100.0 (float)
```

**Применение:** Локальное позиционирование (комната, здание, квартал)

---

### L2: SENSORY (Сенсорное)

**Назначение:** Параметры сенсорного восприятия

```python
# Оси:
X: float  # Салиентность (заметность): 0.0-1.0
Y: float  # Валентность (привлекательность): -1.0 до +1.0
Z: float  # Новизна (степень изменения): 0.0-1.0

# Кодирование:
тип: int16
масштаб: 10000
диапазон: -1.0 до +1.0 (нормализовано)

# Примеры:
# X (Салиентность): 0 до 10000 → 0.0 (незаметно) до 1.0 (очень заметно)
encode_coordinate(0.5, scale=10000)  # → 5000

# Y (Валентность): -10000 до +10000 → -1.0 (негативно) до +1.0 (позитивно)
encode_coordinate(-0.3, scale=10000)  # → -3000

# Z (Новизна): 0 до 10000 → 0.0 (привычно) до 1.0 (абсолютно ново)
encode_coordinate(0.8, scale=10000)  # → 8000
```

---

### L3: MOTOR (Моторное)

**Назначение:** Параметры движения и действия

```python
# Оси:
X: float  # Линейная скорость (м/с)
Y: float  # Линейное ускорение (м/с²)
Z: float  # Угловая скорость (рад/с)

# Кодирование:
тип: int16
масштаб: 1000
диапазон: -32.768 до +32.767 в соответствующих единицах

# Примеры:
encode_coordinate(5.0, scale=1000)    # → 5000 (5 м/с)
encode_coordinate(-2.5, scale=1000)   # → -2500 (-2.5 м/с²)
encode_coordinate(3.14, scale=1000)   # → 3140 (3.14 рад/с ≈ 0.5 об/сек)
```

**Применение:** Управление движением, планирование траекторий

---

### L4: EMOTIONAL (Эмоциональное)

**Назначение:** VAD модель эмоций (Valence-Arousal-Dominance)

```python
# Оси:
X: float  # Valence (валентность): -1.0 (неудовольствие) до +1.0 (удовольствие)
Y: float  # Arousal (возбуждение): -1.0 (пассивность) до +1.0 (активность)
Z: float  # Dominance (доминирование): -1.0 (подчинение) до +1.0 (контроль)

# Кодирование:
тип: int16
масштаб: 10000
диапазон: -1.0 до +1.0 для всех осей

# Примеры эмоций:
# Радость:      V=+0.8, A=+0.5, D=+0.3
# Грусть:       V=-0.6, A=-0.4, D=-0.5
# Гнев:         V=-0.5, A=+0.8, D=+0.7
# Спокойствие:  V=+0.3, A=-0.6, D=0.0
```

**Применение:** Эмоциональный интеллект, аффективные вычисления

---

### L5: COGNITIVE (Когнитивное)

**Назначение:** Параметры когнитивной обработки

```python
# Оси:
X: float  # Когнитивная нагрузка: 0.0 (нет) до 1.0 (максимум)
Y: float  # Уровень абстракции: 0.0 (конкретный) до 1.0 (философский)
Z: float  # Уверенность/Определённость: 0.0 (неуверенность) до 1.0 (абсолют)

# Кодирование:
тип: int16
масштаб: 10000
диапазон: 0.0 до 1.0 (нормализовано)

# Примеры:
# Простая задача:  нагрузка=0.2, абстракция=0.1, уверенность=0.9
# Сложная задача:  нагрузка=0.9, абстракция=0.7, уверенность=0.4
```

**Применение:** Управление вниманием, планирование, рассуждения

---

### L6: SOCIAL (Социальное)

**Назначение:** Параметры социального взаимодействия

```python
# Оси:
X: float  # Социальная дистанция: -1.0 (враждебный) до +1.0 (интимный)
Y: float  # Иерархический статус: -1.0 (подчинённый) до +1.0 (доминантный)
Z: float  # Принадлежность: -1.0 (чужой/out-group) до +1.0 (свой/in-group)

# Кодирование:
тип: int16
масштаб: 10000
диапазон: -1.0 до +1.0 для всех осей

# Примеры отношений:
# Друг:      дистанция=+0.8, статус=0.0, принадлежность=+0.9
# Начальник: дистанция=-0.3, статус=+0.7, принадлежность=+0.5
# Враг:      дистанция=-0.9, статус=-0.5, принадлежность=-0.8
```

**Применение:** Социальная навигация, моделирование отношений

---

### L7: TEMPORAL (Темпоральное)

**Назначение:** Параметры временной локализации

```python
# Оси:
X: float  # Временное смещение (секунды, относительно timestamp)
Y: float  # Длительность (секунды)
Z: float  # Периодичность/Частота (Герцы)

# Кодирование:
тип: int16
масштаб: 100 (для X,Y), 1000 (для Z)
диапазон X: ±327 секунд
диапазон Y: 0-327 секунд
диапазон Z: 0-32.7 Гц

# Примеры:
# Событие через 5 минут:  offset=300, duration=60, frequency=0
# Периодический сигнал:   offset=0, duration=1, frequency=10.0
```

**Применение:** Временная координация, планирование, ритмы

---

### L8: ABSTRACT (Абстрактное)

**Назначение:** Семантические отношения и логика

```python
# Оси:
X: float  # Семантическая близость: -1.0 (антоним) до +1.0 (синоним)
Y: float  # Каузальная связь: -1.0 (ингибирует) до +1.0 (вызывает)
Z: float  # Логическая модальность: -1.0 (невозможно) до +1.0 (необходимо)

# Кодирование:
тип: int16
масштаб: 10000
диапазон: -1.0 до +1.0 для всех осей

# Примеры:
# Синоним:       X=+0.9, Y=0.0, Z=0.5
# Причина-следствие: X=0.0, Y=+0.8, Z=0.3
# Логическое противоречие: X=-0.9, Y=-0.5, Z=-1.0
```

**Применение:** Семантический анализ, рассуждения, логический вывод

---

## Таблица масштабов

| Уровень | Название | Масштаб | Диапазон | Единицы |
|---------|----------|---------|----------|---------|
| L1 | Physical | 100 | ±327.67 | метры |
| L2 | Sensory | 10000 | 0.0-1.0 или ±1.0 | нормализовано |
| L3 | Motor | 1000 | ±32.767 | м/с, м/с², рад/с |
| L4 | Emotional | 10000 | ±1.0 | VAD модель |
| L5 | Cognitive | 10000 | 0.0-1.0 | нормализовано |
| L6 | Social | 10000 | ±1.0 | нормализовано |
| L7 | Temporal | 100 (X,Y), 1000 (Z) | ±327 сек, 0-32.7 Гц | секунды, Герцы |
| L8 | Abstract | 10000 | ±1.0 | нормализовано |

---

## 2. ID (4 bytes)

### Структура идентификатора

ID токена — это uint32, содержащий **метаданные**:

```python
# Структура ID (32 бита):
# Биты 0-23:  Локальный ID (16,777,216 уникальных токенов)
# Биты 24-27: Тип сущности (16 типов)
# Биты 28-31: Домен/область (16 доменов)

def extract_local_id(token_id: int) -> int:
    """Извлечь локальный ID"""
    return token_id & 0xFFFFFF  # Биты 0-23

def extract_entity_type(token_id: int) -> int:
    """Извлечь тип сущности"""
    return (token_id >> 24) & 0xF  # Биты 24-27

def extract_domain(token_id: int) -> int:
    """Извлечь домен"""
    return (token_id >> 28) & 0xF  # Биты 28-31

def create_id(local_id: int, entity_type: int, domain: int) -> int:
    """Создать ID из компонентов"""
    return (domain << 28) | (entity_type << 24) | (local_id & 0xFFFFFF)
```

### Типы сущностей

```python
ENTITY_TYPES = {
    0x0: "UNDEFINED",     # Неопределённый тип
    0x1: "OBJECT",        # Физический объект
    0x2: "EVENT",         # Событие
    0x3: "STATE",         # Состояние
    0x4: "PROCESS",       # Процесс
    0x5: "CONCEPT",       # Концепт/идея
    0x6: "RELATION",      # Отношение
    0x7: "PATTERN",       # Паттерн
    0x8: "RULE",          # Правило
    0x9: "GOAL",          # Цель
    0xA: "MEMORY",        # Память
    0xB: "SENSOR",        # Сенсор
    0xC: "ACTUATOR",      # Актуатор
    0xD: "CONTROLLER",    # Контроллер
    0xE: "BUFFER",        # Буфер
    0xF: "RESERVED"       # Зарезервировано
}
```

### Примеры ID

```python
# Физический объект #42 в домене 1
id = create_id(local_id=42, entity_type=0x1, domain=0x1)
# = 0x11000042

# Событие #1000 в домене 0
id = create_id(local_id=1000, entity_type=0x2, domain=0x0)
# = 0x020003E8

# Память #500 в домене 3
id = create_id(local_id=500, entity_type=0xA, domain=0x3)
# = 0x3A0001F4
```

---

## 3. FLAGS (2 bytes)

### Структура флагов (16 бит)

```python
# ═══════════════════════════════════════════════════════
# СИСТЕМНЫЕ ФЛАГИ (Биты 0-7)
# ═══════════════════════════════════════════════════════

FLAG_ACTIVE       = 0x0001  # (Бит 0) Токен активен
FLAG_PERSISTENT   = 0x0002  # (Бит 1) Должен быть сохранён
FLAG_MUTABLE      = 0x0004  # (Бит 2) Может изменяться
FLAG_SYNCHRONIZED = 0x0008  # (Бит 3) Синхронизирован
FLAG_COMPRESSED   = 0x0010  # (Бит 4) Данные сжаты
FLAG_ENCRYPTED    = 0x0020  # (Бит 5) Данные зашифрованы
FLAG_DIRTY        = 0x0040  # (Бит 6) Изменён, не сохранён
FLAG_LOCKED       = 0x0080  # (Бит 7) Заблокирован

# ═══════════════════════════════════════════════════════
# СЕМАНТИЧЕСКИЕ ФЛАГИ: ТИП СУЩНОСТИ (Биты 8-11)
# ═══════════════════════════════════════════════════════

ENTITY_TYPE_MASK = 0x0F00  # Маска для выделения битов 8-11

TYPE_UNDEFINED   = 0x0000  # (0000) Неопределённый
TYPE_OBJECT      = 0x0100  # (0001) Физический объект
TYPE_EVENT       = 0x0200  # (0010) Событие
TYPE_STATE       = 0x0300  # (0011) Состояние
TYPE_PROCESS     = 0x0400  # (0100) Процесс
TYPE_CONCEPT     = 0x0500  # (0101) Концепт/идея
TYPE_RELATION    = 0x0600  # (0110) Отношение
TYPE_PATTERN     = 0x0700  # (0111) Паттерн
TYPE_RULE        = 0x0800  # (1000) Правило
TYPE_GOAL        = 0x0900  # (1001) Цель
TYPE_MEMORY      = 0x0A00  # (1010) Память
TYPE_SENSOR      = 0x0B00  # (1011) Сенсор
TYPE_ACTUATOR    = 0x0C00  # (1100) Актуатор
TYPE_CONTROLLER  = 0x0D00  # (1101) Контроллер
TYPE_BUFFER      = 0x0E00  # (1110) Буфер
TYPE_RESERVED    = 0x0F00  # (1111) Зарезервировано

# ═══════════════════════════════════════════════════════
# ПОЛЬЗОВАТЕЛЬСКИЕ ФЛАГИ (Биты 12-15)
# ═══════════════════════════════════════════════════════

FLAG_USER_1      = 0x1000  # (Бит 12) Пользовательский 1
FLAG_USER_2      = 0x2000  # (Бит 13) Пользовательский 2
FLAG_USER_3      = 0x4000  # (Бит 14) Пользовательский 3
FLAG_USER_4      = 0x8000  # (Бит 15) Пользовательский 4
```

### Работа с флагами

```python
# Инициализация (активный процесс)
flags = FLAG_ACTIVE | TYPE_PROCESS

# Проверка типа
entity_type = flags & ENTITY_TYPE_MASK
if entity_type == TYPE_PROCESS:
    print("Это токен процесса")

# Проверка флага
if flags & FLAG_ACTIVE:
    print("Токен активен")

# Изменение типа
flags = (flags & ~ENTITY_TYPE_MASK) | TYPE_EVENT

# Установка флага
flags |= FLAG_DIRTY

# Сброс флага
flags &= ~FLAG_DIRTY
```

---

## 4. WEIGHT (4 bytes)

### Весовой коэффициент (IEEE 754 float32)

```python
class TokenWeight:
    def __init__(self, weight: float = 0.0):
        self.weight = max(0.0, min(1.0, weight))
    
    @property
    def intensity(self) -> float:
        """Интенсивность токена (0.0-1.0)"""
        return self.weight
    
    @property
    def priority(self) -> int:
        """Приоритет (0-255)"""
        return int(self.weight * 255)
    
    @property
    def energy_level(self) -> float:
        """Энергетический уровень (квадратичная зависимость)"""
        return self.weight ** 2
    
    def decay(self, factor: float = 0.99) -> 'TokenWeight':
        """Затухание веса"""
        return TokenWeight(self.weight * factor)
    
    def amplify(self, factor: float = 1.1) -> 'TokenWeight':
        """Усиление веса"""
        return TokenWeight(min(1.0, self.weight * factor))
```

### Семантика веса

```
0.0       - неактивный/отсутствующий токен
0.1-0.3   - слабая активация
0.3-0.7   - умеренная активация
0.7-0.9   - сильная активация
0.9-1.0   - максимальная активация
```

---

## 5. FIELD_RADIUS (1 byte)

### Радиус влияния поля

```python
# Тип: uint8 (0-255)
# Масштаб: 100
# Диапазон: 0.00 до 2.55

def encode_field_radius(radius: float) -> int:
    """Кодировать радиус в uint8"""
    return int(max(0, min(255, radius * 100)))

def decode_field_radius(encoded: int) -> float:
    """Декодировать радиус из uint8"""
    return encoded / 100.0

# Примеры:
encode_field_radius(1.0)   # → 100 (uint8)
decode_field_radius(100)   # → 1.0 (float)

encode_field_radius(0.5)   # → 50
encode_field_radius(2.55)  # → 255 (максимум)
```

### Семантика радиуса

- **< 0.5**: Узкое влияние (специфичный токен)
- **0.5-1.5**: Стандартное влияние (обычный токен)
- **> 1.5**: Широкое влияние (абстрактный токен)

---

## 6. FIELD_STRENGTH (1 byte)

### Интенсивность силы поля

```python
# Тип: uint8 (0-255)
# Масштаб: 255 (нормализация)
# Диапазон: 0.0 до 1.0

def encode_field_strength(strength: float) -> int:
    """Кодировать силу в uint8"""
    return int(max(0, min(255, strength * 255)))

def decode_field_strength(encoded: int) -> float:
    """Декодировать силу из uint8"""
    return encoded / 255.0

# Примеры:
encode_field_strength(1.0)    # → 255 (максимум)
decode_field_strength(255)    # → 1.0

encode_field_strength(0.5)    # → 127 (≈0.498)
decode_field_strength(128)    # → 0.502
```

### Семантика силы

- **< 0.4**: Слабое поле (редкий токен)
- **0.4-0.8**: Среднее поле (обычный токен)
- **> 0.8**: Сильное поле (важный токен)

---

## 7. TIMESTAMP (4 bytes)

### Unix timestamp (uint32)

```python
import time
from datetime import datetime

class TokenTimestamp:
    def __init__(self, timestamp: Optional[int] = None):
        self.timestamp = timestamp or int(time.time())
    
    @property
    def datetime(self) -> datetime:
        """Преобразовать в datetime"""
        return datetime.fromtimestamp(self.timestamp)
    
    @property
    def age_seconds(self) -> int:
        """Возраст в секундах"""
        return int(time.time()) - self.timestamp
    
    @property
    def age_minutes(self) -> float:
        """Возраст в минутах"""
        return self.age_seconds / 60.0
    
    def is_recent(self, seconds: int = 60) -> bool:
        """Свежий ли токен?"""
        return self.age_seconds < seconds
    
    def is_stale(self, seconds: int = 3600) -> bool:
        """Устаревший ли токен?"""
        return self.age_seconds > seconds
```

### Диапазон timestamp

```
Минимум: 0 (1 января 1970)
Максимум: 4,294,967,295 (7 февраля 2106)
```

---

## Python API

### Класс Token v2.0

```python
import struct
import numpy as np
from typing import Optional, Tuple
from dataclasses import dataclass

@dataclass
class Token:
    """
    Token v2.0 - 64-байтная структура
    """
    
    # Константы
    SIZE = 64
    PACK_FORMAT = '<24h I H f 2B I'
    
    # Масштабы для координат
    COORDINATE_SCALES = {
        0: 100,    # L1: Physical
        1: 10000,  # L2: Sensory
        2: 1000,   # L3: Motor
        3: 10000,  # L4: Emotional
        4: 10000,  # L5: Cognitive
        5: 10000,  # L6: Social
        6: 100,    # L7: Temporal (X,Y), 1000 (Z) - обрабатывается отдельно
        7: 10000   # L8: Abstract
    }
    
    def __init__(self, id: int = 0):
        self.id = id
        self.coordinates = np.full((8, 3), 127, dtype=np.int16)  # Undefined
        self.flags = 0x0001  # FLAG_ACTIVE
        self.weight = 0.5
        self.field_radius = 1.0
        self.field_strength = 1.0
        self.timestamp = int(time.time())
    
    def set_coordinates(self, level: int, x: float, y: float, z: float):
        """
        Установить координаты на уровне
        
        Args:
            level: Уровень (0-7)
            x, y, z: Координаты
        """
        scale = self.COORDINATE_SCALES[level]
        
        # L7 имеет разные масштабы для Z
        if level == 6:
            self.coordinates[level] = [
                self._encode_coord(x, 100),   # X: секунды
                self._encode_coord(y, 100),   # Y: секунды
                self._encode_coord(z, 1000)   # Z: Герцы
            ]
        else:
            self.coordinates[level] = [
                self._encode_coord(x, scale),
                self._encode_coord(y, scale),
                self._encode_coord(z, scale)
            ]
    
    def get_coordinates(self, level: int) -> Optional[Tuple[float, float, float]]:
        """
        Получить координаты уровня
        
        Args:
            level: Уровень (0-7)
        
        Returns:
            (x, y, z) или None если не определены
        """
        coords = self.coordinates[level]
        
        if coords[0] == 127:  # Undefined
            return None
        
        scale = self.COORDINATE_SCALES[level]
        
        # L7 имеет разные масштабы
        if level == 6:
            return (
                self._decode_coord(coords[0], 100),
                self._decode_coord(coords[1], 100),
                self._decode_coord(coords[2], 1000)
            )
        else:
            return (
                self._decode_coord(coords[0], scale),
                self._decode_coord(coords[1], scale),
                self._decode_coord(coords[2], scale)
            )
    
    def _encode_coord(self, value: Optional[float], scale: int) -> int:
        """Кодировать координату"""
        if value is None:
            return 127
        scaled = int(value * scale)
        return max(-32767, min(32767, scaled))
    
    def _decode_coord(self, encoded: int, scale: int) -> Optional[float]:
        """Декодировать координату"""
        if encoded == 127:
            return None
        return encoded / scale
    
    def pack(self) -> bytes:
        """
        Сериализовать токен в 64 байта
        
        Returns:
            bytes: 64-байтный блок данных
        """
        # Координаты: 24 × int16 = 48 bytes
        coords_flat = self.coordinates.flatten()
        
        # Остальные поля
        field_radius_encoded = int(self.field_radius * 100)
        field_strength_encoded = int(self.field_strength * 255)
        
        # Pack всё вместе
        data = struct.pack(
            self.PACK_FORMAT,
            *coords_flat,                    # 24 × int16
            self.id,                         # uint32
            self.flags,                      # uint16
            self.weight,                     # float32
            field_radius_encoded,            # uint8
            field_strength_encoded,          # uint8
            self.timestamp                   # uint32
        )
        
        return data
    
    @classmethod
    def unpack(cls, buffer: bytes) -> 'Token':
        """
        Десериализовать токен из 64 байт
        
        Args:
            buffer: 64-байтный блок данных
        
        Returns:
            Token: Восстановленный токен
        """
        if len(buffer) != cls.SIZE:
            raise ValueError(f"Expected {cls.SIZE} bytes, got {len(buffer)}")
        
        # Распаковываем
        unpacked = struct.unpack(cls.PACK_FORMAT, buffer)
        
        # Создаём токен
        token = cls(id=unpacked[24])  # ID на позиции 24
        
        # Координаты (первые 24 значения)
        coords = np.array(unpacked[:24], dtype=np.int16).reshape(8, 3)
        token.coordinates = coords
        
        # Остальные поля
        token.flags = unpacked[25]
        token.weight = unpacked[26]
        token.field_radius = unpacked[27] / 100.0
        token.field_strength = unpacked[28] / 255.0
        token.timestamp = unpacked[29]
        
        return token
    
    def to_dict(self) -> dict:
        """Экспортировать в словарь"""
        return {
            'id': self.id,
            'id_parts': {
                'local_id': self.id & 0xFFFFFF,
                'entity_type': (self.id >> 24) & 0xF,
                'domain': (self.id >> 28) & 0xF
            },
            'coordinates': {
                f'L{i+1}': self.get_coordinates(i)
                for i in range(8)
            },
            'flags': {
                'value': self.flags,
                'active': bool(self.flags & 0x0001),
                'persistent': bool(self.flags & 0x0002),
                'entity_type': (self.flags & 0x0F00) >> 8
            },
            'weight': self.weight,
            'field': {
                'radius': self.field_radius,
                'strength': self.field_strength
            },
            'timestamp': self.timestamp,
            'age_seconds': int(time.time()) - self.timestamp
        }
```

---

## Примеры использования

### Пример 1: Создание токена объекта

```python
import time

# Создание токена для физического объекта
local_id = 42
entity_type = 0x1  # OBJECT
domain = 0x0

token_id = create_id(local_id, entity_type, domain)
token = Token(id=token_id)

# Установка физической позиции (L1)
token.set_coordinates(level=0, x=10.5, y=20.3, z=1.5)

# Установка эмоциональных параметров (L4)
# Радость: V=+0.8, A=+0.5, D=+0.3
token.set_coordinates(level=3, x=0.8, y=0.5, z=0.3)

# Настройка поля
token.field_radius = 1.0
token.field_strength = 0.8
token.weight = 0.7

# Установка флагов
token.flags = FLAG_ACTIVE | FLAG_PERSISTENT | TYPE_OBJECT

print(token.to_dict())
```

### Пример 2: Сериализация и десериализация

```python
# Создание токена
token1 = Token(id=12345)
token1.set_coordinates(0, x=5.0, y=10.0, z=2.0)
token1.weight = 0.9

# Сериализация в 64 байта
binary_data = token1.pack()
print(f"Размер: {len(binary_data)} байт")

# Десериализация
token2 = Token.unpack(binary_data)
print(f"ID восстановлен: {token2.id}")
print(f"Координаты L1: {token2.get_coordinates(0)}")
print(f"Weight: {token2.weight}")
```

### Пример 3: Работа с ID

```python
# Создание ID с метаданными
local_id = 1000
entity_type = 0x5  # CONCEPT
domain = 0x2

token_id = create_id(local_id, entity_type, domain)
print(f"Token ID: 0x{token_id:08X}")

# Извлечение компонентов
print(f"Local ID: {extract_local_id(token_id)}")
print(f"Entity Type: 0x{extract_entity_type(token_id):X} ({ENTITY_TYPES[extract_entity_type(token_id)]})")
print(f"Domain: {extract_domain(token_id)}")
```

### Пример 4: Работа с флагами

```python
token = Token(id=100)

# Установка типа и состояния
token.flags = FLAG_ACTIVE | FLAG_MUTABLE | TYPE_PROCESS

# Проверка флагов
if token.flags & FLAG_ACTIVE:
    print("Токен активен")

# Извлечение типа
entity_type = (token.flags & ENTITY_TYPE_MASK) >> 8
print(f"Тип: {ENTITY_TYPES.get(entity_type, 'UNKNOWN')}")

# Изменение состояния
token.flags |= FLAG_DIRTY  # Установить DIRTY
token.flags &= ~FLAG_MUTABLE  # Сбросить MUTABLE
```

### Пример 5: Затухание веса

```python
token = Token(id=200)
token.weight = 1.0

# Симуляция затухания со временем
for step in range(10):
    token.weight *= 0.95  # Затухание 5% за шаг
    print(f"Шаг {step}: weight={token.weight:.3f}")
```

---

## Валидация

### Проверка корректности токена

```python
def validate_token(token: Token) -> bool:
    """
    Валидация токена
    
    Returns:
        True если токен валиден
    """
    # Проверка ID
    if not (0 <= token.id <= 0xFFFFFFFF):
        return False
    
    # Проверка координат
    for level in range(8):
        coords = token.coordinates[level]
        if not all(-32767 <= c <= 32767 for c in coords):
            return False
    
    # Проверка weight
    if not (0.0 <= token.weight <= 1.0):
        return False
    
    # Проверка field_radius
    if not (0.0 <= token.field_radius <= 2.55):
        return False
    
    # Проверка field_strength
    if not (0.0 <= token.field_strength <= 1.0):
        return False
    
    # Проверка timestamp
    if not (0 <= token.timestamp <= 0xFFFFFFFF):
        return False
    
    return True
```

---

## Совместимость и миграция

### Отличия от Token v1.0

| Аспект | v1.0 | v2.0 |
|--------|------|------|
| Размер | 64 байта | 64 байта ✅ |
| Координаты | Фиксированный масштаб | Разные масштабы для уровней |
| ID | Простой uint32 | uint32 с метаданными |
| Flags | 16 бит состояния | Состояние + тип сущности |
| field_radius | Не было | Есть (1 байт) |
| field_strength | Не было | Есть (1 байт) |
| reserved | 2 байта | Используются для полей |

### Миграция с v1.0

```python
def migrate_v1_to_v2(token_v1) -> Token:
    """Мигрировать токен из v1.0 в v2.0"""
    
    token_v2 = Token(id=token_v1.id)
    
    # Копируем координаты (масштабы могут отличаться!)
    for level in range(8):
        coords_v1 = token_v1.get_coordinates(level)
        if coords_v1:
            # Предполагаем, что v1.0 использовал масштаб 100 везде
            # Нужно пересчитать для новых масштабов
            scale_v1 = 100
            scale_v2 = Token.COORDINATE_SCALES[level]
            
            x = (coords_v1[0] * scale_v2) / scale_v1
            y = (coords_v1[1] * scale_v2) / scale_v1
            z = (coords_v1[2] * scale_v2) / scale_v1
            
            token_v2.set_coordinates(level, x, y, z)
    
    # Копируем базовые поля
    token_v2.flags = token_v1.flags
    token_v2.weight = token_v1.weight
    token_v2.timestamp = token_v1.timestamp
    
    # Новые поля - значения по умолчанию
    token_v2.field_radius = 1.0
    token_v2.field_strength = 1.0
    
    return token_v2
```

---

## Ограничения и рекомендации

### Ограничения

- **Размер:** Строго 64 байта (нельзя расширить)
- **Координаты:** int16 ограничивает диапазон и точность
- **Timestamp:** До 2106 года (uint32)
- **Масштабы:** Жёстко заданы для каждого уровня

### Рекомендации

1. **Выбор уровня:** Используйте уровни по назначению (L1 для физики, L4 для эмоций и т.д.)
2. **Масштабы:** Помните о разных масштабах для разных уровней
3. **Валидация:** Всегда валидируйте токены перед использованием
4. **Timestamp:** Обновляйте timestamp при изменении токена
5. **Flags:** Используйте правильные комбинации флагов
6. **Метаданные в ID:** Планируйте структуру доменов и типов заранее

---

## Расширения (RESERVED)

### Возможные типы расширений

Для будущих версий предусмотрены типы расширений:

```python
EXTENSION_TYPES = {
    0x0: "NONE",          # Нет расширения
    0x1: "PARENT_REF",    # Ссылка на родителя
    0x2: "CLUSTER_ID",    # ID кластера
    0x3: "PRIORITY_HINT", # Подсказка приоритета
    0x4: "COLOR_TAG",     # Цветовая метка
    0x5: "SIZE_HINT",     # Подсказка размера
    0x6: "LIFECYCLE",     # Жизненный цикл
    0x7: "ACCESS_COUNT",  # Счётчик обращений
    0x8: "ERROR_CODE",    # Код ошибки
    0x9: "BATCH_ID",      # ID пакета
    0xA: "VERSION",       # Версия токена
    0xB: "CHECKSUM",      # Контрольная сумма
    0xC: "COMPRESSION",   # Информация о сжатии
    0xD: "SECURITY",      # Уровень безопасности
    0xE: "CUSTOM_1",      # Пользовательское 1
    0xF: "CUSTOM_2"       # Пользовательское 2
}
```

**Примечание:** Расширения могут храниться вне токена (в отдельных структурах данных), так как 64 байта полностью заняты.

---

## Заключение

Token v2.0 — это компактная, самодостаточная структура данных, оптимизированная для представления сущностей в многомерном пространстве состояний NeuroGraph OS.

**Ключевые особенности:**
- ✅ Строго 64 байта
- ✅ 8 семантических уровней
- ✅ Разные масштабы координат
- ✅ Метаданные в ID
- ✅ Комбинированные flags
- ✅ Поля для полевой модели
- ✅ Временная метка

---

**Версия документа:** 2.0.0  
**Дата:** 2025-10-13  
**Статус:** Official Specification  
**Автор:** NeuroGraph OS Team