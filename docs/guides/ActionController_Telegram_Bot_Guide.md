## Telegram Bot with ActionController - Guide

> **Complete integration: Gateway v2.0 + ActionController + SignalPipeline**

---

## 📋 Overview

This bot demonstrates the **full v0.56.0 stack**:

```
Telegram Message
      ↓
SignalPipeline
      ↓
Gateway v2.0 → SignalEvent (8D encoding)
      ↓
[Rust Core] → ProcessingResult (optional, future)
      ↓
ActionController → Hot/Cold Path
      ↓
Hot Path: Text Response (immediate)
Cold Path: Logging + Metrics (background)
      ↓
Telegram Response
```

---

## 🚀 Quick Start

### Step 1: Install Dependencies

```bash
pip install python-telegram-bot
```

### Step 2: Get Bot Token

1. Open Telegram
2. Find @BotFather
3. Send `/newbot`
4. Follow instructions
5. Copy token

### Step 3: Set Token

```bash
export TELEGRAM_BOT_TOKEN="123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11"
```

### Step 4: Run Bot

```bash
python examples/telegram_bot_actioncontroller.py
```

---

## 📖 Commands

| Command | Description |
|---------|-------------|
| `/start` | Show welcome message and features |
| `/stats` | View pipeline, gateway, and controller statistics |
| `/test` | Test full pipeline with detailed output |
| `<any text>` | Process through full pipeline → get response |

---

## 🎯 Features

### 1. SignalPipeline Integration

**What it does:**
- Orchestrates Gateway → Core → ActionController
- Manages end-to-end signal processing
- Tracks statistics and performance

**Code:**
```python
self.pipeline = SignalPipeline()
result = await self.pipeline.process_text(
    text=update.message.text,
    user_id=str(update.message.from_user.id),
    chat_id=str(update.message.chat.id),
    priority=200
)
```

### 2. ActionController

**What it does:**
- Selects appropriate actions based on signal
- Routes to Hot Path (immediate) or Cold Path (background)
- Executes actions and returns results

**Registered Actions:**
- `text_response` (Hot Path) - Generate text responses
- `logging` (Cold Path) - Log events to file
- `metrics` (Cold Path) - Track statistics

### 3. Hot Path / Cold Path

**Hot Path (Immediate):**
- Text response generation
- Sent directly to Telegram user
- <1ms execution time
- Blocks until complete

**Cold Path (Background):**
- Logging to `telegram_bot_actions.log`
- Metrics tracking
- Non-blocking queue
- Runs asynchronously

---

## 💡 Examples

### Example 1: Send Message

```
User: Hello!

Bot: Acknowledged.
```

**Behind the scenes:**
1. Gateway encodes "Hello!" → 8D vector
2. Core processes (if available)
3. ActionController selects `text_response` action
4. Response generated
5. Sent to Telegram

### Example 2: Check Statistics

```
User: /stats

Bot: 📊 Statistics

Pipeline:
• Total processed: 42
• Without Core: 42

Gateway:
• Events: 42
• NeuroTick: 42

ActionController:
• Total executions: 42
• Hot path: 42
• Cold path: 42
• Failed: 0
```

### Example 3: Test Pipeline

```
User: /test

Bot: 🔄 Testing full pipeline...

✅ Pipeline Test Complete

Signal Event:
• Event ID: a48c9f0a-d4ef...
• Priority: 220
• Vector: [0.00, ...]

Actions:
• Hot path: 1
• Cold path: 2

Timing:
• Total: 0.87ms
```

---

## 🏗️ Architecture

### Components

**1. SignalPipeline** ([src/integration/pipeline.py](../../src/integration/pipeline.py))
- Orchestrates full flow
- Manages Gateway + Controller
- Provides unified API

**2. Gateway v2.0** ([src/gateway/](../../src/gateway/))
- Receives raw text
- Encodes to 8D vector
- Creates SignalEvent

**3. ActionController** ([src/action_controller/](../../src/action_controller/))
- Selects actions
- Routes to Hot/Cold paths
- Executes and tracks results

**4. Action Executors** ([src/action_controller/executors/](../../src/action_controller/executors/))
- TextResponseAction - Generate responses
- LoggingAction - Log events
- MetricsAction - Track statistics

### Data Flow

```
Telegram Update
      ↓
handle_message()
      ↓
SignalPipeline.process_text()
      ├─→ Gateway.push_text() → SignalEvent
      ├─→ [Core.emit()] → ProcessingResult
      └─→ ActionController.process()
            ├─→ ActionSelector.select() → [text_response, logging, metrics]
            ├─→ Hot Path: text_response.execute() → response_text
            └─→ Cold Path: [logging, metrics] queued
      ↓
Telegram Response
```

---

## 📊 Logs and Monitoring

### Console Logs

```
INFO - SignalPipeline initialized (core=disabled)
INFO - Registered 3 action types
INFO - Configured action paths
INFO - Bot is running with ActionController!
INFO - Generated text response (12 chars, 0.05ms)
DEBUG - Gateway created event: a48c9f0a-d4ef...
```

### Log File

**File:** `telegram_bot_actions.log`

**Format:**
```json
{"timestamp": "2025-12-22T17:30:45.123456", "event_id": "a48c9f0a", "processing": {"token_id": null, "is_novel": false, "triggered_actions": []}, "data": "Hello!"}
```

### Metrics

Get via `/stats` command or `bot.pipeline.get_stats()`:

```python
{
  "pipeline": {
    "total_processed": 42,
    "with_core": 0,
    "without_core": 42
  },
  "gateway": {
    "total_events": 42,
    "neuro_tick": 42,
    "registered_sensors": 3
  },
  "controller": {
    "total_executions": 42,
    "hot_path_executed": 42,
    "cold_path_executed": 84,
    "failed_actions": 0
  }
}
```

---

## 🔧 Customization

### Add Custom Action

```python
from src.action_controller import Action, ActionResult, ActionStatus

class CustomAction(Action):
    async def execute(self, context):
        # Your custom logic here
        return ActionResult(
            action_id=self.action_id,
            status=ActionStatus.COMPLETED,
            success=True,
            data={"custom": "result"}
        )

    def can_execute(self, context):
        return True

# Register
bot.pipeline.register_action(
    action_type="custom_action",
    action_class=CustomAction,
    priority=ActionPriority.NORMAL
)

# Add to path
bot.pipeline.configure_actions(
    hot_path=["text_response", "custom_action"],
    cold_path=["logging", "metrics"]
)
```

### Add Selection Rule

```python
from src.action_controller.selector import PathType

def is_important(context):
    event = context.get("signal_event")
    return event.routing.priority >= 250

bot.pipeline.controller.selector.add_rule(
    name="important_messages",
    condition=is_important,
    action_types=["custom_action"],
    path=PathType.HOT,
    priority_boost=50
)
```

---

## 🐛 Troubleshooting

### Bot not responding

**Check:**
1. Token set correctly: `echo $TELEGRAM_BOT_TOKEN`
2. Bot running: Should see "✅ Bot is running!"
3. Internet connection

### Actions not executing

**Check:**
1. Action registered: Check `bot.pipeline.controller.registry.list_actions()`
2. Path configured: Check `bot.pipeline.get_stats()['controller']`
3. Logs for errors

### Performance issues

**Check:**
1. `/stats` command for metrics
2. Cold path queue size
3. Log file size (may need rotation)

---

## 📈 Next Steps

1. **Connect to Rust Core** (v0.57.0)
   - SignalSystem processing
   - Pattern matching
   - Novelty detection

2. **Advanced Actions**
   - Database persistence
   - External API calls
   - Multi-modal responses

3. **Production Deployment**
   - Docker container
   - Health checks
   - Prometheus metrics

---

## 🔗 See Also

- [ActionController API](../api/action_controller.md)
- [SignalPipeline Guide](SignalPipeline_Guide.md)
- [Gateway v2.0 User Guide](Gateway_v2_0_User_Guide.md)

---

**v0.56.0** - ActionController Foundation Complete! 🎉
