# Telegram Bot Setup Guide - Gateway v2.0

> **Пошаговая инструкция по запуску Telegram бота с Gateway v2.0**

---

## 📋 Требования

- Python 3.8+
- NeuroGraph OS (Gateway v2.0)
- Telegram аккаунт
- Доступ к интернету

---

## 🚀 Быстрый старт (5 минут)

### Шаг 1: Установить зависимости

```bash
pip install python-telegram-bot
```

### Шаг 2: Создать бота через @BotFather

1. Открыть Telegram
2. Найти @BotFather (официальный бот Telegram)
3. Отправить `/newbot`
4. Следовать инструкциям:
   - Ввести имя бота (например: "My NeuroGraph Bot")
   - Ввести username (например: "my_neurograph_bot")
5. **Скопировать токен** (выглядит как `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`)

### Шаг 3: Установить токен

```bash
export TELEGRAM_BOT_TOKEN="ваш_токен_здесь"
```

### Шаг 4: Запустить бота

```bash
cd /path/to/neurograph-os-mvp
python examples/telegram_bot_simple.py
```

### Шаг 5: Протестировать

1. Найти своего бота в Telegram (по username)
2. Отправить `/start`
3. Отправить любое сообщение
4. Бот покажет, как оно обработалось через Gateway!

---

## 📚 Два примера ботов

### Simple Bot (`telegram_bot_simple.py`)

**Функции:**
- ✅ Базовая обработка сообщений
- ✅ Команды `/start`, `/help`, `/stats`, `/reset`
- ✅ Показывает 8D вектор и метаданные
- ✅ Conversation tracking
- ✅ Event filtering demonstration

**Когда использовать:**
- Учебный пример
- Тестирование Gateway
- Простой чат-бот

**Запуск:**
```bash
export TELEGRAM_BOT_TOKEN="your_token"
python examples/telegram_bot_simple.py
```

### Advanced Bot (`telegram_bot_advanced.py`)

**Функции:**
- ✅ Subscription system (4 подписчика)
- ✅ Analytics tracking
- ✅ High-priority detection
- ✅ Sentiment analysis
- ✅ Event logging to file
- ✅ Multiple filters
- ✅ `/priority` command for urgent messages

**Когда использовать:**
- Production bot
- Демонстрация subscription filters
- Event-driven architecture
- Analytics и мониторинг

**Запуск:**
```bash
export TELEGRAM_BOT_TOKEN="your_token"
python examples/telegram_bot_advanced.py
```

---

## 🔧 Конфигурация

### Постоянная установка токена (Linux/Mac)

Добавить в `~/.bashrc` или `~/.zshrc`:

```bash
export TELEGRAM_BOT_TOKEN="123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
```

Затем:
```bash
source ~/.bashrc  # или source ~/.zshrc
```

### Проверка токена

```bash
echo $TELEGRAM_BOT_TOKEN
# Должен показать токен
```

### Альтернатива: .env файл (не рекомендуется для production)

```bash
# Создать файл .env
echo "TELEGRAM_BOT_TOKEN=your_token" > .env

# Использовать python-dotenv
pip install python-dotenv

# В коде:
from dotenv import load_dotenv
load_dotenv()
```

---

## 📖 Использование

### Simple Bot - Команды

| Команда | Описание |
|---------|----------|
| `/start` | Показать приветствие и инструкции |
| `/help` | Показать функции Gateway |
| `/stats` | Показать статистику (события, sensors, NeuroTick) |
| `/reset` | Сбросить conversation (начать новый thread) |

**Обычные сообщения:**
- Любой текст обрабатывается через Gateway
- Показывается 8D вектор, priority, urgency, encoding
- Проверяется фильтр telegram_user_messages_filter

### Advanced Bot - Команды

| Команда | Описание |
|---------|----------|
| `/start` | Приветствие |
| `/stats` | Gateway stats + Analytics + Sentiment counts |
| `/subscribers` | Список активных подписчиков и их статистика |
| `/priority <text>` | Отправить сообщение с высоким приоритетом |

**Обычные сообщения:**
- Обрабатываются всеми подписчиками
- Analytics считает статистику
- Sentiment анализирует тональность
- HighPriority реагирует на срочные сообщения
- Logging записывает в файл

### Подписчики в Advanced Bot

1. **AnalyticsSubscriber**
   - Фильтр: `signal.input.*` (все входящие события)
   - Считает: общее количество сообщений, команды, уникальных пользователей
   - Логирует в консоль

2. **HighPrioritySubscriber**
   - Фильтр: `telegram_high_priority_filter` (priority >= 200, urgency >= 0.7)
   - Отправляет уведомление пользователю об urgent message
   - Демонстрирует proactive bot behavior

3. **SentimentSubscriber**
   - Фильтр: `modality == text`
   - Анализирует polarity из semantic vector
   - Реагирует на очень позитивные/негативные сообщения
   - Считает статистику по sentiment

4. **LoggingSubscriber**
   - Фильтр: `signal.*` (все события)
   - Записывает в `gateway_events.log`
   - Формат: timestamp, event_id, type, tick, text

---

## 🎯 Примеры использования

### Пример 1: Простое общение (Simple Bot)

```
User: Hello!

Bot: ✅ Message processed!

📝 Text: Hello!

Gateway Processing:
• 8D Vector: [1.00, 0.00, 0.00, 0.00, ...]
• Priority: 200
• Urgency: 0.78
• NeuroTick: 1
• Encoding: text_tfidf

Event ID: a48c9f0a-d4ef...

✨ Matched subscription filter!
```

### Пример 2: Статистика (Simple Bot)

```
User: /stats

Bot: 📊 Gateway Statistics

Total Events: 5
NeuroTick: 5
Registered Sensors: 3
Active Conversations: 1

Your Conversation:
Chat ID: 123456789
Sequence: conv_telegram_123456789_0
Messages: 5
```

### Пример 3: High Priority (Advanced Bot)

```
User: /priority This is urgent!

Bot: ⚡ Sent with high priority (priority=220)
Event: a48c9f0a-d4ef...

[Immediately after]

Bot: ⚡ High priority message detected! Your message is being processed with urgency.

[Console output]
🔥 [HighPriority] Urgent message detected in chat 123456789
```

### Пример 4: Sentiment Response (Advanced Bot)

```
User: I absolutely love this system! It's amazing!

Bot: ✅ Processed (tick=3, vec=[0.95, 0.20, ...])

[Immediately after]

Bot: 😊 I sense very positive energy in your message!

[Console output]
💭 [Sentiment] positive (0.95)
```

### Пример 5: Subscribers Info (Advanced Bot)

```
User: /subscribers

Bot: Active Subscribers (4):

📌 Analytics
  Events handled: 12
  Filter: {'event_type': {'$wildcard': 'signal.input.*'}}

📌 HighPriority
  Events handled: 2
  Filter: {'$and': [...]}

📌 Sentiment
  Events handled: 10
  Filter: {'source.modality': 'text'}

📌 Logging
  Events handled: 12
  Filter: {'event_type': {'$wildcard': 'signal.*'}}
```

---

## 🔍 Troubleshooting

### Проблема: Bot not responding

**Причины:**
- Неправильный токен
- Бот не запущен
- Сетевые проблемы

**Решение:**
```bash
# Проверить токен
echo $TELEGRAM_BOT_TOKEN

# Проверить что бот запущен
# Должно быть: "✅ Bot is running!"

# Проверить интернет
ping telegram.org
```

### Проблема: ImportError: No module named 'telegram'

**Решение:**
```bash
pip install python-telegram-bot
```

### Проблема: ModuleNotFoundError: No module named 'src'

**Решение:**
```bash
# Запускать из корня проекта
cd /path/to/neurograph-os-mvp
python examples/telegram_bot_simple.py

# Или установить PYTHONPATH
export PYTHONPATH=/path/to/neurograph-os-mvp
```

### Проблема: Error: TELEGRAM_BOT_TOKEN not set

**Решение:**
```bash
export TELEGRAM_BOT_TOKEN="your_token_here"

# Проверить
echo $TELEGRAM_BOT_TOKEN
```

### Проблема: Бот показывает старые данные после изменений

**Решение:**
- Остановить бота (Ctrl+C)
- Запустить заново
- Gateway создаётся заново при каждом запуске

---

## 📊 Мониторинг

### Логи в консоли (Advanced Bot)

```
📊 [Analytics] Total: 5 msgs, 2 users
💭 [Sentiment] positive (0.85)
🔥 [HighPriority] Urgent message detected in chat 123456789
```

### Логи в файле (Advanced Bot)

Файл: `gateway_events.log`

```
2025-12-22T15:30:45.123456 | Event: a48c9f0a | Type: signal.input.external.text.text_chat | Tick: 1 | Text: Hello!
2025-12-22T15:30:47.234567 | Event: b59d8e1b | Type: signal.input.external.text.text_chat | Tick: 2 | Text: This is a test
```

### Gateway статистика

```python
# В коде бота можно получить:
stats = self.gateway.get_stats()
# {
#   "total_events": 42,
#   "neuro_tick": 42,
#   "registered_sensors": 3,
#   "enabled_sensors": 3
# }
```

---

## 🎓 Обучение

### Рекомендуемая последовательность

1. **День 1**: Запустить Simple Bot
   - Понять flow: Message → TelegramAdapter → Gateway → Event
   - Поэкспериментировать с разными текстами
   - Посмотреть как меняются векторы

2. **День 2**: Изучить фильтры
   - Запустить Advanced Bot
   - Посмотреть как работают подписчики
   - Отправить `/subscribers` и `/stats`

3. **День 3**: Создать своего подписчика
   - Скопировать `telegram_bot_advanced.py`
   - Добавить свой `CustomSubscriber`
   - Протестировать фильтры

4. **День 4**: Интегрировать с Core (будущее)
   - Подключить `_core.SignalSystem`
   - Получать ProcessingResult
   - Использовать triggered_actions

---

## 🔗 Связанные ресурсы

- [Gateway v2.0 User Guide](Gateway_v2_0_User_Guide.md)
- [Subscription Filters](Gateway_v2_0_User_Guide.md#subscription-filters)
- [TelegramAdapter API](../api/gateway_adapters.md)
- [Examples](../../examples/)

---

## 📝 Чек-лист для первого запуска

- [ ] Python 3.8+ установлен
- [ ] `pip install python-telegram-bot` выполнен
- [ ] Бот создан через @BotFather
- [ ] Токен скопирован
- [ ] `export TELEGRAM_BOT_TOKEN="..."` выполнен
- [ ] Бот запущен (видно "✅ Bot is running!")
- [ ] Бот найден в Telegram
- [ ] `/start` отправлен и получен ответ
- [ ] Обычное сообщение отправлено и обработано

**Если все ✅ - поздравляю! Бот работает! 🎉**
