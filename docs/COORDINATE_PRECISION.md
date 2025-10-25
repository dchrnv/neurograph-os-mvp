# Точность координат в Token V2.0

**Версия:** 0.12.0 mvp_TokenR
**Дата:** 2025-10-25

## Важно: Используйте точность x.xx

При работе с координатами в Token V2.0 всегда используйте **два знака после запятой** (precision x.xx), даже если второй знак равен нулю.

### ✅ Правильно

```rust
// Rust
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);
token.set_coordinates(CoordinateSpace::L4Emotional, 0.80, 0.60, 0.50);
```

```python
# Python
token.set_coordinates(level=0, x=10.50, y=20.30, z=5.20)
token.set_coordinates(level=3, x=0.80, y=0.60, z=0.50)
```

### ❌ Неправильно

```rust
// Rust - плохо: неявная точность
token.set_coordinates(CoordinateSpace::L1Physical, 10.5, 20.3, 5.2);
token.set_coordinates(CoordinateSpace::L4Emotional, 0.8, 0.6, 0.5);
```

```python
# Python - плохо: неявная точность
token.set_coordinates(level=0, x=10.5, y=20.3, z=5.2)
token.set_coordinates(level=3, x=0.8, y=0.6, z=0.5)
```

## Почему это важно?

### 1. Фиксированная точка (Fixed-point)

Координаты кодируются как **16-битные целые числа (i16)** с масштабированием:

```rust
// Пример для L1 Physical (scale = 100)
10.50 * 100 = 1050 (i16) ✅
10.5  * 100 = 1050 (i16) ✅ (технически то же самое)

// Но для читаемости и консистентности используйте x.xx
```

### 2. Разные масштабы для разных пространств

| Space | Scale | Precision | Range |
|-------|-------|-----------|-------|
| L1 Physical | 100 | **0.01** | ±327.67 |
| L2 Sensory | 10000 | **0.0001** | ±3.2767 |
| L3 Motor | 1000 | **0.001** | ±32.767 |
| L4 Emotional | 10000 | **0.0001** | ±3.2767 |
| L5 Cognitive | 10000 | **0.0001** | ±3.2767 |
| L6 Social | 10000 | **0.0001** | ±3.2767 |
| L7 Temporal | 100/1000 | **0.01/0.001** | ±327/±32.7 |
| L8 Abstract | 10000 | **0.0001** | ±3.2767 |

### 3. Явная точность делает код читаемым

```rust
// Явно видно, что мы используем два знака после запятой
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);

// Vs неявная точность - непонятно, это намеренно или случайно
token.set_coordinates(CoordinateSpace::L1Physical, 10.5, 20.3, 5.2);
```

### 4. Консистентность кода

Во всей кодовой базе и документации используется формат **x.xx**:

- Примеры в README
- Тесты
- Демо-приложения
- Документация

## Примеры для всех пространств

### L1 Physical (scale 100, precision 0.01)

```rust
// Meters: x.xx (two decimal places)
token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);
//                                                 ^^^^^ ^^^^^ ^^^^^
//                                                 X     Y     Z
```

### L2 Sensory (scale 10000, precision 0.0001)

```rust
// Normalized 0.0-1.0: x.xx (внутри будет 0.xxxx)
token.set_coordinates(CoordinateSpace::L2Sensory, 0.80, 0.60, 0.50);
//                                                ^^^^^ ^^^^^ ^^^^^
//                                                Sal.  Val.  Nov.
```

### L4 Emotional (scale 10000, precision 0.0001)

```rust
// VAD model -1.0 to +1.0: x.xx
token.set_coordinates(CoordinateSpace::L4Emotional, 0.80, 0.60, 0.50);
//                                                  ^^^^^ ^^^^^ ^^^^^
//                                                  Val.  Ar.   Dom.
```

### L7 Temporal (scale 100/1000, precision 0.01/0.001)

```rust
// Seconds and Hz: x.xx
token.set_coordinates(CoordinateSpace::L7Temporal, 10.50, 5.20, 1.50);
//                                                 ^^^^^ ^^^^^ ^^^^^
//                                                 Offs. Dur.  Freq.
```

## Кодирование и декодирование

### Encoding (float → i16)

```rust
fn encode_coordinate(value: f32, space: CoordinateSpace) -> i16 {
    let scale = SCALE_FACTORS[space as usize];
    let scaled = value * scale;
    scaled.clamp(i16::MIN as f32, i16::MAX as f32) as i16
}

// Пример:
// L1 Physical: 10.50 * 100 = 1050 (i16)
// L4 Emotional: 0.80 * 10000 = 8000 (i16)
```

### Decoding (i16 → float)

```rust
fn decode_coordinate(encoded: i16, space: CoordinateSpace) -> f32 {
    let scale = SCALE_FACTORS[space as usize];
    (encoded as f32) / scale
}

// Пример:
// L1 Physical: 1050 / 100 = 10.50
// L4 Emotional: 8000 / 10000 = 0.80
```

## В тестах

Всегда используйте явную точность в тестах:

```rust
#[test]
fn test_coordinate_precision() {
    let mut token = Token::new(1);

    // ✅ Хорошо - явная точность
    token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);

    let coords = token.get_coordinates(CoordinateSpace::L1Physical);
    assert!((coords[0] - 10.50).abs() < 0.01); // ✅
    assert!((coords[1] - 20.30).abs() < 0.01); // ✅
    assert!((coords[2] - 5.20).abs() < 0.01);  // ✅
}
```

## Summary

- ✅ **Всегда используйте x.xx** (два знака после запятой)
- ✅ **Явная точность** лучше неявной
- ✅ **Консистентность** во всей кодовой базе
- ✅ **Читаемость** и понятность кода

## См. также

- [Token V2 Specification](Token%20V2.md) - Полная спецификация
- [Token V2 Rust README](../src/core_rust/README.md) - API документация
- [SCALE_FACTORS](../src/core_rust/src/token.rs#L75) - Таблица масштабов

---

*Последнее обновление: 2025-10-25*
