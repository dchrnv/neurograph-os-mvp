# Спецификация Bootstrap Library v1.2

Версия: 1.2.0 (Semantic Crystal Update)

Дата: 2025-11-22

Статус: RFC / Ready for Implementation

Зависимости: Token v2.0, Connection v3.0, Grid v2.0, CDNA v2.1

Внешние крейты: linfa (PCA), fasthash (Murmur3), rayon (Parallel iterators)

---

## 1. Философия: От "Супа" к "Кристаллу"

### 1.1 Проблема v1.1

В предыдущей версии предлагалось загружать данные "как есть" (слайсинг векторов) и связывать их полным перебором ($O(N^2)$). Это привело бы к:

1. **Семантическому шуму:** Первые 3 измерения 300-мерного вектора не несут всей топологии смысла.
    
2. **Параличу загрузки:** Связывание 50k токенов заняло бы часы.
    
3. **Фрагментации:** Цвет "Красный" и слово "Красный" были бы разными сущностями.
    

### 1.2 Решение v1.2

Мы строим **Семантический Кристалл** — жестко структурированное начальное состояние.

1. **Математическая проекция (PCA):** Сжимаем 300D смыслы в 3D пространство Grid, сохраняя глобальную структуру.
    
2. **Пространственная индексация:** Используем мощь `Grid v2.0` для поиска связей за $O(N \log N)$.
    
3. **Синестезия:** Создаем мультимодальные токены-якоря, объединяющие сенсорику и смысл.
    
4. **Философская калибровка:** Проецируем данные на оси Форма/Хаос, Эрос/Танатос, Воля/Ничто.
    

---

## 2. Архитектура Модуля

### 2.1 Структуры данных
Rust

```Rust

pub struct BootstrapLibrary {
    config: BootstrapConfig,
    grid: Arc<Grid>,                    // Ссылка на Grid для индексации
    pca_model: Option<PcaModel>,        // Сериализуемая модель проекции
    id_registry: HashSet<u32>,          // Контроль коллизий хешей
}

#[derive(Deserialize)]
pub struct BootstrapConfig {
    pub embeddings_path: String,        // Путь к GloVe/Word2Vec
    pub pca_components: usize,          // Цель: 3 (для X, Y, Z)
    pub similarity_radius: f32,         // Радиус поиска соседей в Grid
    pub max_neighbors: usize,           // Лимит связей (во избежание hairball)
    pub system_seed: u64,               // Соль для детерминизма
}
```

### 2.2 Генерация ID (Deterministic Hashing)

Отказ от жестких диапазонов. ID зависит только от контента.

Rust

```Rust

fn generate_id(namespace: &str, value: &str, registry: &HashSet<u32>) -> u32 {
    let base_key = format!("{}:{}", namespace, value);
    let mut hash = fasthash::murmur3::hash32(&base_key);
    
    // Linear Probing для разрешения коллизий
    while registry.contains(&hash) || hash < 1000 { 
        // < 1000 зарезервировано под системные нужды
        hash = hash.wrapping_add(1); 
    }
    hash
}
```

---

## 3. Пайплайн Обработки Данных

### Этап 1: Математическая Проекция (Embedding Processing)

Превращаем "сырые" векторы (300D) в координаты Grid (3D).

**Алгоритм:**

1. **Загрузка:** Читаем GloVe/Word2Vec (50k-100k слов).
    
2. **Обучение PCA:** Строим матрицу проекции, которая максимизирует дисперсию (сохраняет максимум смысла) при сжатии в 3D.
    
3. **Трансформация:** Проецируем все вектора.
    
4. **Нормализация (Z-Score):** Масштабируем результат так, чтобы он заполнил диапазон Grid `[-3.27, +3.27]` (стандартное отклонение).
    

> Важно: Ось X, Y, Z после PCA — это математические абстракции. Но в CDNA v2.1 мы задали им смысл.
> 
> Гипотеза: 1-я компонента PCA (самая сильная вариативность) часто коррелирует с осью "Активность/Пассивность" (Воля). 2-я — с "Позитив/Негатив" (Эрос).

### Этап 2: Создание Якорей (Multimodal Anchors)

Создание токенов, существующих сразу в нескольких слоях.

**Пример: Токен "FIRE" (Огонь)**

- **ID:** `hash("concept:fire")`
    
- **L8 (Abstract):** Координаты из PCA слова "fire".
    
    - _Пример:_ `[-2.5 (Хаос), -1.0 (Разрушение/Танатос), +3.0 (Энергия/Воля)]`.
        
- **L2 (Sensory):** Координаты цвета (Оранжево-Красный).
    
    - RGB `(255, 69, 0)` -> Grid Coordinates.
        
- **L4 (Emotional):** VAD векторы (High Arousal, Low Valence).
    

### Этап 3: Массовая Индексация (Batch Indexing)

Вместо поштучной вставки используем `Grid::batch_insert`.

1. Создаем `Vec<Token>` (все слова + якоря).
    
2. Загружаем в `Grid`.
    
3. Grid автоматически строит **KD-Tree** (или Octree) для L8 пространства.
    

---

## 4. Генерация Связей (The Weaving)

Используем Grid для поиска связей. Это превращает сложность $O(N^2)$ в $O(N \log N)$.

Rust

```rust

pub fn weave_connections(grid: &Arc<Grid>, tokens: &[Token]) -> Vec<Connection> {
    // Используем Rayon для параллельной обработки
    tokens.par_iter().flat_map(|token| {
        // 1. Спрашиваем у Grid соседей в смысловом пространстве (L8)
        let neighbors = grid.find_k_nearest(
            token.id, 
            CoordinateSpace::L8Abstract, 
            10 // Берем только ТОП-10 самых близких (защита от перегрузки)
        );
        
        let mut conns = Vec::new();
        for (neighbor_id, dist) in neighbors {
            // Сила связи зависит от близости
            let strength = (1.0 - dist / MAX_DIST).max(0.0);
            
            // Создаем связь (Immutable, так как это словарь)
            conns.push(Connection::new_immutable(
                token.id,
                neighbor_id,
                ConnectionType::SimilarTo,
                strength
            ));
        }
        conns
    }).collect()
}

```

---

## 5. Сохранение Состояния (Artifacts)

Bootstrap не просто загружает память, он создает артефакты для будущего использования.

1. **`pca_model.bin`**: Сериализованная матрица PCA.
    
    - _Зачем:_ Когда в будущем система встретит новое слово "Kryptex", она не будет переобучать всю модель. Она прогонит вектор слова через эту матрицу и сразу получит его координаты в Grid.
        
2. **`bootstrap_map.bin`**: `HashMap<String, u32>`.
    
    - _Зачем:_ Быстрый резолвинг слов в ID токенов (для CLI и дебага).
        

---

## 6. Конфигурация (bootstrap.toml)

Ini, TOML

```bash

[source]
embeddings_file = "data/glove.6B.100d.txt"
wordnet_path = "data/wordnet" # Опционально для иерархии

[math]
pca_components = 3
normalize_scale = 3.0 # Растянуть данные на 3 сигмы

[graph]
max_neighbors = 10
min_similarity = 0.6 # Игнорировать слабые связи

[system]
seed = 20251122

```

---

## 7. План Реализации

1. **Dependencies:** Добавить `linfa`, `linfa-reduction`, `fasthash`, `rayon` в `Cargo.toml`.
    
2. **Structs:** Реализовать `BootstrapLibrary` и `BootstrapConfig`.
    
3. **PCA Logic:** Написать загрузчик векторов и пайплайн обучения PCA.
    
4. **Token Factory:** Реализовать генерацию токенов с хешированием ID.
    
5. **Grid Integration:** Написать вызов `grid.find_k_nearest` для генерации связей.
    
6. **Test:** Загрузить малый датасет (100 слов), проверить, что "Cat" и "Dog" рядом, а "Cat" и "Car" далеко.
    

---

Автор: NeuroGraph Architect

Версия: 1.2 (Final)