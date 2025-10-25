# Graph V2.0

## Graph Specification v2.0 — Official Documentation

**Статус:** Official Specification  
**Версия:** 2.0.0  
**Дата:** 2025-10-21  
**Зависимости:** Token v2.0, Connection v1.0, Grid v2.0  
**Целевые языки:** Rust, C++, любой системный язык

## Оглавление

1. [Обзор и философия](#обзор-и-философия)
2. [Архитектура Graph](#архитектура-graph)
3. [Структуры данных](#структуры-данных)
4. [Топологические операции](#топологические-операции)
5. [Алгоритмы обхода](#алгоритмы-обхода)
6. [Навигация по связям](#навигация-по-связям)
7. [Анализ топологии](#анализ-топологии)
8. [Подграфы](#подграфы)
9. [Производительность](#производительность)
10. [Интеграция с модулями](#интеграция-с-модулями)
11. [Инварианты и валидация](#инварианты-и-валидация)
12. [Сериализация](#сериализация)

## 1. Обзор и философия

### 1.1 Определение

**Graph (Граф)** — это модуль NeuroGraph OS, предоставляющий топологическое представление и навигацию по связям между Token. Graph определяет логическую структуру отношений, в то время как Grid определяет пространственную структуру.

Graph является вторичной структурой: он не хранит Token и Connection, а предоставляет эффективные алгоритмы обхода и анализа топологии, используя существующие данные из Grid и Connection.

### 1.2 Природа Graph

Graph — это не хранилище данных. Это вычислительный слой, который:

- Индексирует топологию связей для быстрого обхода
- Предоставляет алгоритмы навигации (BFS, DFS, поиск путей)
- Анализирует топологические свойства (связность, центральность)
- Извлекает и анализирует подграфы

**Аналогия:**
- Grid: пространственные запросы ↔ Graph: топологические запросы
- Grid: "где находится" ↔ Graph: "как связано"

### 1.3 Философия дизайна

**Топологическая навигация:** Graph позволяет отвечать на вопросы типа:
- Какие токены связаны с данным токеном?
- Существует ли путь между двумя токенами?
- Какие токены находятся на расстоянии N шагов?
- Какие токены наиболее центральны в сети?

**Разделение ответственности:**
- **Token:** содержит данные
- **Connection:** определяет отношения
- **Grid:** организует пространственно
- **Graph:** навигирует топологически

**Эффективность:** Graph оптимизирован для:
- Быстрого обхода связей (O(1) доступ к соседям)
- Поиска путей с минимальными затратами
- Анализа структуры без перебора всех узлов

**Интеграция:** Graph тесно интегрирован с Grid для:
- Пространственных эвристик при поиске путей
- Комбинированных запросов (топология + расстояние)
- Визуализации графа в пространстве

### 1.4 Отличия от Grid

| Аспект | Grid | Graph |
|--------|------|-------|
| Определяет | Координатное пространство | Топологическую структуру |
| Хранит | Token (узлы) | Индексы связей |
| Запросы | Пространственные (FindNeighbors) | Топологические (FindPath) |
| Метрика | Евклидово расстояние | Количество шагов (hops) |
| Операции | RangeQuery, KNN | BFS, DFS, PathFind |
| Использует | Connection для полей | Connection для навигации |

### 1.5 Роль в NeuroGraph OS

Graph выполняет следующие функции:

1. **Топологическая навигация** — обход связей между Token
2. **Поиск путей** — нахождение маршрутов в сети отношений
3. **Анализ структуры** — выявление паттернов связности
4. **Извлечение подграфов** — работа с фрагментами сети
5. **Интеграция с рассуждениями** — поддержка логического вывода

## 2. Архитектура Graph

### 2.1 Компоненты системы

Graph состоит из следующих логических компонентов:

**1. Топологический индекс (Topology Index):**
- Adjacency lists (списки смежности)
- Быстрый доступ к соседям узла
- Направленные индексы (in/out)

**2. Менеджер обхода (Traversal Manager):**
- Алгоритмы BFS, DFS
- Поиск путей (Dijkstra, A*)
- Итераторы для ленивого обхода

**3. Анализатор топологии (Topology Analyzer):**
- Вычисление метрик (степень, центральность)
- Обнаружение структур (компоненты, циклы)
- Статистика графа

**4. Менеджер подграфов (Subgraph Manager):**
- Извлечение подграфов
- Вычисление метрик подграфов
- Операции с подграфами

**5. Интеграционный слой (Integration Layer):**
- Работа с Grid для комбинированных запросов
- Использование Connection для типизации связей
- Синхронизация с изменениями топологии

### 2.2 Модель хранения

Graph НЕ хранит узлы и связи, а работает с ссылками:

**Узлы (nodes):**
```
Type: Set<NodeId>
где NodeId = Token.id (uint32)
```

**Связи (edges):**
```
Type: Set<EdgeId>
где EdgeId = Connection.connection_id (string) или hash
```

**Топологический индекс:**
```
adjacency_out: Map<NodeId, List<EdgeId>> // Исходящие
adjacency_in: Map<NodeId, List<EdgeId>>  // Входящие
```

**Разрешение ссылок:**

Для получения данных узла:
```
node_id → Grid.GetNode(node_id) → Token
```

Для получения данных связи:
```
edge_id → ConnectionStore.Get(edge_id) → Connection
```

### 2.3 Синхронизация с Grid и Connection

**При добавлении Token в Grid:**
1. `Grid.InsertNode(token)`
2. Graph получает уведомление (или poll)
3. Graph добавляет node_id в свой индекс узлов

**При добавлении Connection:**
1. `ConnectionStore.Add(connection)`
2. Graph получает уведомление
3. Graph обновляет adjacency lists:
   ```
   adjacency_out[from_id].append(edge_id)
   adjacency_in[to_id].append(edge_id)
   ```

**При удалении Token:**
1. `Grid.RemoveNode(node_id)`
2. Graph удаляет узел и все связанные рёбра
3. Обновление adjacency lists

**При удалении Connection:**
1. `ConnectionStore.Remove(edge_id)`
2. Graph удаляет ребро из adjacency lists

**Стратегии синхронизации:**

**Push (события):**
```
Grid/Connection → emit event → Graph.HandleEvent()
```
- **Преимущества:** мгновенная синхронизация
- **Недостатки:** требует event bus

**Pull (периодический опрос):**
```
Graph периодически запрашивает изменения
```
- **Преимущества:** простота
- **Недостатки:** задержка синхронизации

**Lazy (по запросу):**
```
Graph перестраивает индексы при первом обращении
```
- **Преимущества:** минимальные затраты
- **Недостатки:** задержка при первом запросе

### 2.4 Жизненный цикл Graph

**Инициализация:**
1. Создать пустые индексы (adjacency lists)
2. Подключиться к Grid и ConnectionStore
3. Построить начальные индексы (если данные уже существуют)

**Работа:**
1. Получать уведомления об изменениях
2. Обновлять индексы инкрементально
3. Обрабатывать запросы обхода/анализа
4. Периодически пересчитывать кэшированные метрики

**Завершение:**
1. Сохранить состояние индексов (опционально)
2. Отключиться от источников данных

### 2.5 Направленность графа

Graph поддерживает направленные связи:

**Connection определяет направление:**
```
from_token_id → to_token_id
```

**Graph индексирует в обоих направлениях:**
```
adjacency_out[from_id] = [edge1, edge2, ...] // Исходящие
adjacency_in[to_id] = [edge1, edge2, ...]    // Входящие
```

**Двунаправленные связи:**
```
Если Connection.bidirectional = true:
  Связь доступна в обоих направлениях при обходе
```

**Обход в определённом направлении:**
```
При обходе:
  if connection.bidirectional:
    можно идти как от from → to, так и от to → from

OUTGOING: использовать только adjacency_out
INCOMING: использовать только adjacency_in
BOTH: использовать оба индекса
```

## 3. Структуры данных

### 3.1 NodeId и EdgeId

**NodeId:**
```
Type: uint32
Semantics: Уникальный идентификатор узла (Token.id)
Range: 1 to 4,294,967,295
Special: 0 зарезервирован (означает отсутствие узла)
```

**EdgeId:**
```
Type: Зависит от реализации
```

**Вариант 1 (String-based):**
```
Type: String
Value: Connection.connection_id
Преимущества: Прямое соответствие Connection
Недостатки: Больше памяти, медленнее сравнение
```

**Вариант 2 (Hash-based):**
```
Type: uint64
Value: Hash(Connection.connection_id) или Hash(from_id, to_id, type)
Преимущества: Компактность, быстрое сравнение
Недостатки: Необходимость resolve hash → Connection
```

**Рекомендация:** Hash-based для производительности

### 3.2 Adjacency Lists

**Структура:**
```rust
AdjacencyList {
    outgoing: HashMap<NodeId, Vec<EdgeId>>,
    incoming: HashMap<NodeId, Vec<EdgeId>>,
}
```

**Характеристики:**
- Доступ к соседям: O(1)
- Память: O(E) где E = количество рёбер
- Вставка ребра: O(1) amortized
- Удаление ребра: O(degree)

**Детальная структура:**

Для каждого NodeId хранится:
```
Vec<EdgeId> — список исходящих/входящих рёбер
```

**Пример:**
```
adjacency_out = {
    42: [edge_1, edge_2, edge_5],
    100: [edge_3],
    200: [edge_4, edge_6]
}

adjacency_in = {
    100: [edge_1],
    150: [edge_2, edge_3],
    200: [edge_4, edge_5, edge_6]
}
```

**Оптимизация для плотных узлов:**

Если узел имеет > THRESHOLD исходящих рёбер:
- Использовать `Set<EdgeId>` вместо `Vec<EdgeId>`
- Для быстрой проверки существования ребра: O(1) vs O(n)
- THRESHOLD = 100 (рекомендуемое значение)

### 3.3 NodeSet и EdgeSet

**NodeSet:**
```
Type: HashSet<NodeId>
Usage: Множество узлов (для подграфов, visited sets)
Operations:
  - Insert: O(1)
  - Contains: O(1)
  - Remove: O(1)
```

**EdgeSet:**
```
Type: HashSet<EdgeId>
Usage: Множество рёбер (для подграфов, фильтрации)
Operations: Аналогично NodeSet
```

### 3.4 Path (Путь)

**Определение пути:**
```rust
Path {
    nodes: Vec<NodeId>,      // Последовательность узлов
    edges: Vec<EdgeId>,      // Последовательность рёбер
    total_cost: float,       // Суммарная стоимость
    length: usize,           // Длина пути (количество шагов)
}
```

**Инвариант:**
```
edges.len() = nodes.len() - 1
```

**Пример:**
```
nodes = [1, 2, 5, 10]
edges = [edge_1_2, edge_2_5, edge_5_10]
length = 3 (три шага)
```

**Свойства пути:**

```rust
is_empty() -> bool:
    return nodes.len() = 0

is_valid() -> bool:
    Проверка:
    - edges.len() = nodes.len() - 1
    - Все узлы существуют
    - Все рёбра существуют
    - Рёбра соединяют соответствующие узлы

contains_node(node_id: NodeId) -> bool:
    return node_id in nodes

contains_edge(edge_id: EdgeId) -> bool:
    return edge_id in edges
```

### 3.5 Subgraph (Подграф)

**Структура подграфа:**
```rust
Subgraph {
    subgraph_id: String,                           // Уникальный ID подграфа
    nodes: NodeSet,                                // Узлы подграфа
    edges: EdgeSet,                                // Рёбра подграфа
    
    // Метаданные
    created_at: uint32,                            // Unix timestamp
    source_query: Option<String>,                  // Запрос, создавший подграф
    
    // Кэшированные метрики (опционально)
    cached_metrics: Option<SubgraphMetrics>,
}
```

```rust
SubgraphMetrics {
    node_count: usize,
    edge_count: usize,
    density: float,
    avg_degree: float,
    diameter: Option<usize>,                       // Максимальное расстояние между узлами
    clustering_coefficient: float,
}
```

**Операции с подграфом:**
```rust
contains_node(node_id: NodeId) -> bool
contains_edge(edge_id: EdgeId) -> bool
is_connected() -> bool                             // Проверка связности
get_degree(node_id: NodeId) -> usize
calculate_metrics() -> SubgraphMetrics
```

### 3.6 TraversalState (Состояние обхода)

Для итеративных алгоритмов обхода:

```rust
TraversalState {
    current_node: NodeId,                          // Текущий узел
    visited: NodeSet,                              // Посещённые узлы
    queue: Queue<NodeId>,                          // Очередь для BFS
    stack: Stack<NodeId>,                          // Стек для DFS
    distances: HashMap<NodeId, usize>,             // Расстояния от начала
    predecessors: HashMap<NodeId, NodeId>,         // Предшественники (для восстановления пути)
}
```

**Использование:**

Позволяет приостанавливать и возобновлять обход, а также реализовывать итераторы для ленивого обхода.

## 4. Топологические операции

### 4.1 Базовые операции с узлами

**AddNode(node_id: NodeId):**

```
Назначение: Зарегистрировать узел в топологическом индексе

Предусловия:
  - Узел существует в Grid (Grid.GetNode(node_id) не None)
  - node_id > 0

Алгоритм:
  1. Проверить существование узла в Grid
  2. Добавить node_id в индекс узлов
  3. Инициализировать пустые adjacency lists:
       adjacency_out[node_id] = []
       adjacency_in[node_id] = []

Постусловия:
  - node_id присутствует в Graph
  - Готов для добавления рёбер

Сложность: O(1)
```

**RemoveNode(node_id: NodeId):**

```
Назначение: Удалить узел из топологического индекса

Алгоритм:
  1. Получить все исходящие рёбра: edges_out = adjacency_out[node_id]
  2. Получить все входящие рёбра: edges_in = adjacency_in[node_id]
  3. Для каждого ребра:
       RemoveEdge(edge_id)
  4. Удалить node_id из adjacency_out
  5. Удалить node_id из adjacency_in
  6. Удалить node_id из индекса узлов

Постусловия:
  - node_id отсутствует в Graph
  - Все связанные рёбра удалены

Сложность: O(degree_in + degree_out)
```

**ContainsNode(node_id: NodeId) -> bool:**

```
Назначение: Проверить присутствие узла в Graph

Алгоритм:
  return node_id in adjacency_out

Сложность: O(1)
```

**GetNodeCount() -> usize:**

```
Назначение: Получить количество узлов

Алгоритм:
  return adjacency_out.len()

Сложность: O(1)
```

### 4.2 Базовые операции с рёбрами

**AddEdge(edge_id: EdgeId, from_id: NodeId, to_id: NodeId):**

```
Назначение: Добавить ребро в топологический индекс

Предусловия:
  - Ребро существует в ConnectionStore
  - Оба узла существуют в Graph
  - from_id ≠ to_id (нет самопетель, если не разрешены)

Алгоритм:
  1. Проверить существование узлов
  2. Добавить edge_id в adjacency_out[from_id]
  3. Добавить edge_id в adjacency_in[to_id]
  4. Опционально: обновить кэш степеней узлов

Постусловия:
  - Ребро доступно для обхода
  - degree_out(from_id) увеличился
  - degree_in(to_id) увеличился

Сложность: O(1) amortized
```

**RemoveEdge(edge_id: EdgeId):**

```
Назначение: Удалить ребро из топологического индекса

Предусловия:
  - Ребро существует в Graph

Алгоритм:
  1. Получить Connection для определения from_id и to_id:
       connection = ConnectionStore.Get(edge_id)
  2. Удалить edge_id из adjacency_out[from_id]
  3. Удалить edge_id из adjacency_in[to_id]
  4. Опционально: обновить кэш степеней

Постусловия:
  - Ребро недоступно для обхода
  - degree_out(from_id) уменьшился
  - degree_in(to_id) уменьшился

Сложность: O(degree) для поиска в списке
Оптимизация: O(1) если использовать Set вместо Vec для плотных узлов
```

**ContainsEdge(from_id: NodeId, to_id: NodeId) -> bool:**

```
Назначение: Проверить существование ребра между узлами

Алгоритм:
  1. Получить исходящие рёбра: edges = adjacency_out[from_id]
  2. Для каждого edge_id в edges:
       connection = ConnectionStore.Get(edge_id)
       if connection.to_token_id = to_id:
         return true
  3. return false

Сложность: O(degree_out(from_id))
```

**GetEdgeCount() -> usize:**

```
Назначение: Получить общее количество рёбер

Алгоритм:
  count = 0
  for node_id in adjacency_out:
    count += adjacency_out[node_id].len()
  return count

Сложность: O(V) где V = количество узлов
Оптимизация: Кэшировать значение, обновлять при Add/RemoveEdge
```

### 4.3 Операции со степенями узлов

**GetDegree(node_id: NodeId, direction: Direction) -> usize:**

```
Параметры:
  direction: OUTGOING | INCOMING | BOTH

Алгоритм:
  match direction:
    OUTGOING:
      return adjacency_out[node_id].len()
    INCOMING:
      return adjacency_in[node_id].len()
    BOTH:
      return adjacency_out[node_id].len() + adjacency_in[node_id].len()

Сложность: O(1)
```

**GetHighDegreeNodes(threshold: usize, direction: Direction) -> Vec<NodeId>:**

```
Назначение: Найти узлы с высокой степенью (хабы)

Алгоритм:
  result = []
  for node_id in all_nodes:
    degree = GetDegree(node_id, direction)
    if degree ≥ threshold:
      result.append(node_id)
  return result

Сложность: O(V)
```

### 4.4 Получение соседей

**GetNeighbors(node_id: NodeId, direction: Direction, filter: Option<EdgeFilter>) -> Vec<NodeId>:**

```
Назначение: Получить соседей узла

Параметры:
  direction: направление обхода
  filter: опциональная фильтрация по типу связи, весу и т.д.

Алгоритм:
  1. edges = match direction:
       OUTGOING: adjacency_out[node_id]
       INCOMING: adjacency_in[node_id]
       BOTH: adjacency_out[node_id] + adjacency_in[node_id]
  
  2. neighbors = []
  
  3. Для каждого edge_id в edges:
       connection = ConnectionStore.Get(edge_id)
       
       // Применить фильтр
       if filter is not None:
         if not filter.matches(connection):
           continue
       
       // Определить соседа
       neighbor_id = if direction = OUTGOING:
                       connection.to_token_id
                     else if direction = INCOMING:
                       connection.from_token_id
                     else: // BOTH
                       if connection.from_token_id = node_id:
                         connection.to_token_id
                       else:
                         connection.from_token_id
       
       neighbors.append(neighbor_id)
  
  4. return neighbors

Сложность: O(degree)
```

**EdgeFilter структура:**

```rust
EdgeFilter {
    connection_types: Option<Set<uint8>>,          // Фильтр по типу
    min_weight: Option<float>,                     // Минимальный вес
    max_weight: Option<float>,                     // Максимальный вес
    min_rigidity: Option<float>,                   // Минимальная жёсткость
    active_on_levels: Option<Set<uint8>>,          // Активность на уровнях
    custom_predicate: Option<Fn(Connection) -> bool>, // Пользовательский фильтр
}

matches(connection: Connection) -> bool:
    Проверяет, соответствует ли connection всем критериям фильтра
```

## 5. Алгоритмы обхода

### 5.1 Breadth-First Search (BFS)

**BFS(start_id: NodeId, max_depth: Option<usize>, visitor: Fn(NodeId, usize)):**

```
Назначение: Обход графа в ширину

Параметры:
  start_id: Начальный узел
  max_depth: Максимальная глубина (None = без ограничений)
  visitor: Функция, вызываемая для каждого посещённого узла

Алгоритм:
  1. visited = Set()
  2. queue = Queue()
  3. queue.push((start_id, 0)) // (node_id, depth)
  4. visited.insert(start_id)
  
  5. while queue is not empty:
       (current_id, depth) = queue.pop()
       visitor(current_id, depth)
       
       if max_depth is not None and depth ≥ max_depth:
         continue
       
       neighbors = GetNeighbors(current_id, BOTH)
       for neighbor_id in neighbors:
         if neighbor_id not in visited:
           visited.insert(neighbor_id)
           queue.push((neighbor_id, depth + 1))

Сложность: O(V + E) где V = узлы, E = рёбра
Пространство: O(V)
```

**BFSIterator:**

```
Назначение: Ленивый итератор для BFS обхода

Использование:
  iterator = graph.BFSIterator(start_id)
  for (node_id, depth) in iterator:
    process(node_id)

Преимущества:
  - Не требует обхода всего графа сразу
  - Можно прервать в любой момент
  - Меньше памяти для больших графов
```

### 5.2 Depth-First Search (DFS)

**DFS(start_id: NodeId, max_depth: Option<usize>, visitor: Fn(NodeId, usize)):**

```
Назначение: Обход графа в глубину

Алгоритм:
  1. visited = Set()
  2. stack = Stack()
  3. stack.push((start_id, 0))
  
  4. while stack is not empty:
       (current_id, depth) = stack.pop()
       
       if current_id in visited:
         continue
       
       visited.insert(current_id)
       visitor(current_id, depth)
       
       if max_depth is not None and depth ≥ max_depth:
         continue
       
       neighbors = GetNeighbors(current_id, BOTH)
       for neighbor_id in neighbors.reverse(): // Reverse для сохранения порядка
         if neighbor_id not in visited:
           stack.push((neighbor_id, depth + 1))

Сложность: O(V + E)
Пространство: O(V)
```

**DFSRecursive (альтернатива):**

```rust
DFSRecursive(current_id: NodeId, depth: usize, visited: Set,
             max_depth: Option<usize>, visitor: Fn):
  if current_id in visited:
    return
  
  visited.insert(current_id)
  visitor(current_id, depth)
  
  if max_depth is not None and depth ≥ max_depth:
    return
  
  neighbors = GetNeighbors(current_id, BOTH)
  for neighbor_id in neighbors:
    DFSRecursive(neighbor_id, depth + 1, visited, max_depth, visitor)
```

**Примечание:** Рекурсивная версия элегантнее, но ограничена глубиной стека

### 5.3 Поиск пути (FindPath)

**FindPath(from_id: NodeId, to_id: NodeId, max_depth: Option<usize>) -> Option<Path>:**

```
Назначение: Найти путь между двумя узлами (BFS, кратчайший путь)

Алгоритм:
  1. if from_id = to_id:
       return Path { nodes: [from_id], edges: [], length: 0 }
  
  2. visited = Set()
  3. queue = Queue()
  4. predecessors = Map() // node_id → (predecessor_id, edge_id)
  5. queue.push(from_id)
  6. visited.insert(from_id)
  
  7. while queue is not empty:
       current_id = queue.pop()
       current_depth = get_depth(current_id, predecessors)
       
       if max_depth is not None and current_depth ≥ max_depth:
         continue
       
       neighbors = GetNeighbors(current_id, OUTGOING)
       for (neighbor_id, edge_id) in neighbors_with_edges:
         if neighbor_id = to_id:
           // Путь найден, восстановить
           return reconstruct_path(from_id, to_id, predecessors, edge_id)
         
         if neighbor_id not in visited:
           visited.insert(neighbor_id)
           predecessors[neighbor_id] = (current_id, edge_id)
           queue.push(neighbor_id)
  
  8. return None // Путь не найден
```

**Функция reconstruct_path:**

```rust
path_nodes = []
path_edges = []
current = to_id

while current ≠ from_id:
  path_nodes.push_front(current)
  (prev, edge) = predecessors[current]
  path_edges.push_front(edge)
  current = prev

path_nodes.push_front(from_id)

return Path {
  nodes: path_nodes,
  edges: path_edges,
  length: path_edges.len(),
  total_cost: calculate_cost(path_edges)
}
```

**Сложность:** O(V + E)  
**Пространство:** O(V)

### 5.4 Dijkstra (взвешенный кратчайший путь)

**FindShortestPath(from_id: NodeId, to_id: NodeId, cost_fn: Fn(Connection) -> float) -> Option<Path>:**

```
Назначение: Найти путь с минимальной стоимостью

Параметры:
  cost_fn: Функция вычисления стоимости ребра
           По умолчанию: 1.0 / connection.weight

Алгоритм (Dijkstra):
  1. if from_id = to_id:
       return Path { nodes: [from_id], edges: [], length: 0, total_cost: 0.0 }
  
  2. distances = Map()        // node_id → минимальная стоимость от from_id
  3. predecessors = Map()     // node_id → (predecessor_id, edge_id)
  4. priority_queue = MinHeap() // (cost, node_id)
  
  5. distances[from_id] = 0.0
  6. priority_queue.push((0.0, from_id))
  
  7. while priority_queue is not empty:
       (current_cost, current_id) = priority_queue.pop()
       
       // Ранняя остановка если достигли цели
       if current_id = to_id:
         break
       
       // Пропустить если уже нашли лучший путь
       if current_cost > distances.get(current_id, ∞):
         continue
       
       // Обработать соседей
       neighbors = GetNeighborsWithEdges(current_id, OUTGOING)
       for (neighbor_id, edge_id) in neighbors:
         connection = ConnectionStore.Get(edge_id)
         edge_cost = cost_fn(connection)
         new_cost = current_cost + edge_cost
         
         if new_cost < distances.get(neighbor_id, ∞):
           distances[neighbor_id] = new_cost
           predecessors[neighbor_id] = (current_id, edge_id)
           priority_queue.push((new_cost, neighbor_id))
  
  8. if to_id not in predecessors and to_id ≠ from_id:
       return None // Путь не найден
  
  9. return reconstruct_path(from_id, to_id, predecessors)
```

**Сложность:** O((V + E) log V) с binary heap  
**Пространство:** O(V)

### 5.5 A* (эвристический поиск)

**FindPathAStar(from_id: NodeId, to_id: NodeId, heuristic: Fn(NodeId, NodeId) -> float) -> Option<Path>:**

```
Назначение: Найти путь с использованием эвристики

Параметры:
  heuristic: Функция оценки расстояния до цели
             Должна быть допустимой (admissible): h(n) ≤ истинное расстояние

Алгоритм:
  1. if from_id = to_id:
       return Path { nodes: [from_id], edges: [], length: 0, total_cost: 0.0 }
  
  2. g_scores = Map()      // node_id → стоимость пути от from_id
  3. f_scores = Map()      // node_id → оценка полной стоимости (g + h)
  4. predecessors = Map()
  5. priority_queue = MinHeap() // (f_score, node_id)
  6. open_set = Set()
  
  7. g_scores[from_id] = 0.0
  8. f_scores[from_id] = heuristic(from_id, to_id)
  9. priority_queue.push((f_scores[from_id], from_id))
  10. open_set.insert(from_id)
  
  11. while priority_queue is not empty:
        (current_f, current_id) = priority_queue.pop()
        open_set.remove(current_id)
        
        if current_id = to_id:
          return reconstruct_path(from_id, to_id, predecessors)
        
        neighbors = GetNeighborsWithEdges(current_id, OUTGOING)
        for (neighbor_id, edge_id) in neighbors:
          connection = ConnectionStore.Get(edge_id)
          edge_cost = 1.0 / connection.weight // Или другая функция стоимости
          tentative_g = g_scores[current_id] + edge_cost
          
          if tentative_g < g_scores.get(neighbor_id, ∞):
            predecessors[neighbor_id] = (current_id, edge_id)
            g_scores[neighbor_id] = tentative_g
            h = heuristic(neighbor_id, to_id)
            f_scores[neighbor_id] = tentative_g + h
            
            if neighbor_id not in open_set:
              priority_queue.push((f_scores[neighbor_id], neighbor_id))
              open_set.insert(neighbor_id)
  
  12. return None
```

**Сложность:** O((V + E) log V) в худшем случае, часто намного лучше с хорошей эвристикой

**Эвристики для A*:**

**Пространственная эвристика (использует Grid):**
```rust
spatial_heuristic(current_id: NodeId, goal_id: NodeId) -> float:
  // Оценить количество шагов на основе пространственного расстояния
  // Для уровня L (например, L8 Abstract):
  distance = Grid.Distance(current_id, goal_id, level=L)
  return distance / MAX_STEP_DISTANCE
```

**Топологическая эвристика:**
```rust
degree_heuristic(current_id: NodeId, goal_id: NodeId) -> float:
  // Узлы с высокой степенью ближе друг к другу в "хабовой" топологии
  degree_current = GetDegree(current_id, BOTH)
  degree_goal = GetDegree(goal_id, BOTH)
  return 1.0 / (1.0 + min(degree_current, degree_goal))
```

**Нулевая эвристика (A* = Dijkstra):**
```rust
zero_heuristic(current_id: NodeId, goal_id: NodeId) -> float:
  return 0.0
```

### 5.6 Обход в заданном радиусе

**GetNodesWithinDistance(start_id: NodeId, max_distance: usize, filter: Option<EdgeFilter>) -> Vec<(NodeId, usize)>:**

```
Назначение: Получить все узлы на расстоянии ≤ max_distance шагов

Алгоритм (BFS с ограничением глубины):
  1. result = []
  2. visited = Set()
  3. queue = Queue()
  4. queue.push((start_id, 0))
  5. visited.insert(start_id)
  
  6. while queue is not empty:
       (current_id, distance) = queue.pop()
       result.append((current_id, distance))
       
       if distance ≥ max_distance:
         continue
       
       neighbors = GetNeighbors(current_id, BOTH, filter)
       for neighbor_id in neighbors:
         if neighbor_id not in visited:
           visited.insert(neighbor_id)
           queue.push((neighbor_id, distance + 1))
  
  7. return result

Возвращает: Список пар (node_id, distance от start_id)

Сложность: O(V + E) в пределах радиуса
```

## 6. Навигация по связям

### 6.1 GetConnectedComponent

**GetConnectedComponent(node_id: NodeId) -> Set<NodeId>:**

```
Назначение: Получить компоненту связности, содержащую узел

Алгоритм (BFS в обе стороны):
  1. component = Set()
  2. queue = Queue()
  3. queue.push(node_id)
  4. component.insert(node_id)
  
  5. while queue is not empty:
       current_id = queue.pop()
       
       // Обход в обоих направлениях
       neighbors = GetNeighbors(current_id, BOTH)
       for neighbor_id in neighbors:
         if neighbor_id not in component:
           component.insert(neighbor_id)
           queue.push(neighbor_id)
  
  6. return component

Сложность: O(V_c + E_c) где V_c, E_c = размер компоненты
```

### 6.2 GetAllConnectedComponents

**GetAllConnectedComponents() -> Vec<Set<NodeId>>:**

```
Назначение: Найти все компоненты связности в графе

Алгоритм:
  1. components = []
  2. visited = Set()
  
  3. for node_id in all_nodes:
       if node_id not in visited:
         component = GetConnectedComponent(node_id)
         components.append(component)
         visited.update(component)
  
  4. return components

Сложность: O(V + E)
```

### 6.3 IsConnected

**IsConnected(node_a: NodeId, node_b: NodeId) -> bool:**

```
Назначение: Проверить, существует ли путь между узлами

Алгоритм:
  path = FindPath(node_a, node_b, max_depth=None)
  return path is not None

Оптимизация: Ранний выход при нахождении пути

Сложность: O(V + E) в худшем случае
```

### 6.4 GetShortestDistance

**GetShortestDistance(from_id: NodeId, to_id: NodeId) -> Option<usize>:**

```
Назначение: Получить длину кратчайшего пути (количество шагов)

Алгоритм:
  path = FindPath(from_id, to_id, max_depth=None)
  if path is None:
    return None
  return Some(path.length)

Сложность: O(V + E)
```

### 6.5 GetCommonNeighbors

**GetCommonNeighbors(node_a: NodeId, node_b: NodeId, direction: Direction) -> Set<NodeId>:**

```
Назначение: Найти общих соседей двух узлов

Алгоритм:
  1. neighbors_a = GetNeighbors(node_a, direction).to_set()
  2. neighbors_b = GetNeighbors(node_b, direction).to_set()
  3. return neighbors_a.intersection(neighbors_b)

Сложность: O(degree_a + degree_b)
```

### 6.6 FindAllPaths

**FindAllPaths(from_id: NodeId, to_id: NodeId, max_paths: Option<usize>, max_length: Option<usize>) -> Vec<Path>:**

```
Назначение: Найти все пути между узлами (с ограничениями)

Параметры:
  max_paths: Максимальное количество путей (для ограничения вычислений)
  max_length: Максимальная длина пути

Алгоритм (DFS с backtracking):
  paths = []
  current_path = []
  visited = Set()
  
  DFS_FindPaths(from_id, to_id, current_path, visited, paths, max_paths, max_length)
  return paths
```

**Функция DFS_FindPaths:**

```rust
1. if max_paths is not None and paths.len() >= max_paths:
     return // Достигнут лимит

2. if current_path.len() ≥ max_length:
     return // Путь слишком длинный

3. current_path.append(current_id)
4. visited.insert(current_id)

5. if current_id = to_id:
     paths.append(current_path.clone())
     visited.remove(current_id)
     current_path.pop()
     return

6. neighbors = GetNeighbors(current_id, OUTGOING)

7. for neighbor_id in neighbors:
     if neighbor_id not in visited:
       DFS_FindPaths(neighbor_id, to_id, current_path, visited, 
                     paths, max_paths, max_length)

8. visited.remove(current_id)
9. current_path.pop()
```

**Предупреждение:** Может быть экспоненциально медленным для плотных графов  
**Сложность:** O(V!) в худшем случае

## 7. Анализ топологии

### 7.1 Метрики графа

**CalculateGraphMetrics() -> GraphMetrics:**

```rust
GraphMetrics {
    node_count: usize,
    edge_count: usize,
    density: float,
    average_degree: float,
    max_degree: usize,
    min_degree: usize,
    connected_components: usize,
    largest_component_size: usize,
    diameter: Option<usize>,          // Максимальное расстояние между узлами
    average_path_length: Option<float>,
    clustering_coefficient: float,
}
```

**Вычисление:**

```rust
node_count = GetNodeCount()
edge_count = GetEdgeCount()
density = edge_count / (node_count × (node_count - 1))  // Для направленного графа

degrees = [GetDegree(n, BOTH) for n in all_nodes]
average_degree = mean(degrees)
max_degree = max(degrees)
min_degree = min(degrees)

components = GetAllConnectedComponents()
connected_components = components.len()
largest_component_size = max(c.len() for c in components)

diameter = CalculateDiameter()  // Опционально, тяжёлая операция
clustering_coefficient = CalculateClusteringCoefficient()
```

**Сложность:**
- O(V + E) для базовых метрик
- O(V²) для diameter

### 7.2 Степени узлов

**GetDegreeDistribution() -> Map<usize, usize>:**

```
Назначение: Получить распределение степеней узлов

Возвращает: Map { степень → количество узлов с этой степенью }

Алгоритм:
  distribution = Map()
  for node_id in all_nodes:
    degree = GetDegree(node_id, BOTH)
    distribution[degree] = distribution.get(degree, 0) + 1
  return distribution

Сложность: O(V)
```

**GetHubNodes(top_k: usize, direction: Direction) -> Vec<(NodeId, usize)>:**

```
Назначение: Найти узлы с наибольшей степенью (хабы)

Алгоритм:
  nodes_with_degrees = [
    (node_id, GetDegree(node_id, direction))
    for node_id in all_nodes
  ]
  nodes_with_degrees.sort_by_key(|(_, degree)| degree, reverse=True)
  return nodes_with_degrees[0..top_k]

Сложность: O(V log V)
```

### 7.3 Центральность узлов

**Degree Centrality:**

```rust
CalculateDegreeCentrality(node_id: NodeId) -> float:
  // Назначение: Нормализованная степень узла
  degree = GetDegree(node_id, BOTH)
  max_possible_degree = GetNodeCount() - 1
  return degree / max_possible_degree

// Диапазон: [0.0, 1.0]
```

**Betweenness Centrality:**

```rust
CalculateBetweennessCentrality(node_id: NodeId) -> float:
  // Назначение: Сколько кратчайших путей проходят через узел
  
  // Алгоритм (упрощённый):
  paths_through = 0
  total_paths = 0
  
  for source in sample_nodes:
    for target in sample_nodes:
      if source = target:
        continue
      
      path = FindPath(source, target)
      if path is not None:
        total_paths += 1
        if node_id in path.nodes and node_id ≠ source and node_id ≠ target:
          paths_through += 1
  
  if total_paths = 0:
    return 0.0
  
  return paths_through / total_paths

// Примечание: Полный алгоритм Брандеса O(V×E), 
// здесь упрощённая версия с сэмплированием

// Сложность: O(sample_size² × (V + E))
```

**Closeness Centrality:**

```rust
CalculateClosenessCentrality(node_id: NodeId) -> float:
  // Назначение: Обратная средняя длина пути до всех узлов
  
  // Алгоритм:
  total_distance = 0
  reachable_count = 0
  
  for target_id in all_nodes:
    if target_id = node_id:
      continue
    
    distance = GetShortestDistance(node_id, target_id)
    if distance is not None:
      total_distance += distance
      reachable_count += 1
  
  if reachable_count = 0:
    return 0.0
  
  average_distance = total_distance / reachable_count
  return 1.0 / average_distance

// Сложность: O(V × (V + E))
```

**CalculateAllCentralities(centrality_type: CentralityType) -> Map<NodeId, float>:**

```
Назначение: Вычислить центральность для всех узлов

Параметры:
  centrality_type: DEGREE | BETWEENNESS | CLOSENESS

Возвращает: Map { node_id → centrality_score }

Сложность: Зависит от типа
```

### 7.4 Кластеризация

**CalculateClusteringCoefficient(node_id: NodeId) -> float:**

```
Назначение: Локальный коэффициент кластеризации узла

Определение: Доля рёбер между соседями узла от всех возможных

Алгоритм:
  neighbors = GetNeighbors(node_id, BOTH)
  k = neighbors.len()
  
  if k < 2:
    return 0.0 // Недостаточно соседей для кластера
  
  // Подсчитать рёбра между соседями
  edges_between_neighbors = 0
  for i in 0..neighbors.len():
    for j in (i+1)..neighbors.len():
      if ContainsEdge(neighbors[i], neighbors[j]) or
         ContainsEdge(neighbors[j], neighbors[i]):
        edges_between_neighbors += 1
  
  max_possible_edges = k × (k - 1) / 2
  return edges_between_neighbors / max_possible_edges

Сложность: O(k²) где k = степень узла
```

**CalculateGlobalClusteringCoefficient() -> float:**

```
Назначение: Глобальный коэффициент кластеризации графа

Алгоритм:
  sum_local_coefficients = 0.0
  count = 0
  
  for node_id in all_nodes:
    coefficient = CalculateClusteringCoefficient(node_id)
    sum_local_coefficients += coefficient
    count += 1
  
  if count = 0:
    return 0.0
  
  return sum_local_coefficients / count

Сложность: O(V × avg_degree²)
```

### 7.5 Обнаружение циклов

**ContainsCycle() -> bool:**

```
Назначение: Проверить наличие циклов в графе

Алгоритм (DFS с отслеживанием состояний):
  visited = Set()
  rec_stack = Set() // Узлы в текущем рекурсивном стеке
  
  for node_id in all_nodes:
    if node_id not in visited:
      if DFS_HasCycle(node_id, visited, rec_stack):
        return true
  
  return false
```

**Функция DFS_HasCycle:**

```rust
1. visited.insert(node_id)
2. rec_stack.insert(node_id)

3. neighbors = GetNeighbors(node_id, OUTGOING)

4. for neighbor_id in neighbors:
     if neighbor_id not in visited:
       if DFS_HasCycle(neighbor_id, visited, rec_stack):
         return true
     else if neighbor_id in rec_stack:
       return true // Цикл найден

5. rec_stack.remove(node_id)
6. return false

// Сложность: O(V + E)
```

**FindCycle() -> Option<Path>:**

```
Назначение: Найти один цикл в графе

Возвращает: Path представляющий цикл (где nodes[0] == nodes[last])

Алгоритм: Модификация DFS с сохранением пути

Сложность: O(V + E)
```

### 7.6 Топологическая сортировка

**TopologicalSort() -> Option<Vec<NodeId>>:**

```
Назначение: Топологическая сортировка для DAG (направленного ациклического графа)

Возвращает:
  Some(sorted_nodes) если граф ациклический
  None если граф содержит циклы

Алгоритм (Kahn's algorithm):
  1. in_degree = Map() // node_id → входящая степень
  2. for node_id in all_nodes:
       in_degree[node_id] = GetDegree(node_id, INCOMING)
  
  3. queue = Queue()
  4. for node_id in all_nodes:
       if in_degree[node_id] = 0:
         queue.push(node_id)
  
  5. result = []
  
  6. while queue is not empty:
       current_id = queue.pop()
       result.append(current_id)
       
       neighbors = GetNeighbors(current_id, OUTGOING)
       for neighbor_id in neighbors:
         in_degree[neighbor_id] -= 1
         if in_degree[neighbor_id] = 0:
           queue.push(neighbor_id)
  
  7. if result.len() ≠ GetNodeCount():
       return None // Граф содержит цикл
  
  8. return Some(result)

Сложность: O(V + E)
```

## 8. Подграфы

### 8.1 Извлечение подграфа

**ExtractSubgraph(node_ids: Set<NodeId>) -> Subgraph:**

```
Назначение: Извлечь подграф, индуцированный множеством узлов

Определение: Подграф содержит:
  - Все узлы из node_ids
  - Все рёбра, где оба конца в node_ids

Алгоритм:
  1. subgraph = Subgraph {
       subgraph_id: generate_uuid(),
       nodes: node_ids.clone(),
       edges: Set(),
       created_at: current_time(),
       source_query: None,
       cached_metrics: None
     }
  
  2. Для каждого node_id в node_ids:
       // Получить исходящие рёбра
       outgoing = adjacency_out[node_id]
       for edge_id in outgoing:
         connection = ConnectionStore.Get(edge_id)
         if connection.to_token_id in node_ids:
           subgraph.edges.insert(edge_id)
  
  3. return subgraph

Сложность: O(sum of degrees of nodes in node_ids)
```

**ExtractNeighborhood(center_id: NodeId, radius: usize) -> Subgraph:**

```
Назначение: Извлечь окрестность узла (ego-network)

Алгоритм:
  1. nodes_within = GetNodesWithinDistance(center_id, radius)
  2. node_ids = nodes_within.map(|(id, _)| id).collect()
  3. return ExtractSubgraph(node_ids)

Сложность: O(V + E) в пределах радиуса
```

### 8.2 Метрики подграфа

**CalculateSubgraphMetrics(subgraph: Subgraph) -> SubgraphMetrics:**

```rust
SubgraphMetrics {
    node_count: usize,
    edge_count: usize,
    density: float,
    avg_degree: float,
    diameter: Option<usize>,
    clustering_coefficient: float,
}
```

**Вычисление:**

```rust
node_count = subgraph.nodes.len()
edge_count = subgraph.edges.len()
density = edge_count / (node_count × (node_count - 1))

degrees = [GetDegree_InSubgraph(n, subgraph) for n in subgraph.nodes]
avg_degree = mean(degrees)

diameter = CalculateDiameter_InSubgraph(subgraph)
clustering_coefficient = CalculateClusteringCoefficient_InSubgraph(subgraph)

return SubgraphMetrics { ... }
```

**Сложность:** O(V_sub + E_sub)

**GetDegree_InSubgraph(node_id: NodeId, subgraph: Subgraph, direction: Direction) -> usize:**

```
Назначение: Получить степень узла внутри подграфа

Алгоритм:
  count = 0
  edges = match direction:
    OUTGOING: adjacency_out[node_id]
    INCOMING: adjacency_in[node_id]
    BOTH: adjacency_out[node_id] + adjacency_in[node_id]
  
  for edge_id in edges:
    if edge_id in subgraph.edges:
      count += 1
  
  return count

Сложность: O(degree)
```

### 8.3 Операции с подграфами

**MergeSubgraphs(subgraph1: Subgraph, subgraph2: Subgraph) -> Subgraph:**

```
Назначение: Объединить два подграфа

Алгоритм:
  merged = Subgraph {
    subgraph_id: generate_uuid(),
    nodes: subgraph1.nodes.union(subgraph2.nodes),
    edges: subgraph1.edges.union(subgraph2.edges),
    created_at: current_time(),
    source_query: None,
    cached_metrics: None
  }
  return merged

Сложность: O(|V1| + |V2| + |E1| + |E2|)
```

**IntersectSubgraphs(subgraph1: Subgraph, subgraph2: Subgraph) -> Subgraph:**

```
Назначение: Найти пересечение подграфов

Алгоритм:
  common_nodes = subgraph1.nodes.intersection(subgraph2.nodes)
  common_edges = subgraph1.edges.intersection(subgraph2.edges)
  
  // Отфильтровать рёбра, где оба конца не в common_nodes
  valid_edges = Set()
  for edge_id in common_edges:
    connection = ConnectionStore.Get(edge_id)
    if connection.from_token_id in common_nodes and
       connection.to_token_id in common_nodes:
      valid_edges.insert(edge_id)
  
  intersection = Subgraph {
    subgraph_id: generate_uuid(),
    nodes: common_nodes,
    edges: valid_edges,
    created_at: current_time(),
    source_query: None,
    cached_metrics: None
  }
  
  return intersection

Сложность: O(min(|V1|, |V2|) + min(|E1|, |E2|))
```

**IsSubgraphOf(smaller: Subgraph, larger: Subgraph) -> bool:**

```
Назначение: Проверить, является ли один подграф частью другого

Алгоритм:
  nodes_subset = smaller.nodes.is_subset(larger.nodes)
  edges_subset = smaller.edges.is_subset(larger.edges)
  return nodes_subset and edges_subset

Сложность: O(|V_smaller| + |E_smaller|)
```

### 8.4 Поиск подграфов

**FindDenseSubgraphs(min_density: float, min_size: usize) -> Vec<Subgraph>:**

```
Назначение: Найти плотные подграфы (кластеры)

Определение: Подграф с density >= min_density и size >= min_size

Алгоритм (эвристический):
  1. components = GetAllConnectedComponents()
  2. dense_subgraphs = []
  
  3. Для каждой component:
       if component.len() < min_size:
         continue
       
       subgraph = ExtractSubgraph(component)
       metrics = CalculateSubgraphMetrics(subgraph)
       
       if metrics.density ≥ min_density:
         dense_subgraphs.append(subgraph)
  
  4. return dense_subgraphs

Примечание: Полный алгоритм поиска максимально плотных подграфов (NP-hard)
Сложность: O(V + E) для эвристики
```

**FindKCore(k: usize) -> Subgraph:**

```
Назначение: Найти k-core подграф

Определение: Максимальный подграф, где каждый узел имеет степень >= k

Алгоритм:
  1. nodes = all_nodes.clone()
  2. changed = true
  
  3. while changed:
       changed = false
       to_remove = []
       
       for node_id in nodes:
         degree = count edges where both ends in nodes
         if degree < k:
           to_remove.append(node_id)
           changed = true
       
       nodes.remove_all(to_remove)
  
  4. return ExtractSubgraph(nodes)

Сложность: O(E) iterations, O(V×E) total
```

## 9. Производительность

### 9.1 Сложность операций

**Базовые операции:**

| Операция | Временная сложность | Пространственная сложность | Примечания |
|----------|---------------------|---------------------------|------------|
| AddNode | O(1) | O(1) | Добавление в индексы |
| RemoveNode | O(degree) | O(1) | Удаление всех связанных рёбер |
| AddEdge | O(1) amortized | O(1) | Вставка в adjacency lists |
| RemoveEdge | O(degree) | O(1) | Поиск в списке смежности |
| GetDegree | O(1) | O(1) | Прямой доступ к size списка |
| GetNeighbors | O(degree) | O(degree) | Итерация по списку |

**Обход графа:**

| Операция | Временная сложность | Пространственная сложность |
|----------|---------------------|---------------------------|
| BFS | O(V + E) | O(V) |
| DFS | O(V + E) | O(V) |
| FindPath | O(V + E) | O(V) |
| Dijkstra | O((V + E) log V) | O(V) |
| A* | O((V + E) log V) | O(V) |

**Анализ топологии:**

| Операция | Временная сложность | Примечания |
|----------|---------------------|------------|
| GetConnectedComponent | O(V_c + E_c) | Размер компоненты |
| GetAllConnectedComponents | O(V + E) | Полный обход |
| CalculateDegreeCentrality | O(1) | Один узел |
| CalculateBetweennessCentrality | O(V × E) | Алгоритм Брандеса |
| CalculateClosenessCentrality | O(V × (V + E)) | Все кратчайшие пути |
| TopologicalSort | O(V + E) | Алгоритм Кана |
| ContainsCycle | O(V + E) | DFS |

**Подграфы:**

| Операция | Временная сложность | Примечания |
|----------|---------------------|------------|
| ExtractSubgraph | O(Σ degrees) | Сумма степеней узлов |
| CalculateSubgraphMetrics | O(V_sub + E_sub) | Размер подграфа |
| MergeSubgraphs | O(V1 + V2 + E1 + E2) | Объединение множеств |

### 9.2 Оценка памяти

**Индексы:**

```
Adjacency Lists:
  Per node: 2 × Vec<EdgeId> (outgoing + incoming)
  Vec overhead: ~24 bytes per vec
  EdgeId: 8 bytes (если uint64 hash)
  
Для графа с V узлов, E рёбер:
  Nodes: V × 48 bytes
  Edges in lists: 2 × E × 8 bytes (каждое ребро в двух списках)
  Total: V × 48 + E × 16 bytes
  
Пример (100,000 узлов, 1,000,000 рёбер):
  Nodes: 100k × 48 = 4.8 MB
  Edges: 1M × 16 = 16 MB
  Total: ~21 MB
```

**Кэши:**

```
Метрики (опционально):
  Degree cache: V × (2 × 4 bytes) = V × 8 bytes
  Centrality cache: V × 8 bytes (float64)
  Connected components: V × 4 bytes (component_id)
  
  Total per node: ~48 bytes + (degree × 8 bytes)

Total cache: V × 20 bytes

Для 100k узлов: ~2 MB
```

**Подграфы:**

```
Subgraph:
  NodeSet: ~24 bytes overhead + nodes.len() × 4 bytes
  EdgeSet: ~24 bytes overhead + edges.len() × 8 bytes
  Metadata: ~100 bytes
  
Типичный подграф (1000 узлов, 5000 рёбер):
  ~48 + 4k + 40k + 100 = ~44 KB
```

**Итого для системы с 100k узлов, 1M рёбер:**

```
Base Graph:  ~21 MB
Caches:      ~2 MB
10 подграфов: ~440 KB
──────────────────
Total:       ~24 MB
```

### 9.3 Оптимизации

**Индексация:**

1. Использовать Vec для узлов с низкой степенью (<100)
2. Переключаться на HashSet для высокой степени (≥100)
3. Это ускоряет ContainsEdge для хабов

**Кэширование:**

```rust
// Кэш степеней узлов:
degree_cache: HashMap<NodeId, (usize, usize)> // (in, out)

// Обновление при AddEdge/RemoveEdge: O(1)
// Доступ к GetDegree: O(1) вместо O(1) для len()
```

**Выгода:** Минимальная для простых степеней, но полезна для частых запросов

```rust
// Кэш компонент связности:
component_cache: HashMap<NodeId, ComponentId>

// Invalidation: При добавлении/удалении рёбер между компонентами
```

**Выгода:** O(1) вместо O(V + E) для повторных запросов

**Lazy evaluation:**

```
Метрики графа/подграфа:
  - Вычислять только при запросе
  - Кэшировать результаты с TTL
  - Инвалидировать при изменении топологии
```

**Batch операции:**

```rust
BatchAddEdges(edges: Vec<(EdgeId, NodeId, NodeId)>):
  1. Сортировать edges по from_id для локальности доступа
  2. Заблокировать индексы для записи
  3. Добавить все рёбра за один раз
  4. Разблокировать
```

**Выгода:** Минимизация lock contention в многопоточной среде

**Итераторы:**

```
Ленивые итераторы для обхода:
  BFSIterator, DFSIterator

Преимущества:
  - Не требуют обхода всего графа сразу
  - Можно прервать в любой момент
  - Экономия памяти для больших графов
```

### 9.4 Параллелизация

**Thread-safety стратегии:**

**Read-Write Locks:**

```rust
adjacency_out: RwLock<HashMap<NodeId, Vec<EdgeId>>>
adjacency_in: RwLock<HashMap<NodeId, Vec<EdgeId>>>

// Множественные read операции:
- GetNeighbors
- GetDegree
- FindPath (read-only)

// Эксклюзивные write операции:
- AddEdge, RemoveEdge
- AddNode, RemoveNode
```

**Lock-free для read-only:**

```
Snapshot модель:
  1. Создать immutable snapshot индексов
  2. Выполнять read операции на snapshot
  3. Write операции создают новую версию

Подходит для систем с редкими изменениями
```

**Партиционирование:**

```
Разделить граф на партиции по node_id ranges:
  Partition 1: nodes [0, 10000)
  Partition 2: nodes [10000, 20000)
  ...

Каждая партиция с отдельным lock
Операции внутри партиции параллельны
Операции между партициями требуют двух locks
```

**Параллельные алгоритмы:**

```
Parallel BFS:
  1. Использовать thread-safe очередь
  2. Множественные threads обрабатывают узлы параллельно
  3. Синхронизация через atomic visited set

Ускорение: ~2-4x на 8 cores для больших графов
```

```
Parallel Connected Components:
  Алгоритм label propagation
  Каждый thread обрабатывает subset узлов

Ускорение: ~3-6x на 8 cores
```

### 9.5 Benchmarking

**Ключевые метрики:**

```
Throughput:
  - Edges added per second
  - Paths found per second
  - Subgraphs extracted per second

Latency:
  - p50, p95, p99 для FindPath
  - Average BFS time
  - Average subgraph extraction time

Memory:
  - Bytes per node
  - Bytes per edge
  - Cache overhead

Scalability:
  - Performance vs V (количество узлов)
  - Performance vs E (количество рёбер)
  - Multi-thread scaling
```

**Целевые показатели:**

```
Для графа с 100k узлов, 1M рёбер на современном CPU (2024):

Edge operations:     >1,000,000 ops/sec
FindPath (BFS):      <1ms для paths длиной <10
GetNeighbors:        <10 μs для degree <100
Connected Components: <100ms
Subgraph extraction:  <50ms для 1000 узлов

Memory:               <50 MB total
Cache hit rate:       >80%
```

## 10. Интеграция с модулями

### 10.1 Token v2.0

Graph не хранит Token, а работает с ссылками:

```
Получение данных узла:
  node_id → Grid.GetNode(node_id) → Token
```

**Graph требует от Token:**
- id: uint32 (уникальный идентификатор)

**Graph НЕ использует напрямую:**
- coordinates (это для Grid)
- field_radius/strength (это для Connection)
- weight (опционально для взвешенных алгоритмов)

**Протокол взаимодействия:**

```
При создании узла:
  1. Token создаётся
  2. Grid.InsertNode(token)
  3. Graph.AddNode(token.id)

При удалении узла:
  1. Grid.RemoveNode(node_id)
  2. Graph получает уведомление
  3. Graph.RemoveNode(node_id) // Каскадное удаление рёбер
```

### 10.2 Connection v1.0

Graph использует Connection для:
- Определения топологии (from/to)
- Типизации рёбер (connection_type)
- Взвешивания (weight, rigidity)
- Фильтрации при обходе

**Протокол взаимодействия:**

```
При создании связи:
  1. Connection создаётся
  2. ConnectionStore.Add(connection)
  3. Graph.AddEdge(connection.id или hash, from_id, to_id)

При удалении связи:
  1. ConnectionStore.Remove(connection_id)
  2. Graph получает уведомление
  3. Graph.RemoveEdge(edge_id)

Получение данных ребра:
  edge_id → ConnectionStore.Get(edge_id) → Connection
```

**Использование Connection для алгоритмов:**

```rust
// Взвешенный поиск пути:
cost_fn = lambda conn: 1.0 / conn.weight
path = FindShortestPath(from_id, to_id, cost_fn)

// Фильтрация по типу:
filter = EdgeFilter {
  connection_types: Some([CAUSALITY, DEPENDENCY])
}
neighbors = GetNeighbors(node_id, OUTGOING, Some(filter))

// Направленность:
if connection.bidirectional:
  // Ребро доступно в обоих направлениях
```

### 10.3 Grid v2.0

**Комбинированные запросы (Graph + Grid):**

**Пространственно-топологический поиск:**

```rust
FindNearbyConnectedNodes(center_id: NodeId, spatial_radius: float, 
                         level: uint8, max_hops: usize) -> Vec<NodeId>:
  // Назначение: Найти узлы, которые близки И топологически И пространственно
  
  // Алгоритм:
  1. spatial_neighbors = Grid.FindNeighbors(center_id, level, spatial_radius)
  2. topological_neighbors = Graph.GetNodesWithinDistance(center_id, max_hops)
  3. return spatial_neighbors.intersection(topological_neighbors)

// Использование: Поиск семантически связанных объектов в локальной области
```

**Эвристики для A*:**

```rust
spatial_heuristic(current_id: NodeId, goal_id: NodeId) -> float:
  // Использует Grid.Distance для оценки количества шагов:
  distance = Grid.Distance(current_id, goal_id, level=L8_ABSTRACT)
  avg_connection_distance = estimate_avg_connection_distance()
  estimated_hops = distance / avg_connection_distance
  return estimated_hops
```

**Визуализация графа в пространстве:**

```
Для отображения графа в 3D пространстве:
  1. Получить coordinates из Token для каждого узла
  2. Отрисовать узлы в позициях Grid
  3. Отрисовать рёбра между позициями
  4. Использовать Grid.IdentifyFields для выделения кластеров
```

**Синхронизация:**

```
При миграции Token в Grid:
  1. Connection генерирует силы
  2. Token.coordinates обновляются
  3. Grid.UpdateNodeCoordinates()
  4. Graph НЕ требует обновления (ID не изменился)

Топология остаётся неизменной при пространственной миграции
```

### 10.4 FSC (будущий модуль)

**Предполагаемая интеграция:**

```
FSC будет использовать Graph для:
  - Навигации по семантическим отношениям
  - Поиска путей в концептуальном пространстве
  - Построения иерархий концептов

Graph будет использовать FSC для:
  - Определения семантической близости для эвристик
  - Автоматического создания связей между похожими концептами
```

**Пример:**

```rust
CreateSemanticLink(token_a: Token, token_b: Token):
  // FSC вычисляет семантическую близость
  similarity = FSC.ComputeSimilarity(token_a, token_b)
  
  if similarity > THRESHOLD:
    connection = Connection {
      from_token_id: token_a.id,
      to_token_id: token_b.id,
      connection_type: SIMILARITY,
      weight: similarity,
      ...
    }
    ConnectionStore.Add(connection)
    Graph.AddEdge(...)
```

### 10.5 External systems

Graph экспортирует данные для:

**Визуализация:**

```rust
ExportForVisualization(format: VisualizationFormat) -> String:
  // Форматы:
  - GraphML (XML)
  - GML (Graph Modelling Language)
  - DOT (Graphviz)
  - JSON (D3.js compatible)
  - GEXF (Gephi)
  
  // Включает:
  - Узлы (ID, метаданные из Token)
  - Рёбра (from, to, тип, вес)
  - Позиции из Grid (если доступно)
  - Вычисленные метрики (centrality, clustering)
```

**Анализ:**

```rust
ExportToNetworkX() -> Dict:
  // Экспорт в формат, совместимый с Python NetworkX
  // Для продвинутого анализа и ML

ExportToIGraph() -> Dict:
  // Экспорт для R igraph package
```

**Persistence:**

```
Graph может сохранять состояние индексов:
  - Adjacency lists
  - Кэшированные метрики
  - Подграфы

Для быстрой загрузки без пересчёта
```

## 11. Инварианты и валидация

### 11.1 Системные инварианты

**Инвариант 1: Согласованность узлов**

```
Для каждого node_id в adjacency_out:
  assert node_id существует в Grid

Для каждого node_id в adjacency_in:
  assert node_id существует в Grid
```

**Инвариант 2: Согласованность рёбер**

```
Для каждого edge_id в adjacency_out[node_id]:
  connection = ConnectionStore.Get(edge_id)
  assert connection существует
  assert connection.from_token_id = node_id
  assert connection.to_token_id существует в Graph

Для каждого edge_id в adjacency_in[node_id]:
  connection = ConnectionStore.Get(edge_id)
  assert connection существует
  assert connection.to_token_id = node_id
  assert connection.from_token_id существует в Graph
```

**Инвариант 3: Симметрия индексов**

```
Если edge_id в adjacency_out[from_id]:
  То edge_id должен быть в adjacency_in[to_id]
  где to_id = ConnectionStore.Get(edge_id).to_token_id

Обратно:
  Если edge_id в adjacency_in[to_id]:
  То edge_id должен быть в adjacency_out[from_id]
```

**Инвариант 4: Отсутствие дубликатов**

```
Для каждого adjacency_out[node_id]:
  assert len(list) = len(set(list)) // Нет дубликатов

Аналогично для adjacency_in
```

**Инвариант 5: Валидность путей**

```rust
Для Path { nodes, edges }:
  assert edges.len() = nodes.len() - 1
  
  for i in 0..edges.len():
    connection = ConnectionStore.Get(edges[i])
    assert connection.from_token_id == nodes[i]
    assert connection.to_token_id == nodes[i+1]
```

### 11.2 Валидация операций

**Валидация AddNode:**

```
Перед добавлением:
  1. assert node_id > 0
  2. assert node_id not in adjacency_out
  3. assert Grid.GetNode(node_id) существует

После добавления:
  assert node_id in adjacency_out
  assert adjacency_out[node_id].is_empty()
  assert adjacency_in[node_id].is_empty()
```

**Валидация AddEdge:**

```
Перед добавлением:
  1. connection = ConnectionStore.Get(edge_id)
  2. assert connection существует
  3. assert connection.from_token_id in adjacency_out
  4. assert connection.to_token_id in adjacency_out
  5. assert connection.from_token_id ≠ connection.to_token_id (если запрещены петли)

После добавления:
  assert edge_id in adjacency_out[from_id]
  assert edge_id in adjacency_in[to_id]
```

**Валидация FindPath:**

```
После нахождения пути:
  if path is not None:
    assert path.is_valid()
    assert path.nodes[0] = from_id
    assert path.nodes[last] = to_id
    assert все узлы существуют
    assert все рёбра существуют
```

### 11.3 Периодическая валидация

**Integrity check:**

```rust
ValidateGraphIntegrity():
  1. Проверить все инварианты
  
  2. Проверить симметрию индексов:
     for node_id in adjacency_out:
       for edge_id in adjacency_out[node_id]:
         connection = ConnectionStore.Get(edge_id)
         to_id = connection.to_token_id
         assert edge_id in adjacency_in[to_id]
  
  3. Проверить отсутствие "потерянных" рёбер:
     all_edges_out = flatten(adjacency_out.values())
     all_edges_in = flatten(adjacency_in.values())
     assert all_edges_out.sort() == all_edges_in.sort()
  
  4. Проверить согласованность с Grid и ConnectionStore
  
  5. Логировать любые несоответствия
```

**Рекомендуется запускать:**
- После больших batch операций
- Периодически (например, каждые 100,000 операций)
- При подозрении на несогласованность

**Самовосстановление:**

```
При обнаружении нарушения:
  1. Логировать детальную информацию
  2. Попытаться исправить:
     - Удалить невалидные рёбра
     - Восстановить симметрию индексов
     - Пересинхронизировать с Grid/ConnectionStore
  3. Если не удалось:
     - Пометить Graph как "degraded"
     - Предложить полную перестройку индексов
```

### 11.4 Тестирование

**Unit тесты:**

```
Тестировать каждую операцию:
  - AddNode/RemoveNode
  - AddEdge/RemoveEdge
  - GetNeighbors с различными фильтрами
  - FindPath с различными конфигурациями
  - BFS/DFS обход
  - Анализ топологии
```

**Integration тесты:**

```
Тестировать взаимодействие:
  - Graph + Grid: комбинированные запросы
  - Graph + Connection: типизация и веса
  - Синхронизация при изменениях
```

**Stress тесты:**

```
Проверить производительность:
  - 100k узлов, 1M рёбер
  - Поиск путей в плотном графе
  - Множественные подграфы
  - Конкурентный доступ
```

**Property-based тесты:**

```
Проверить инварианты:
  - После случайных операций инварианты сохраняются
  - Симметрия индексов не нарушается
  - Пути всегда валидны
```

## 12. Сериализация

### 12.1 Формат файла Graph

**Структура файла:**

```
╔═══════════════════════════════════════════════════════════╗
║ GRAPH HEADER (128 bytes)                                  ║
╠═══════════════════════════════════════════════════════════╣
║ NODE INDEX (variable)                                     ║
║ ┌─────────────────────────────────────────────────────┐ ║
║ │ NodeId #1 (4 bytes)                                 │ ║
║ ├─────────────────────────────────────────────────────┤ ║
║ │ NodeId #2 (4 bytes)                                 │ ║
║ ├─────────────────────────────────────────────────────┤ ║
║ │ ...                                                 │ ║
║ └─────────────────────────────────────────────────────┘ ║
╠═══════════════════════════════════════════════════════════╣
║ ADJACENCY DATA (variable)                                 ║
║ ┌─────────────────────────────────────────────────────┐ ║
║ │ NodeId → OutEdges mapping                           │ ║
║ │ NodeId → InEdges mapping                            │ ║
║ └─────────────────────────────────────────────────────┘ ║
╠═══════════════════════════════════════════════════════════╣
║ CACHED METRICS (optional)                                 ║
╠═══════════════════════════════════════════════════════════╣
║ GRAPH FOOTER (64 bytes)                                   ║
╚═══════════════════════════════════════════════════════════╝
```

### 12.2 Graph Header

**Структура (128 bytes):**

| Offset | Size | Type | Field | Description |
|--------|------|------|-------|-------------|
| 0 | 8 | char[8] | magic | "NGGRAPH2" (ASCII) |
| 8 | 2 | uint16 | version_major | Major version (2) |
| 10 | 2 | uint16 | version_minor | Minor version (0) |
| 12 | 2 | uint16 | version_patch | Patch version (0) |
| 14 | 2 | uint16 | endianness | 0x0102 = little-endian |
| 16 | 8 | uint64 | node_count | Total number of nodes |
| 24 | 8 | uint64 | edge_count | Total number of edges |
| 32 | 4 | uint32 | created_timestamp | File creation time |
| 36 | 4 | uint32 | modified_timestamp | Last modification time |
| 40 | 4 | uint32 | checksum_type | 0=none, 1=CRC32, 2=SHA256 |
| 44 | 4 | uint32 | checksum_value | Checksum (if type=1) |
| 48 | 8 | uint64 | grid_version_compat | Compatible Grid version |
| 56 | 8 | uint64 | connection_version_compat | Compatible Connection version |
| 64 | 4 | uint32 | graph_flags | Graph-level flags |
| 68 | 4 | uint32 | index_format | Format of adjacency data |
| 72 | 8 | uint64 | metrics_offset | Offset to cached metrics (0=none) |
| 80 | 8 | uint64 | metrics_size | Size of metrics section |
| 88 | 40 bytes | | reserved | Reserved for future use |
| **TOTAL** | **128 bytes** | | | |

**Graph flags:**

```
Bit 0: DIRECTED - Граф направленный
Bit 1: WEIGHTED - Рёбра взвешенные
Bit 2: COMPRESSED - Data is compressed
Bit 3: INCLUDE_METRICS - Cached metrics included
Bit 4-31: Reserved
```

**Index format:**

```
0x01: ADJACENCY_LIST - Списки смежности (по умолчанию)
0x02: EDGE_LIST - Список рёбер
0x03: ADJACENCY_MATRIX - Матрица смежности (для плотных графов)
```

### 12.3 Сериализация adjacency lists

**Формат:**

```
Для каждого узла:
  NodeId: uint32 (4 bytes)
  OutDegree: uint32 (4 bytes)
  OutEdges: [EdgeId × OutDegree] // EdgeId = uint64 hash
  InDegree: uint32 (4 bytes)
  InEdges: [EdgeId × InDegree]
```

**Оптимизация для разреженности:**

```
Если EdgeId = hash (uint64):
  EdgeId size = 8 bytes per edge

Сохранять только узлы с degree > 0
Пропускать пустые adjacency lists
```

### 12.4 Операции сериализации

**Save(filepath: string, options: SaveOptions):**

```rust
SaveOptions {
    include_metrics: bool,
    compression: CompressionType,
    index_format: IndexFormat,
}
```

**Алгоритм:**
```
1. Создать header
2. Записать header (128 bytes)
3. Записать индекс узлов (NodeIds)
4. Записать adjacency data в выбранном формате
5. Если include_metrics:
     Записать кэшированные метрики
6. Записать footer с checksums
7. Flush и закрыть файл

Сложность: O(V + E)
```

**Load(filepath: string) -> Graph:**

**Алгоритм:**
```
1. Открыть и валидировать header
2. Создать новый Graph
3. Загрузить индекс узлов
4. Для каждого узла:
     Graph.AddNode(node_id)
5. Загрузить adjacency data
6. Для каждого ребра:
     Graph.AddEdge(edge_id, from_id, to_id)
7. Если include_metrics:
     Загрузить кэшированные метрики
8. Валидировать footer
9. return Graph

Сложность: O(V + E)
```

### 12.5 Incremental updates

**Для больших графов:**

**Стратегия:** Append-only log

```
Файлы:
  graph_base.bin - Полный snapshot
  graph_changes_001.log - Incremental changes
  graph_changes_002.log
  ...

Change log format:
  [OpType (1 byte)][Data (variable)]

OpTypes:
  0x01: ADD_NODE [NodeId: 4 bytes]
  0x02: REMOVE_NODE [NodeId: 4 bytes]
  0x03: ADD_EDGE [EdgeId: 8 bytes][FromId: 4 bytes][ToId: 4 bytes]
  0x04: REMOVE_EDGE [EdgeId: 8 bytes]

Загрузка:
  1. Load base snapshot
  2. Apply all change logs in order
  3. Consolidate into new base периодически
```

---

## Приложения

### A. Константы и параметры

```rust
// Размерности
const MAX_PATH_LENGTH: usize = 1000;         // Максимальная длина пути для поиска
const DEFAULT_MAX_DEPTH: usize = 10;          // Глубина обхода по умолчанию

// Пороги
const HUB_DEGREE_THRESHOLD: usize = 100;      // Порог для определения хаба
const DENSE_SUBGRAPH_MIN_DENSITY: f32 = 0.6;  // Минимальная плотность для dense subgraph
const MIN_CLUSTERING_COEFFICIENT: f32 = 0.3;

// Производительность
const BATCH_SIZE_EDGES: usize = 1000;         // Размер батча для добавления рёбер
const CACHE_TTL_SECONDS: u32 = 300;           // TTL для кэшированных метрик

// Оптимизации
const ADJACENCY_VEC_TO_SET_THRESHOLD: usize = 100; // Переключение Vec→Set
const PARALLEL_BFS_MIN_NODES: usize = 10000;  // Минимальный размер для параллельного BFS
const SUBGRAPH_CACHE_MAX_SIZE: usize = 100;   // Максимум кэшированных подграфов

// Алгоритмы
const DIJKSTRA_HEAP_INITIAL_CAPACITY: usize = 256;
const A_STAR_OPEN_SET_INITIAL_CAPACITY: usize = 128;
const BFS_QUEUE_INITIAL_CAPACITY: usize = 256;
const DFS_STACK_INITIAL_CAPACITY: usize = 128;

// Лимиты
const MAX_NODES: usize = 1_000_000;           // 1 миллион узлов
const MAX_EDGES: usize = 10_000_000;          // 10 миллионов рёбер
const MAX_SUBGRAPHS_CACHE: usize = 100;

// Валидация
const INTEGRITY_CHECK_INTERVAL_OPS: usize = 100_000; // Проверка после операций
const MAX_RECONSTRUCTION_ATTEMPTS: u8 = 3;    // Попыток восстановления

// Сериализация
const GRAPH_MAGIC: &[u8; 8] = b"NGGRAPH2";
const GRAPH_VERSION_MAJOR: u16 = 2;
const GRAPH_VERSION_MINOR: u16 = 0;
const GRAPH_VERSION_PATCH: u16 = 0;
const HEADER_SIZE: usize = 128;
const FOOTER_SIZE: usize = 64;
```

### B. Типы и enum

```rust
// Направление обхода
enum Direction {
    OUTGOING,  // 0: исходящие рёбра
    INCOMING,  // 1: входящие рёбра
    BOTH,      // 2: оба направления
}

// Тип центральности
enum CentralityType {
    DEGREE,       // 0: степень узла
    BETWEENNESS,  // 1: betweenness centrality
    CLOSENESS,    // 2: closeness centrality
    EIGENVECTOR,  // 3: eigenvector centrality (будущее)
}

// Формат сериализации
enum IndexFormat {
    ADJACENCY_LIST = 0x01,   // Списки смежности
    EDGE_LIST = 0x02,        // Список рёбер
    ADJACENCY_MATRIX = 0x03, // Матрица смежности
}

// Тип компрессии
enum CompressionType {
    NONE = 0,   // Без компрессии
    LZ4 = 1,    // LZ4 (быстро)
    ZSTD = 2,   // Zstandard (высокая степень сжатия)
}

// Тип checksum
enum ChecksumType {
    NONE = 0,    // Без проверки
    CRC32 = 1,   // CRC32
    SHA256 = 2,  // SHA-256
}

// Стратегия синхронизации
enum SyncStrategy {
    PUSH,  // Event-driven
    PULL,  // Периодический опрос
    LAZY,  // По запросу
}

// Статус графа
enum GraphStatus {
    HEALTHY,    // Все инварианты соблюдены
    DEGRADED,   // Обнаружены нарушения, но система работает
    CORRUPTED,  // Требуется восстановление
}
```

### C. Интерфейсы (псевдокод)

**Основной интерфейс Graph:**

```rust
trait GraphOperations {
    // Базовые операции с узлами
    fn add_node(&mut self, node_id: NodeId) -> bool;
    fn remove_node(&mut self, node_id: NodeId) -> bool;
    fn contains_node(&self, node_id: NodeId) -> bool;
    fn get_node_count(&self) -> usize;
    
    // Базовые операции с рёбрами
    fn add_edge(&mut self, edge_id: EdgeId, from_id: NodeId, to_id: NodeId) -> bool;
    fn remove_edge(&mut self, edge_id: EdgeId) -> bool;
    fn contains_edge(&self, from_id: NodeId, to_id: NodeId) -> bool;
    fn get_edge_count(&self) -> usize;
    
    // Степени
    fn get_degree(&self, node_id: NodeId, direction: Direction) -> usize;
    fn get_neighbors(&self, node_id: NodeId, direction: Direction, 
                     filter: Option<EdgeFilter>) -> Vec<NodeId>;
    
    // Обход
    fn bfs(&self, start_id: NodeId, max_depth: Option<usize>, 
           visitor: impl Fn(NodeId, usize));
    fn dfs(&self, start_id: NodeId, max_depth: Option<usize>, 
           visitor: impl Fn(NodeId, usize));
    
    // Поиск путей
    fn find_path(&self, from_id: NodeId, to_id: NodeId, 
                 max_depth: Option<usize>) -> Option<Path>;
    fn find_shortest_path(&self, from_id: NodeId, to_id: NodeId, 
                          cost_fn: impl Fn(&Connection) -> f32) -> Option<Path>;
    fn find_path_a_star(&self, from_id: NodeId, to_id: NodeId, 
                        heuristic: impl Fn(NodeId, NodeId) -> f32) -> Option<Path>;
}

trait GraphAnalysis {
    // Топология
    fn get_connected_component(&self, node_id: NodeId) -> Set<NodeId>;
    fn get_all_connected_components(&self) -> Vec<Set<NodeId>>;
    fn is_connected(&self, node_a: NodeId, node_b: NodeId) -> bool;
    
    // Метрики
    fn calculate_graph_metrics(&self) -> GraphMetrics;
    fn calculate_degree_centrality(&self, node_id: NodeId) -> f32;
    fn calculate_betweenness_centrality(&self, node_id: NodeId) -> f32;
    fn calculate_closeness_centrality(&self, node_id: NodeId) -> f32;
    fn calculate_clustering_coefficient(&self, node_id: NodeId) -> f32;
    
    // Структура
    fn contains_cycle(&self) -> bool;
    fn find_cycle(&self) -> Option<Path>;
    fn topological_sort(&self) -> Option<Vec<NodeId>>;
    fn get_hub_nodes(&self, top_k: usize, direction: Direction) -> Vec<(NodeId, usize)>;
}

trait SubgraphOperations {
    // Извлечение
    fn extract_subgraph(&self, node_ids: Set<NodeId>) -> Subgraph;
    fn extract_neighborhood(&self, center_id: NodeId, radius: usize) -> Subgraph;
    
    // Анализ
    fn calculate_subgraph_metrics(&self, subgraph: &Subgraph) -> SubgraphMetrics;
    
    // Операции
    fn merge_subgraphs(&self, subgraph1: &Subgraph, subgraph2: &Subgraph) -> Subgraph;
    fn intersect_subgraphs(&self, subgraph1: &Subgraph, subgraph2: &Subgraph) -> Subgraph;
    fn is_subgraph_of(&self, smaller: &Subgraph, larger: &Subgraph) -> bool;
    
    // Поиск
    fn find_dense_subgraphs(&self, min_density: f32, min_size: usize) -> Vec<Subgraph>;
    fn find_k_core(&self, k: usize) -> Subgraph;
}

trait GraphSerialization {
    // Сохранение/загрузка
    fn save(&self, filepath: &str, options: SaveOptions) -> Result<(), Error>;
    fn load(filepath: &str) -> Result<Self, Error>;
    
    // Экспорт
    fn export_for_visualization(&self, format: VisualizationFormat) -> String;
    fn to_adjacency_matrix(&self) -> Vec<Vec<f32>>;
    fn to_edge_list(&self) -> Vec<(NodeId, NodeId, f32)>;
}

trait GraphValidation {
    // Проверка инвариантов
    fn validate_integrity(&self) -> ValidationReport;
    fn repair(&mut self) -> RepairReport;
    fn get_status(&self) -> GraphStatus;
}
```

### D. Взаимодействие с другими модулями

**Последовательность операций для создания связанных Token:**

```rust
// 1. Создание Token A и Token B:
Token_A = Token::new(id=42, coordinates=[...])
Token_B = Token::new(id=100, coordinates=[...])

// 2. Добавление в Grid:
Grid::insert_node(Token_A)
Grid::insert_node(Token_B)

// 3. Создание Connection:
Connection_AB = Connection::new(
    connection_id="conn_42_100",
    from_token_id=42,
    to_token_id=100,
    connection_type=CAUSALITY,
    weight=0.8
)

// 4. Добавление Connection в хранилище:
ConnectionStore::add(Connection_AB)

// 5. Регистрация в Graph:
Graph::add_node(42)
Graph::add_node(100)
Graph::add_edge(hash("conn_42_100"), 42, 100)

// Теперь:
// - Token A и B существуют в пространстве (Grid)
// - Они топологически связаны (Graph)
// - Связь типизирована и имеет силу (Connection)
```

**Запрос "найти путь между концепциями":**

```rust
// Цель: Найти путь от Token(id=42, concept="cat") до Token(id=1000, concept="animal")

// 1. Топологический поиск:
path = Graph::find_path(42, 1000)
// → Path { nodes: [42, 100, 500, 1000], edges: [...], length: 3 }

// 2. Обогащение данными:
for node_id in path.nodes:
    token = Grid::get_node(node_id)
    print(f"{token.id}: {decode_fsc(token.fsc_code)}")

for edge_id in path.edges:
    connection = ConnectionStore::get(edge_id)
    print(f" →[{connection.type}, weight={connection.weight}]")

// 3. Результат:
// 42 (cat) →[HIERARCHY, 0.9] 100 (mammal) →[HIERARCHY, 0.85] 500 (vertebrate) →[HIERARCHY, 0.95] 1000 (animal)
```

**Комбинированный запрос (пространство + топология):**

```rust
// Запрос: "Найти токены, связанные с токеном 42, и находящиеся в радиусе 10.0 от него"

// 1. Пространственный поиск:
spatial_neighbors = Grid::find_neighbors(center_id=42, level=L8_ABSTRACT, radius=10.0)
// → [50, 51, 52, ..., 99]

// 2. Топологический поиск:
topological_neighbors = Graph::get_nodes_within_distance(start_id=42, max_distance=2)
// → [(42, 0), (100, 1), (101, 1), (500, 2), ...]

// 3. Пересечение:
result = spatial_neighbors.intersection(topological_neighbors)
// → Токены, которые близки И связаны
```

### E. Примеры использования

**Пример 1: Построение графа знаний**

```rust
// Задача: Построить граф с концептами и их отношениями

// Создание концептов
concepts = [
    (1, "animal"),
    (2, "mammal"),
    (3, "cat"),
    (4, "dog"),
    (5, "vertebrate"),
    (6, "fur")
]

// Добавление узлов
for (id, name) in concepts:
    token = Token::new(id, name, coordinates=random())
    Grid::insert_node(token)
    Graph::add_node(id)

// Создание связей
relations = [
    (2, 1, HIERARCHY),     // mammal IS-A animal
    (3, 2, HIERARCHY),     // cat IS-A mammal
    (4, 2, HIERARCHY),     // dog IS-A mammal
    (2, 5, HIERARCHY),     // mammal IS-A vertebrate
    (3, 6, ASSOCIATION),   // cat HAS fur
    (4, 6, ASSOCIATION),   // dog HAS fur
]

for (from, to, type) in relations:
    connection = Connection::new(from, to, type, weight=0.9)
    ConnectionStore::add(connection)
    Graph::add_edge(connection.id, from, to)

// Теперь можно делать запросы:
path = Graph::find_path(3, 1)  // cat → mammal → animal
neighbors = Graph::get_neighbors(2, INCOMING)  // что является mammal?
```

**Пример 2: Анализ сети влияния**

```rust
// Задача: Найти наиболее влиятельные узлы в социальной сети

// Граф уже построен (узлы = люди, рёбра = отношения)

// 1. Найти хабы (много связей)
hubs = Graph::get_hub_nodes(top_k=10, direction=BOTH)
// → [(user_42, degree=1500), (user_100, degree=1200), ...]

// 2. Вычислить betweenness centrality (мосты в сети)
for node_id in all_nodes:
    centrality = Graph::calculate_betweenness_centrality(node_id)
    if centrality > 0.1:
        print(f"Bridge node: {node_id}, centrality: {centrality}")

// 3. Найти плотные сообщества
communities = Graph::find_dense_subgraphs(min_density=0.7, min_size=5)
for community in communities:
    metrics = Graph::calculate_subgraph_metrics(community)
    print(f"Community size: {metrics.node_count}, density: {metrics.density}")
```

**Пример 3: Навигация по семантическим связям**

```rust
// Задача: Найти путь от одного концепта к другому через семантические связи

start_concept = "quantum mechanics"
end_concept = "computer science"

// Найти токены с этими концептами (через FSC в будущем)
start_id = find_token_by_concept(start_concept)
end_id = find_token_by_concept(end_concept)

// Найти путь, используя вес связей
path = Graph::find_shortest_path(
    start_id,
    end_id,
    cost_fn = |connection| 1.0 / connection.weight  // Меньше вес = больше стоимость
)

if path is not None:
    print(f"Semantic path from '{start_concept}' to '{end_concept}':")
    for i in 0..path.nodes.len():
        token = Grid::get_node(path.nodes[i])
        print(f"  {i+1}. {token.concept}")
        if i < path.edges.len():
            connection = ConnectionStore::get(path.edges[i])
            print(f"    ↓ [{connection.type}, strength={connection.weight}]")
else:
    print("No semantic path found")
```

**Пример 4: Обход окрестности**

```rust
// Задача: Исследовать окрестность токена в радиусе 3 шагов, только через CAUSALITY связи

center_id = 42

// Обход с фильтром
filter = EdgeFilter {
    connection_types: Some([CAUSALITY]),
    min_weight: Some(0.5),
    ...
}

visited_nodes = []
Graph::bfs(
    start_id=center_id,
    max_depth=3,
    visitor = |node_id, depth| {
        token = Grid::get_node(node_id)
        visited_nodes.append((node_id, depth))
        print(f"Depth {depth}: {token.concept}")
    }
)

// Результат: список всех причинно связанных концептов в радиусе 3
```

### F. Формулы и определения

**Density (плотность графа):**

Для направленного графа:
```
density = E / (V × (V - 1))
```
где E = количество рёбер, V = количество узлов

Для ненаправленного графа:
```
density = 2E / (V × (V - 1))
```

Диапазон: [0, 1]
- 0 = нет рёбер
- 1 = полный граф (все узлы связаны)

**Average degree (средняя степень):**

```
avg_degree = (Σ degree(v)) / V
```
где сумма по всем узлам v

Для направленного графа:
```
avg_out_degree = E / V
avg_in_degree = E / V
```

**Clustering coefficient (коэффициент кластеризации):**

Локальный (для узла v):
```
C(v) = (число рёбер между соседями v) / (k × (k-1) / 2)
```
где k = количество соседей v

Глобальный:
```
C = (Σ C(v)) / V
```
средний по всем узлам

Диапазон: [0, 1]
- 0 = соседи не связаны друг с другом
- 1 = соседи образуют клику

**Degree centrality (центральность по степени):**

```
C_degree(v) = degree(v) / (V - 1)
```
Нормализовано к [0, 1]

**Closeness centrality (центральность по близости):**

```
C_closeness(v) = (V - 1) / (Σ d(v, u))
```
где d(v, u) = кратчайшее расстояние от v до u  
сумма по всем достижимым узлам u

Интерпретация: насколько быстро информация может достичь других узлов из v

**Betweenness centrality (центральность по посредничеству):**

```
C_betweenness(v) = Σ (σ_st(v) / σ_st)
```
где:
- σ_st = количество кратчайших путей от s до t
- σ_st(v) = количество этих путей, проходящих через v
- сумма по всем парам (s, t)

Интерпретация: насколько часто узел лежит на кратчайших путях между другими узлами

**Diameter (диаметр графа):**

```
diameter = max { d(u, v) | для всех пар узлов (u, v) }
```
где d(u, v) = длина кратчайшего пути

Интерпретация: максимальное расстояние между любыми двумя узлами

**Average path length (средняя длина пути):**

```
L = (Σ d(u, v)) / (V × (V - 1))
```
сумма по всем парам (u, v), u ≠ v

Интерпретация: среднее количество шагов между двумя случайными узлами

### G. Отличия от v1.0

**Основные изменения:**

| Аспект | v1.0 (Python) | v2.0 (Rust/C++) |
|--------|---------------|-----------------|
| Хранение | Хранит Token и Connection | Только индексы (ссылки) |
| Генетика | Встроенные операторы | Вынесены в отдельный модуль |
| CDNA | Встроенная валидация | Опциональная интеграция |
| Experience | Встроенная запись | Опциональная интеграция |
| ID типы | Python int | uint32 (NodeId), uint64 (EdgeId) |
| Сериализация | Python pickle/JSON | Бинарный формат |
| Параллелизм | GIL ограничения | True parallelism |
| Память | ~300 bytes/edge | ~16-48 bytes/edge |

**Новые возможности v2.0:**

- Топологическая сортировка
- K-core decomposition
- Улучшенные алгоритмы центральности
- A* с пространственными эвристиками
- Incremental serialization
- Thread-safe операции
- Интеграция с Grid для комбинированных запросов

**Удалённые из v2.0 (вынесены в другие модули):**

- Генетические операторы (→ Evolution module)
- ADNA/CDNA валидация (→ DNA module)
- Experience Stream (→ Experience module)
- Fitness calculation (→ Evolution module)

### H. Roadmap и будущие возможности

**v2.1 (планируется):**

- Динамические графы (temporal graphs) с поддержкой версионирования
- Incremental algorithms для онлайн обновления метрик
- Distributed graph processing (partitioning)

**v2.2 (планируется):**

- GPU-accelerated алгоритмы для больших графов
- Advanced community detection (Louvain, Label Propagation)

**v3.0 (будущее):**

- Hypergraph support (гиперграфы, рёбра соединяют >2 узлов)
- Probabilistic graphs (вероятностные рёбра)
- Query language для graph patterns
- Graph neural networks integration
- Streaming graph algorithms
- Self-modifying topology
- Meta-learning для оптимизации структуры
- Quantum-inspired algorithms
- Cognitive graph operations

---

## Заключение

**Graph Specification v2.0** определяет модуль топологической навигации и анализа для NeuroGraph OS.

### Ключевые принципы:

✅ **Разделение ответственности** — Graph навигирует, Grid организует, Connection определяет  
✅ **Эффективность** — O(1) доступ к соседям, оптимизированные алгоритмы обхода  
✅ **Гибкость** — Поддержка различных типов анализа и метрик  
✅ **Интеграция** — Тесная связь с Token, Connection и Grid  
✅ **Масштабируемость** — До 1M узлов и 10M рёбер  
✅ **Валидность** — Строгие инварианты и проверки целостности  

Graph — это вычислительный слой, который превращает разрозненные Token и Connection в живую сеть отношений, по которой можно навигировать, анализировать и извлекать знания.

---

**Версия документа:** 2.0.0  
**Дата:** 2025-10-21  
**Статус:** Official Specification  
**Автор:** NeuroGraph OS Team  
**Лицензия:** MIT  

**Зависимости:**
- Token Specification v2.0
- Connection Specification v1.0
- Grid Specification v2.0

**Совместимость:** Rust, C++, системные языки с поддержкой HashMap и HashSet

---

🎯 **Graph v2.0 Specification — Complete**
