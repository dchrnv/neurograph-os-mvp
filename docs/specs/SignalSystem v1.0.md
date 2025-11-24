# Signal System v1.0 - Расширение Graph для нейронной активации

**Версия:** 1.0.0  
**Статус:** Спецификация для реализации  
**Дата:** 2025-01-13  
**Зависимости:** Graph v2.0, Token v2.0, Connection v1.0  
**Язык реализации:** Rust  
**Цель:** Добавить динамику spreading activation в статический граф

---

## 1. Философия и концепция

### 1.1 Проблематика

Graph v2.0 предоставляет отличную топологическую навигацию (BFS, DFS, pathfinding), но это **статическая** структура. Для когнитивной системы нужна **динамика** - способность распространять сигналы, моделировать активацию нейронов, создавать волны возбуждения.

### 1.2 Решение: Neural Dynamics Layer

Добавляем в Graph новый слой функциональности:
- **Activation state** - состояние активации для каждого токена
- **Spreading activation** - алгоритм распространения сигнала
- **Decay & accumulation** - затухание и накопление энергии
- **Thresholds** - пороговая логика срабатывания

### 1.3 Два типа сигналов (как в твоём наброске)

1. **System Signals** (события инфраструктуры)
   - Через ExperienceStream
   - TokenCreated, ConnectionAdded, PolicyUpdated
   - Для логирования и обучения

2. **Neural Signals** (когнитивная активность)
   - Внутри Graph, в памяти
   - Быстрое распространение без сериализации
   - Только результат записывается в ExperienceStream

---

## 2. Архитектура расширения

### 2.1 Новые структуры в Graph

```rust
// Добавляем в src/core_rust/src/graph.rs

/// Состояние активации узла
#[derive(Debug, Clone, Default)]
pub struct NodeActivation {
    /// Текущий уровень активации [0.0, 1.0]
    pub energy: f32,
    
    /// Время последней активации
    pub last_activated: u64,
    
    /// Количество активаций в текущем цикле
    pub activation_count: u32,
    
    /// Источник активации (для трассировки)
    pub source_id: Option<u32>,
}

/// Расширение Graph с нейронной динамикой
pub struct Graph {
    // ... существующие поля ...
    
    /// Состояния активации узлов
    activations: HashMap<NodeId, NodeActivation>,
    
    /// Конфигурация spreading activation
    signal_config: SignalConfig,
}

/// Конфигурация алгоритма spreading activation
#[derive(Debug, Clone)]
pub struct SignalConfig {
    /// Минимальная энергия для продолжения распространения
    pub min_energy: f32,  // Default: 0.01
    
    /// Коэффициент затухания на каждом шаге
    pub decay_rate: f32,  // Default: 0.2
    
    /// Максимальная глубина распространения
    pub max_depth: usize,  // Default: 5
    
    /// Порог активации узла
    pub activation_threshold: f32,  // Default: 0.1
    
    /// Режим накопления энергии
    pub accumulation_mode: AccumulationMode,
}

#[derive(Debug, Clone)]
pub enum AccumulationMode {
    /// Энергия суммируется
    Sum,
    
    /// Берётся максимум
    Max,
    
    /// Среднее взвешенное
    WeightedAverage,
}
```

### 2.2 Результат spreading activation

```rust
/// Результат распространения сигнала
#[derive(Debug, Clone)]
pub struct ActivationResult {
    /// Активированные узлы с их энергией
    pub activated_nodes: Vec<ActivatedNode>,
    
    /// Общее количество обработанных узлов
    pub nodes_visited: usize,
    
    /// Максимальная достигнутая глубина
    pub max_depth_reached: usize,
    
    /// Время выполнения (микросекунды)
    pub execution_time_us: u64,
    
    /// Путь максимальной активации
    pub strongest_path: Option<Path>,
}

#[derive(Debug, Clone)]
pub struct ActivatedNode {
    pub node_id: NodeId,
    pub energy: f32,
    pub depth: usize,
    pub path_from_source: Vec<NodeId>,
}
```

---

## 3. Алгоритмы spreading activation

### 3.1 Базовый алгоритм

```rust
impl Graph {
    /// Главная функция spreading activation
    pub fn spreading_activation(
        &mut self,
        source_id: NodeId,
        initial_energy: f32,
        custom_config: Option<SignalConfig>
    ) -> ActivationResult {
        let start_time = std::time::Instant::now();
        let config = custom_config.unwrap_or_else(|| self.signal_config.clone());
        
        // Очищаем предыдущие активации
        self.clear_activations();
        
        // Инициализация
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut result = ActivationResult::default();
        
        // Активируем источник
        self.activate_node(source_id, initial_energy, None);
        queue.push_back((source_id, initial_energy, 0_usize));
        visited.insert(source_id);
        
        // BFS с затуханием
        while let Some((current_id, current_energy, depth)) = queue.pop_front() {
            // Проверка глубины
            if depth >= config.max_depth {
                result.max_depth_reached = depth;
                break;
            }
            
            // Получаем соседей
            let neighbors = self.get_neighbors(current_id, Direction::Outgoing);
            
            for (neighbor_id, edge_id) in neighbors {
                // Пропускаем посещённые
                if visited.contains(&neighbor_id) {
                    continue;
                }
                
                // Получаем информацию о связи
                let edge = self.edge_map.get(&edge_id).unwrap();
                
                // Вычисляем энергию для соседа
                let transmitted_energy = self.compute_transmitted_energy(
                    current_energy,
                    edge.weight,
                    &config
                );
                
                // Проверяем порог
                if transmitted_energy < config.min_energy {
                    continue;
                }
                
                // Активируем соседа
                self.activate_node(neighbor_id, transmitted_energy, Some(current_id));
                
                // Добавляем в очередь
                queue.push_back((neighbor_id, transmitted_energy, depth + 1));
                visited.insert(neighbor_id);
                
                // Записываем в результат
                result.activated_nodes.push(ActivatedNode {
                    node_id: neighbor_id,
                    energy: transmitted_energy,
                    depth: depth + 1,
                    path_from_source: self.reconstruct_path(source_id, neighbor_id),
                });
            }
            
            result.nodes_visited += 1;
        }
        
        // Сортируем по энергии
        result.activated_nodes.sort_by(|a, b| 
            b.energy.partial_cmp(&a.energy).unwrap()
        );
        
        // Находим самый сильный путь
        if let Some(strongest) = result.activated_nodes.first() {
            result.strongest_path = Some(Path {
                nodes: strongest.path_from_source.clone(),
                edges: vec![], // TODO: заполнить
                total_cost: strongest.energy,
                length: strongest.depth,
            });
        }
        
        result.execution_time_us = start_time.elapsed().as_micros() as u64;
        result
    }
    
    /// Вычисление передаваемой энергии
    fn compute_transmitted_energy(
        &self,
        source_energy: f32,
        edge_weight: f32,
        config: &SignalConfig
    ) -> f32 {
        // Базовая формула: E_new = E_source * weight * (1 - decay)
        source_energy * edge_weight * (1.0 - config.decay_rate)
    }
    
    /// Активация узла
    fn activate_node(
        &mut self,
        node_id: NodeId,
        energy: f32,
        source: Option<NodeId>
    ) {
        let activation = self.activations.entry(node_id).or_default();
        
        // Режим накопления
        match self.signal_config.accumulation_mode {
            AccumulationMode::Sum => {
                activation.energy += energy;
            }
            AccumulationMode::Max => {
                activation.energy = activation.energy.max(energy);
            }
            AccumulationMode::WeightedAverage => {
                let count = activation.activation_count as f32;
                activation.energy = 
                    (activation.energy * count + energy) / (count + 1.0);
            }
        }
        
        activation.activation_count += 1;
        activation.last_activated = current_timestamp();
        activation.source_id = source;
    }
    
    /// Очистка активаций
    pub fn clear_activations(&mut self) {
        self.activations.clear();
    }
    
    /// Получение текущей активации узла
    pub fn get_activation(&self, node_id: NodeId) -> Option<f32> {
        self.activations.get(&node_id).map(|a| a.energy)
    }
}
```

### 3.2 Продвинутые алгоритмы

```rust
impl Graph {
    /// Spreading activation с учётом типов связей
    pub fn typed_spreading_activation(
        &mut self,
        source_id: NodeId,
        initial_energy: f32,
        allowed_types: &[ConnectionType]
    ) -> ActivationResult {
        // Фильтруем связи по типам
        // Используем только разрешённые типы для распространения
        // ...
    }
    
    /// Двунаправленное распространение (вперёд и назад)
    pub fn bidirectional_spreading(
        &mut self,
        source_id: NodeId,
        initial_energy: f32
    ) -> (ActivationResult, ActivationResult) {
        // Запускаем в обе стороны одновременно
        let forward = self.spreading_activation(source_id, initial_energy, None);
        
        // Меняем направление
        let mut backward_config = self.signal_config.clone();
        let backward = self.spreading_activation_internal(
            source_id, 
            initial_energy,
            Direction::Incoming,
            backward_config
        );
        
        (forward, backward)
    }
    
    /// Множественные источники активации
    pub fn multi_source_activation(
        &mut self,
        sources: &[(NodeId, f32)]
    ) -> ActivationResult {
        self.clear_activations();
        
        let mut combined_result = ActivationResult::default();
        
        // Активируем от каждого источника
        for &(source_id, energy) in sources {
            let partial = self.spreading_activation(source_id, energy, None);
            
            // Объединяем результаты
            for activated in partial.activated_nodes {
                // Проверяем, был ли узел уже активирован
                if let Some(existing) = combined_result.activated_nodes
                    .iter_mut()
                    .find(|n| n.node_id == activated.node_id) 
                {
                    // Применяем режим накопления
                    match self.signal_config.accumulation_mode {
                        AccumulationMode::Sum => existing.energy += activated.energy,
                        AccumulationMode::Max => existing.energy = existing.energy.max(activated.energy),
                        _ => {}
                    }
                } else {
                    combined_result.activated_nodes.push(activated);
                }
            }
        }
        
        combined_result
    }
    
    /// Волновое распространение с временной задержкой
    pub async fn wave_propagation(
        &mut self,
        source_id: NodeId,
        initial_energy: f32,
        wave_delay_ms: u64
    ) -> ActivationResult {
        // Распространение по слоям с задержкой
        let mut current_layer = vec![(source_id, initial_energy)];
        let mut result = ActivationResult::default();
        let mut depth = 0;
        
        while !current_layer.is_empty() && depth < self.signal_config.max_depth {
            let mut next_layer = Vec::new();
            
            // Обрабатываем текущий слой
            for (node_id, energy) in current_layer {
                let neighbors = self.get_neighbors(node_id, Direction::Outgoing);
                
                for (neighbor_id, edge_id) in neighbors {
                    let edge = self.edge_map.get(&edge_id).unwrap();
                    let transmitted = self.compute_transmitted_energy(
                        energy,
                        edge.weight,
                        &self.signal_config
                    );
                    
                    if transmitted >= self.signal_config.min_energy {
                        next_layer.push((neighbor_id, transmitted));
                        self.activate_node(neighbor_id, transmitted, Some(node_id));
                    }
                }
            }
            
            // Визуализация волны (опционально)
            if wave_delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(wave_delay_ms)).await;
            }
            
            current_layer = next_layer;
            depth += 1;
        }
        
        result
    }
}
```

---

## 4. Интеграция с ActionController

### 4.1 SignalExecutor реализация

```rust
use crate::graph::Graph;
use crate::action_controller::{ActionExecutor, ActionResult, ExecutionError};

/// Исполнитель для активации и распространения сигналов
pub struct SignalExecutor {
    graph: Arc<RwLock<Graph>>,
}

impl SignalExecutor {
    pub fn new(graph: Arc<RwLock<Graph>>) -> Self {
        Self { graph }
    }
}

#[async_trait]
impl ActionExecutor for SignalExecutor {
    fn id(&self) -> &str {
        "signal_executor"
    }
    
    fn supported_actions(&self) -> Vec<ActionType> {
        vec![
            ActionType::ActivateToken,
            ActionType::PropagateSignal,
        ]
    }
    
    async fn execute(&self, params: [f32; 8]) -> Result<ActionResult, ExecutionError> {
        // Распаковка параметров
        let source_token_id = params[0] as u32;
        let initial_energy = params[1].clamp(0.0, 1.0);
        let decay_rate = params[2].clamp(0.0, 1.0);
        let max_depth = params[3] as usize;
        
        // Создаём конфигурацию
        let config = SignalConfig {
            min_energy: 0.01,
            decay_rate,
            max_depth,
            activation_threshold: 0.1,
            accumulation_mode: AccumulationMode::Sum,
        };
        
        // Выполняем spreading activation
        let mut graph = self.graph.write().await;
        let activation_result = graph.spreading_activation(
            source_token_id,
            initial_energy,
            Some(config)
        );
        
        // Формируем результат
        Ok(ActionResult {
            success: true,
            output: json!({
                "source_token": source_token_id,
                "activated_count": activation_result.activated_nodes.len(),
                "max_energy": activation_result.activated_nodes
                    .first()
                    .map(|n| n.energy)
                    .unwrap_or(0.0),
                "execution_time_us": activation_result.execution_time_us,
                "strongest_path": activation_result.strongest_path
                    .map(|p| p.nodes),
            }),
            modified_tokens: activation_result.activated_nodes
                .iter()
                .map(|n| n.node_id)
                .collect(),
            modified_connections: vec![],
            execution_time_ns: activation_result.execution_time_us * 1000,
            error: None,
        })
    }
}
```

---

## 5. Оптимизации и производительность

### 5.1 Целевые метрики

```yaml
Производительность:
  Small graph (1K nodes, 10K edges):
    - Spreading (depth=3): < 100μs
    - Multi-source (10 sources): < 500μs
    
  Medium graph (10K nodes, 100K edges):
    - Spreading (depth=3): < 1ms
    - Multi-source (10 sources): < 5ms
    
  Large graph (100K nodes, 1M edges):
    - Spreading (depth=3): < 10ms
    - Multi-source (10 sources): < 50ms

Память:
  NodeActivation: 20 bytes per node
  10K nodes: ~200KB overhead
  100K nodes: ~2MB overhead
```

### 5.2 Оптимизации

1. **Lazy activation** - активации создаются только при необходимости
2. **Early termination** - прерывание при достижении минимальной энергии
3. **Priority queue** - обработка узлов по приоритету энергии
4. **Batch operations** - групповая обработка соседей
5. **Cache-friendly** - последовательный доступ к памяти

```rust
/// Оптимизированная версия с priority queue
pub fn spreading_activation_optimized(
    &mut self,
    source_id: NodeId,
    initial_energy: f32
) -> ActivationResult {
    use std::collections::BinaryHeap;
    
    #[derive(PartialEq)]
    struct QueueItem {
        node_id: NodeId,
        energy: f32,
        depth: usize,
    }
    
    impl Ord for QueueItem {
        fn cmp(&self, other: &Self) -> Ordering {
            // Больше энергии = выше приоритет
            self.energy.partial_cmp(&other.energy).unwrap().reverse()
        }
    }
    
    let mut queue = BinaryHeap::new();
    queue.push(QueueItem {
        node_id: source_id,
        energy: initial_energy,
        depth: 0,
    });
    
    // Обрабатываем узлы в порядке убывания энергии
    while let Some(item) = queue.pop() {
        // Энергичные узлы обрабатываются первыми
        // Это даёт более естественное распространение
        // ...
    }
}
```

---

## 6. Тестирование

### 6.1 Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_spreading() {
        let mut graph = create_test_graph();
        
        // Создаём простую цепочку: 1 -> 2 -> 3 -> 4
        graph.add_edge(1, 2, 1.0);
        graph.add_edge(2, 3, 0.8);
        graph.add_edge(3, 4, 0.6);
        
        let result = graph.spreading_activation(1, 1.0, None);
        
        // Проверяем затухание
        assert!(result.activated_nodes.len() >= 3);
        assert!(result.activated_nodes[0].energy < 1.0);
        assert!(result.activated_nodes[1].energy < result.activated_nodes[0].energy);
    }
    
    #[test]
    fn test_accumulation_modes() {
        let mut graph = create_test_graph();
        
        // Создаём ромб: 1 -> 2,3 -> 4
        graph.add_edge(1, 2, 0.5);
        graph.add_edge(1, 3, 0.5);
        graph.add_edge(2, 4, 1.0);
        graph.add_edge(3, 4, 1.0);
        
        // Тест режима Sum
        graph.signal_config.accumulation_mode = AccumulationMode::Sum;
        let result = graph.spreading_activation(1, 1.0, None);
        
        // Узел 4 должен получить энергию от обоих путей
        let node4_energy = graph.get_activation(4).unwrap();
        assert!(node4_energy > 0.5); // Больше, чем от одного пути
    }
    
    #[test]
    fn test_max_depth_limit() {
        let mut graph = create_long_chain(10); // Цепь из 10 узлов
        
        let mut config = SignalConfig::default();
        config.max_depth = 3;
        
        let result = graph.spreading_activation(1, 1.0, Some(config));
        
        // Должны активироваться только 3 узла после источника
        assert_eq!(result.max_depth_reached, 3);
        assert!(result.activated_nodes.len() <= 3);
    }
}
```

### 6.2 Интеграционный тест

```rust
#[tokio::test]
async fn test_signal_executor_integration() {
    // Создаём полный стек
    let graph = Arc::new(RwLock::new(Graph::new()));
    let executor = SignalExecutor::new(graph.clone());
    
    // Создаём граф знаний
    {
        let mut g = graph.write().await;
        g.add_edge(1, 2, 0.9); // "огонь" -> "жар"
        g.add_edge(1, 3, 0.8); // "огонь" -> "опасность"
        g.add_edge(2, 4, 0.7); // "жар" -> "боль"
    }
    
    // Выполняем активацию
    let params = [
        1.0,  // source: "огонь"
        1.0,  // initial energy
        0.2,  // decay rate
        3.0,  // max depth
        0.0, 0.0, 0.0, 0.0  // unused
    ];
    
    let result = executor.execute(params).await.unwrap();
    
    assert!(result.success);
    assert_eq!(result.modified_tokens.len(), 3); // жар, опасность, боль
}
```

---

## 7. Конфигурация

```toml
# signal_system.toml

[spreading]
min_energy = 0.01           # Минимальная энергия для продолжения
decay_rate = 0.2            # Затухание на каждом шаге
max_depth = 5               # Максимальная глубина
activation_threshold = 0.1   # Порог активации узла

[accumulation]
mode = "sum"                # sum | max | weighted_average

[performance]
use_priority_queue = true   # Оптимизированный алгоритм
parallel_neighbors = false  # Параллельная обработка соседей
cache_paths = true         # Кэшировать пути активации

[debug]
trace_activation = false    # Логировать каждую активацию
measure_timings = true     # Замерять время выполнения
```

---

## 8. Выводы

### Что даёт Signal System:

1. **Динамика** - граф становится "живым", может моделировать мышление
2. **Ассоциации** - автоматическая активация связанных концептов
3. **Контекст** - волны активации создают семантический контекст
4. **Обучение** - паттерны активации можно использовать для обучения

### Интеграция с NeuroGraph OS:

- **IntuitionModule** использует паттерны активации для создания рефлексов
- **ADNA** анализирует пути максимальной активации
- **ExperienceStream** записывает только результаты, не детали
- **ActionController** управляет запуском активации

### Следующие шаги:

1. Реализовать базовый spreading activation в Graph
2. Добавить SignalExecutor в ActionController
3. Протестировать на простых сценариях
4. Оптимизировать для больших графов

---

**Signal System v1.0 - Neural Dynamics for NeuroGraph OS**  
*"From static structure to living thought"*
