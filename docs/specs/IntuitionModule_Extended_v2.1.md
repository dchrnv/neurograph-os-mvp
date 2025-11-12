# Модуль Intuition v2.1 — Расширенная спецификация

**Версия:** 2.0.0
**Дата:** 2025-11-11
**Статус:** Расширенная спецификация для интеграции
**Зависимости:** ExperienceStream v2.0, ADNA v3.0, Token v2.0, Connection v1.0
**Язык:** Rust + tokio async runtime

---

## 1. Общая архитектура модуля

### 1.1 Позиционирование в системе

Модуль Intuition — это промежуточный слой между событийной системой и обучением:

```
События (горячий путь) → Интуиция → Обучение (холодный путь)
```

### 1.2 Основные компоненты

```rust
pub struct IntuitionModule {
    // Горячий путь - обработка в реальном времени
    event_processor: EventProcessor,       // Прием и первичная обработка
    appraisers: AppraiserSet,             // 4 оценщика
    hot_buffer: HotBuffer,                 // Кольцевой буфер событий
  
    // Теплый путь - асинхронный анализ
    pattern_analyzer: PatternAnalyzer,     // Поиск паттернов
    correlation_engine: CorrelationEngine, // Корреляция событий
  
    // Холодный путь - архивация и обучение  
    experience_archive: ExperienceArchive, // Долгосрочное хранение
    token_compressor: TokenCompressor,     // Event → ExperienceToken
  
    // Конфигурация и состояние
    adna_context: Arc<RwLock<ADNAv3>>,    // Текущая конфигурация
    metrics: IntuitionMetrics,             // Метрики производительности
}
```

---

## 2. Поток данных и обработка событий

### 2.1 Трехуровневая обработка

```
┌─────────────────────────────────────────────────────┐
│                   ГОРЯЧИЙ ПУТЬ                       │
│   ExperienceEvent (128 байт из ExperienceStream)     │
│                       ↓                              │
│   EventProcessor (валидация, enrichment)             │
│                       ↓                              │
│   Broadcast → 4 Appraisers (параллельно)            │
│                       ↓                              │
│   reward accumulation → обновленный Event           │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│                   ТЕПЛЫЙ ПУТЬ                        │
│   HotBuffer (кольцевой буфер, 100K событий)         │
│                       ↓                              │
│   PatternAnalyzer (batch processing каждые 10 сек)  │
│                       ↓                              │
│   Identified Patterns + Correlations                 │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│                  ХОЛОДНЫЙ ПУТЬ                       │
│   TokenCompressor (Event → ExperienceToken 128B)     │
│                       ↓                              │
│   ExperienceArchive (долгосрочное хранение)         │
│                       ↓                              │
│   Replay Buffer для обучения                         │
└─────────────────────────────────────────────────────┘
```

### 2.2 EventProcessor — точка входа

```rust
impl EventProcessor {
    /// Принимает событие из ExperienceStream
    pub async fn process(&mut self, event: ExperienceEvent) -> ProcessedEvent {
        // 1. Обогащение контекстом
        let enriched = self.enrich_with_context(event).await;
      
        // 2. Проверка валидности через ADNA
        if !self.validate_against_adna(&enriched) {
            return ProcessedEvent::rejected(event.event_id);
        }
      
        // 3. Broadcast всем appraisers
        let rewards = self.broadcast_to_appraisers(&enriched).await;
      
        // 4. Аккумуляция rewards
        let mut processed = ProcessedEvent::from(enriched);
        processed.total_reward = rewards.iter().sum();
        processed.reward_components = rewards;
      
        // 5. Запись в hot buffer
        self.hot_buffer.push(processed.clone());
      
        processed
    }
}
```

---

## 3. Appraisers — параллельная оценка

### 3.1 Архитектура AppraiserSet

```rust
pub struct AppraiserSet {
    homeostasis: HomeostasisAppraiser,
    goal_directed: GoalDirectedAppraiser,  
    curiosity: CuriosityAppraiser,
    efficiency: EfficiencyAppraiser,
  
    // Веса из ADNA v3
    weights: AppraiserWeights,
}

impl AppraiserSet {
    /// Параллельная оценка всеми appraisers
    pub async fn evaluate_all(&self, event: &EnrichedEvent) -> Vec<RewardComponent> {
        let futures = vec![
            self.homeostasis.evaluate(event),
            self.goal_directed.evaluate(event),
            self.curiosity.evaluate(event),
            self.efficiency.evaluate(event),
        ];
      
        // Параллельное выполнение
        let results = futures::future::join_all(futures).await;
      
        // Применение весов из ADNA
        results.into_iter()
            .zip(&[
                self.weights.homeostasis,
                self.weights.goal_directed,
                self.weights.curiosity,
                self.weights.efficiency,
            ])
            .map(|(reward, weight)| reward * weight)
            .collect()
    }
}
```

### 3.2 Enriched Event — обогащенное событие

```rust
/// Событие с дополнительным контекстом для appraisers
pub struct EnrichedEvent {
    // Базовое событие из ExperienceStream (128 байт)
    pub base: ExperienceEvent,

    // Sequence number для обновления rewards
    pub sequence_number: u64,

    // Контекст из Token/Connection/Grid
    pub token_context: Option<TokenContext>,
    pub connection_context: Option<ConnectionContext>,
    pub spatial_context: Option<SpatialContext>,

    // История для GoalDirectedAppraiser
    pub causal_chain: Vec<Uuid>,          // caused_by chain
    pub trajectory_id: Option<u64>,        // текущая траектория

    // Метрики для EfficiencyAppraiser
    pub compute_time_μs: u64,
    pub memory_delta_bytes: i64,

    // Новизна для CuriosityAppraiser
    pub novelty_score: f32,                // 0.0 = полностью известно, 1.0 = абсолютно ново
    pub uncertainty: f32,                   // неопределенность предсказания
}
```

---

## 4. HotBuffer — кольцевой буфер событий

### 4.1 Структура

```rust
pub struct HotBuffer {
    // Кольцевой буфер фиксированного размера
    buffer: Vec<ProcessedEvent>,
    capacity: usize,                       // default: 100_000
    head: usize,                          // указатель записи
    tail: usize,                          // указатель чтения
  
    // Индексы для быстрого поиска
    by_episode: HashMap<u64, Vec<usize>>, // episode_id → позиции
    by_trajectory: HashMap<u64, Vec<usize>>, // trajectory_id → позиции
  
    // Статистика
    total_written: u64,
    total_overwritten: u64,
}

impl HotBuffer {
    /// Добавить событие (O(1))
    pub fn push(&mut self, event: ProcessedEvent) {
        let pos = self.head % self.capacity;
      
        // Если буфер заполнен, удаляем старое из индексов
        if self.buffer.len() == self.capacity {
            self.remove_from_indices(pos);
            self.total_overwritten += 1;
        }
      
        // Записываем новое
        self.buffer[pos] = event;
        self.add_to_indices(pos, &event);
      
        self.head += 1;
        self.total_written += 1;
    }
  
    /// Получить последние N событий
    pub fn recent(&self, n: usize) -> Vec<ProcessedEvent> {
        // Эффективная выборка из кольцевого буфера
        let start = self.head.saturating_sub(n);
        (start..self.head)
            .map(|i| self.buffer[i % self.capacity].clone())
            .collect()
    }
  
    /// Найти события по episode_id
    pub fn by_episode(&self, episode_id: u64) -> Vec<ProcessedEvent> {
        self.by_episode
            .get(&episode_id)
            .map(|indices| {
                indices.iter()
                    .map(|&i| self.buffer[i].clone())
                    .collect()
            })
            .unwrap_or_default()
    }
}
```

---

## 5. PatternAnalyzer — поиск паттернов

### 5.1 Алгоритм анализа

```rust
pub struct PatternAnalyzer {
    // Параметры кластеризации
    min_cluster_size: usize,               // мин. размер кластера
    similarity_threshold: f32,             // порог схожести состояний
  
    // Кеш найденных паттернов
    pattern_cache: LruCache<u64, Pattern>,
  
    // Метрики
    patterns_found: u64,
    analysis_cycles: u64,
}

impl PatternAnalyzer {
    /// Анализ батча событий
    pub async fn analyze_batch(&mut self, events: &[ProcessedEvent]) -> Vec<Pattern> {
        // 1. Кластеризация состояний
        let state_clusters = self.cluster_states(events);
      
        // 2. Для каждого кластера — анализ действий и наград
        let mut patterns = Vec::new();
      
        for cluster in state_clusters {
            // Группировка по действиям внутри кластера
            let action_groups = self.group_by_action(&cluster);
          
            for (action, group) in action_groups {
                // Статистика наград для этой комбинации state-action
                let reward_stats = self.calculate_reward_stats(&group);
              
                // Если есть значимая корреляция
                if reward_stats.confidence > 0.8 {
                    patterns.push(Pattern {
                        state_cluster_id: cluster.id,
                        action_pattern: action,
                        expected_reward: reward_stats.mean,
                        confidence: reward_stats.confidence,
                        sample_size: group.len(),
                    });
                }
            }
        }
      
        self.patterns_found += patterns.len() as u64;
        self.analysis_cycles += 1;
      
        patterns
    }
  
    /// Кластеризация состояний (упрощенный k-means)
    fn cluster_states(&self, events: &[ProcessedEvent]) -> Vec<StateCluster> {
        // Извлечение state векторов
        let states: Vec<[f32; 8]> = events.iter()
            .map(|e| e.base.state)
            .collect();
      
        // Простой k-means в 8D пространстве
        // В реальности здесь будет более сложный алгоритм
        simple_kmeans(&states, self.min_cluster_size)
    }
}
```

---

## 6. TokenCompressor — сжатие для архива

### 6.1 Алгоритм компрессии

```rust
pub struct TokenCompressor {
    compression_level: CompressionLevel,
}

impl TokenCompressor {
    /// Преобразование ProcessedEvent → ExperienceToken (128 байт)
    pub fn compress(&self, event: &ProcessedEvent) -> ExperienceToken {
        ExperienceToken {
            // Идентификация (24 байта)
            event_id: compress_uuid(event.base.event_id),  // 16 → 8 байт
            timestamp: (event.base.timestamp / 1000) as u32, // μs → ms, 4 байта
            episode_id: event.base.episode_id as u32,       // 4 байта
            step: event.base.step_number as u16,            // 2 байта
            event_type: event.base.event_type as u8,        // 1 байт
            flags: compress_flags(event.base.flags),        // 1 байт
          
            // State — квантизация до int8 (8 байт)
            state_compressed: quantize_vector(event.base.state),
          
            // Action — квантизация до int8 (8 байт)  
            action_compressed: quantize_vector(event.base.action),
          
            // Rewards — сохраняем все компоненты (16 байт)
            reward_total: event.total_reward,
            reward_homeostasis: event.reward_components[0],
            reward_goal: event.reward_components[1],
            reward_curiosity: event.reward_components[2],
          
            // ADNA контекст (8 байт)
            adna_hash: event.base.adna_version_hash,
            adna_generation: extract_generation(event.base.adna_version_hash),
          
            // Оставшееся место для метаданных (64 байта)
            metadata: compress_metadata(event),
        }
    }
  
    /// Обратное преобразование для replay
    pub fn decompress(&self, token: &ExperienceToken) -> ApproximateEvent {
        // Восстановление с потерями (квантизация необратима)
        ApproximateEvent {
            state: dequantize_vector(token.state_compressed),
            action: dequantize_vector(token.action_compressed),
            reward: token.reward_total,
            // ... остальные поля
        }
    }
}
```

---

## 7. Интеграция с ADNA v3

### 7.1 Использование конфигурации

```rust
impl IntuitionModule {
    /// Обновление конфигурации из ADNA
    pub async fn update_from_adna(&mut self, adna: &ADNAv3) {
        // Layer 2: Appraisers — обновляем веса
        self.appraisers.weights = AppraiserWeights {
            homeostasis: adna.layer2_appraisers.homeostasis_weight,
            goal_directed: adna.layer2_appraisers.goal_weight,
            curiosity: adna.layer2_appraisers.curiosity_weight,
            efficiency: adna.layer2_appraisers.efficiency_weight,
        };
      
        // Layer 3: Learning — параметры обучения
        self.pattern_analyzer.min_cluster_size = 
            adna.layer3_learning.min_pattern_samples as usize;
        self.pattern_analyzer.similarity_threshold = 
            adna.layer3_learning.pattern_similarity_threshold;
      
        // Layer 4: Intuition — параметры анализа
        self.correlation_engine.confidence_threshold = 
            adna.layer4_intuition.confidence_threshold;
        self.correlation_engine.max_lag = 
            adna.layer4_intuition.temporal_window_steps;
    }
}
```

---

## 8. Оптимизация производительности

### 8.1 Ключевые решения

1. **Lock-free структуры где возможно**

   - HotBuffer использует atomic индексы
   - Appraisers работают без блокировок
2. **Батчинг и throttling**

   ```rust
   // Обработка событий батчами по 100
   let batch_size = 100;
   let throttle_ms = 10;
   ```
3. **Приоритезация событий**

   ```rust
   enum EventPriority {
       Critical = 0,    // Ошибки, аварии
       High = 1,        // Целевые действия
       Normal = 2,      // Обычная активность
       Low = 3,         // Фоновые процессы
   }
   ```
4. **Адаптивная частота анализа**

   ```rust
   // Анализируем чаще при высокой активности
   let analysis_interval = match self.metrics.events_per_second {
       0..=10 => Duration::from_secs(60),     // Редко
       11..=100 => Duration::from_secs(10),   // Нормально
       101..=1000 => Duration::from_secs(1),  // Часто
       _ => Duration::from_millis(100),       // Очень часто
   };
   ```

### 8.2 Метрики производительности

```rust
pub struct IntuitionMetrics {
    // Throughput
    pub events_per_second: f64,
    pub patterns_per_minute: f64,
  
    // Latency
    pub event_processing_p50_μs: u64,
    pub event_processing_p99_μs: u64,
  
    // Buffer health
    pub hot_buffer_utilization: f32,
    pub hot_buffer_overwrites_per_sec: f64,
  
    // Appraisers
    pub appraiser_latencies_μs: [u64; 4],
  
    // Memory
    pub total_memory_mb: usize,
    pub archive_size_mb: usize,
}
```

---

## 9. API для внешних модулей

### 9.1 Публичный интерфейс

```rust
#[async_trait]
pub trait IntuitionAPI {
    /// Записать событие (основной вход)
    async fn write_event(&self, event: ExperienceEvent) -> Result<Uuid>;
  
    /// Получить последние N обработанных событий
    async fn get_recent_events(&self, n: usize) -> Vec<ProcessedEvent>;
  
    /// Найти паттерны в заданном временном окне
    async fn find_patterns(&self, 
        start_time: u64, 
        end_time: u64
    ) -> Vec<Pattern>;
  
    /// Получить текущие метрики
    async fn get_metrics(&self) -> IntuitionMetrics;
  
    /// Принудительный анализ (для отладки)
    async fn force_analysis(&self) -> Result<AnalysisReport>;
}
```

### 9.2 Подписка на события

```rust
/// Канал для подписчиков
pub struct IntuitionSubscriber {
    receiver: tokio::sync::broadcast::Receiver<ProcessedEvent>,
}

impl IntuitionModule {
    /// Создать нового подписчика
    pub fn subscribe(&self) -> IntuitionSubscriber {
        IntuitionSubscriber {
            receiver: self.event_processor.broadcast_channel.subscribe(),
        }
    }
}
```

---

## 10. Конфигурация

### 10.1 IntuitionConfig

```toml
[intuition]
# Hot Buffer
hot_buffer_capacity = 100000        # событий в памяти
hot_buffer_type = "ring"            # "ring" или "lru"

# Event Processing  
batch_size = 100                    # событий в батче
throttle_ms = 10                    # задержка между батчами
max_parallel_appraisers = 4         # параллельных оценщиков

# Pattern Analysis
analysis_interval_secs = 10         # частота анализа
min_pattern_confidence = 0.8        # мин. уверенность
pattern_cache_size = 10000          # кешированных паттернов

# Archive
archive_enabled = true              # сохранять в архив
archive_compression = "lz4"         # тип сжатия
archive_retention_days = 30         # срок хранения

# Performance
use_simd = true                     # SIMD оптимизации
thread_pool_size = 4                # потоков для анализа
```

---

## 11. Пример использования

```rust
use neurograph_os::intuition::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Создание модуля
    let config = IntuitionConfig::from_file("intuition.toml")?;
    let adna = ADNAv3::load()?;
  
    let mut intuition = IntuitionModule::new(config, adna);
  
    // Запуск фоновых задач
    let intuition_handle = tokio::spawn(async move {
        intuition.run().await
    });
  
    // Подписка на обработанные события
    let mut subscriber = intuition.subscribe();
  
    tokio::spawn(async move {
        while let Ok(event) = subscriber.recv().await {
            println!("Processed event: {:?}", event);
            println!("Total reward: {}", event.total_reward);
        }
    });
  
    // Отправка события
    let event = ExperienceEvent {
        event_id: Uuid::new_v4(),
        timestamp: current_timestamp_μs(),
        state: [1.0, 0.5, 0.0, -0.5, 0.2, 0.8, 0.0, 0.0],
        action: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        // ... остальные поля
    };
  
    intuition.write_event(event).await?;
  
    // Ждем завершения
    intuition_handle.await?;
  
    Ok(())
}
```

---

## 12. Заключение

Эта расширенная спецификация обеспечивает:

1. **Интеграцию с существующими модулями** — ExperienceStream, ADNA v3, Token/Connection
2. **Трехуровневую обработку** — горячий/теплый/холодный пути
3. **Масштабируемость** — параллельные appraisers, батчинг, throttling
4. **Эффективность** — кольцевые буферы, компрессия, SIMD
5. **Гибкость** — конфигурируемые параметры через ADNA

Модуль готов к поэтапной реализации без необходимости переписывать существующие компоненты.
