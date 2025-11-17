# Участие в разработке NeuroGraph

Спасибо за интерес к проекту! Мы рады любому вкладу.

## Как помочь проекту

### 1. Исследование и обратная связь

- Тестируйте MVP и сообщайте о багах
- Предлагайте улучшения в Issues
- Делитесь идеями по архитектуре

### 2. Разработка

- Добавляйте новые фичи
- Исправляйте баги
- Улучшайте документацию
- Пишите тесты

### 3. Документация

- Улучшайте существующие docs
- Добавляйте примеры использования
- Переводите на другие языки

---

## Процесс разработки

### Шаг 1: Fork и клонирование

```bash
# Fork репозитория через GitHub UI
git clone https://github.com/YOUR_USERNAME/neurograph-os-mvp.git
cd neurograph-os-mvp
git remote add upstream https://github.com/dchrnv/neurograph-os-mvp.git
```

### Шаг 2: Создание ветки

```bash
git checkout -b feature/your-feature-name
# или
git checkout -b bugfix/issue-123
```

**Naming conventions:**

- `feature/` - новые функции
- `bugfix/` - исправления багов
- `docs/` - изменения документации
- `refactor/` - рефакторинг без изменения API

### Шаг 3: Разработка

```bash
# Rust core development
cd src/core_rust

# Build library
cargo build --lib

# Run tests
cargo test --lib

# Run benchmarks
cargo bench

# Run integration tests
cargo test --tests
```

### Шаг 4: Тестирование

```bash
# Запустить все тесты библиотеки
cd src/core_rust
cargo test --lib

# Запустить конкретный тест
cargo test --lib test_name

# Запустить с выводом
cargo test --lib -- --nocapture

# Integration tests
cargo test --test learning_loop_e2e
cargo test --test action_controller_e2e
cargo test --test persistence_e2e
```

### Шаг 5: Коммит

```bash
git add .
git commit -m "feat: add awesome feature

Detailed description of what was changed and why.

Closes #123"
```

**Commit message format:**

```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

**Types:**

- `feat:` - новая функция
- `fix:` - исправление бага
- `docs:` - изменения в документации
- `refactor:` - рефакторинг
- `test:` - добавление тестов
- `chore:` - изменения инфраструктуры

### Шаг 6: Push и Pull Request

```bash
git push origin feature/your-feature-name
```

Затем создайте Pull Request через GitHub UI:

1. Опишите что изменили
2. Укажите связанные Issues
3. Добавьте скриншоты (если UI)
4. Отметьте чеклист

---

## Checklist для Pull Request

- [ ] Код работает и протестирован
- [ ] Добавлены тесты для новой функциональности
- [ ] Документация обновлена (README, docs/)
- [ ] Commit messages следуют формату
- [ ] Нет конфликтов с main веткой
- [ ] `cargo test --lib` проходит без ошибок
- [ ] `cargo build --lib` компилируется без warnings

---

## Code Style

### Rust

- Следуем Rust API Guidelines
- Используем `cargo fmt` для форматирования
- Используем `cargo clippy` для линтинга
- Документируем публичные API с `///` doc comments
- Максимальная длина строки: 100 символов

```rust
/// Creates a new token with specified parameters
///
/// # Arguments
///
/// * `id` - Unique token identifier (u32)
///
/// # Returns
///
/// New Token instance with default values
///
/// # Example
///
/// ```
/// let token = Token::new(42);
/// assert_eq!(token.id, 42);
/// ```
pub fn new(id: u32) -> Self {
    Self {
        id,
        weight: 0.0,
        // ... other fields
    }
}
```

### TypeScript/React (Desktop UI)

- ESLint rules
- Functional components с hooks
- Typed props
- CSS modules или styled-components

```typescript
interface TokenCardProps {
  token: Token;
  onDelete: (id: number) => void;
}

export const TokenCard: React.FC<TokenCardProps> = ({ token, onDelete }) => {
  return (
    <div className="token-card">
      <h3>Token #{token.id}</h3>
      <button onClick={() => onDelete(token.id)}>Delete</button>
    </div>
  );
};
```

---

## Тестирование

### Unit Tests (Rust)

```rust
// src/core_rust/src/token.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(42);
        let token_id = token.id;
        assert_eq!(token_id, 42);
    }
}
```

### Integration Tests (Rust)

```rust
// src/core_rust/tests/integration/learning_loop_e2e.rs
#[tokio::test]
async fn test_full_learning_loop() {
    // Setup components
    let mut stream = ExperienceStream::new();
    let intuition = IntuitionEngine::new();

    // Test learning loop
    // ...
}
```

### Benchmarks (Rust)

```rust
// src/core_rust/benches/token_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_token_creation(c: &mut Criterion) {
    c.bench_function("token_new", |b| {
        b.iter(|| Token::new(black_box(1)))
    });
}

criterion_group!(benches, bench_token_creation);
criterion_main!(benches);
```

---

## Reporting Bugs

При создании Issue для бага, укажите:

1. **Описание проблемы** - что произошло
2. **Ожидаемое поведение** - что должно было быть
3. **Шаги воспроизведения** - как повторить баг
4. **Окружение**:
   - OS: Linux/macOS/Windows
   - Rust версия: `rustc --version`
   - Версия NeuroGraph: (из Cargo.toml)
5. **Логи/Скриншоты** - если есть

**Пример:**

```markdown
### Описание
Token creation fails with panic when coordinates out of bounds

### Шаги воспроизведения
1. Create token: `Token::new(1)`
2. Set coordinates: `token.set_coordinates(L1Physical, 9999.0, 0.0, 0.0)`
3. Panic occurs

### Ожидаемое поведение
Should clamp coordinates or return Result<>

### Окружение
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Version: v0.27.0
```

---

## Предложение фич

При создании Issue для новой фичи, укажите:

1. **Проблема** - какую задачу решает фича
2. **Решение** - как вы предлагаете её решить
3. **Альтернативы** - другие варианты решения
4. **Приоритет** - насколько это важно

---

## Архитектура проекта

Перед началом разработки, ознакомьтесь с:

- [README.md](README.md) - общее описание проекта
- [docs/PROJECT_HISTORY.md](docs/PROJECT_HISTORY.md) - история версий
- [architecture_blueprint.json](architecture_blueprint.json) - архитектура системы

**Основные модули:**

- `src/core_rust/src/token.rs` - Token V2.0 (64 bytes)
- `src/core_rust/src/connection.rs` - Connection V2.0 (32 bytes)
- `src/core_rust/src/grid.rs` - 8D Spatial indexing
- `src/core_rust/src/graph.rs` - Topological navigation
- `src/core_rust/src/cdna.rs` - Constitutional DNA
- `src/core_rust/src/adna.rs` - Active DNA (Policy Engine)
- `src/core_rust/src/experience_stream.rs` - Experience tracking
- `src/core_rust/src/intuition_engine.rs` - Pattern detection
- `src/core_rust/src/action_controller.rs` - Action selection

---

## Правила документации

### Общие правила:

1. **README.md** - только актуальная версия проекта
2. **PROJECT_HISTORY.md** - вся история разработки
3. **CONTRIBUTING.md** - гайд для контрибьюторов

### В коде:

- Rust: используем `///` doc comments для публичных API
- Пишем примеры в docstrings с `# Example`
- Документируем сложные алгоритмы
- Комментируем ПОЧЕМУ, а не ЧТО

---

## Code Review Process

После создания PR:

1. **Automated checks** - GitHub Actions запустит тесты
2. **Code review** - мейнтейнеры проверят код
3. **Discussion** - обсуждение изменений в комментариях
4. **Approval** - получение approval от мейнтейнеров
5. **Merge** - слияние в main ветку

---

## Признание вклада

Все контрибьюторы будут добавлены в:

- GitHub Contributors list
- CONTRIBUTORS.md (планируется)
- Release notes

---

## Вопросы?

- **GitHub Issues** - для багов и фич
- **GitHub Discussions** - для общих вопросов
- **Email**: <dreeftwood@gmail.com> - для приватных вопросов

---

## Лицензия

Этот проект лицензирован на условиях **GNU Affero General Public License v3.0**.
Полный текст лицензии можно найти в файле [LICENSE](LICENSE).

---

Спасибо за вклад в NeuroGraph!
