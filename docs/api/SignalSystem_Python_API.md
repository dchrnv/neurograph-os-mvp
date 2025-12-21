# SignalSystem v1.1 - Python API Reference

**Version:** 1.1.0
**Module:** `neurograph._core.SignalSystem`
**Status:** ✅ Implemented

---

## Overview

SignalSystem provides a high-performance event processing system with subscription-based filtering. Events are processed in Rust with minimal Python overhead through PyO3 bindings.

**Key Features:**
- 🚀 Fast event processing (<100μs typical)
- 🔍 Flexible subscription filters (wildcard, numeric, bitmap)
- 🔄 Multiple callback types (push/polling)
- 📊 Real-time statistics
- 🧵 Thread-safe (GIL release during processing)

---

## Installation

```bash
cd src/core_rust
maturin develop --features python-bindings
```

---

## Quick Start

```python
from neurograph._core import SignalSystem

# Create system
system = SignalSystem()

# Emit event
result = system.emit(
    event_type="signal.input.text",
    vector=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    priority=200
)

# Subscribe with filter
def handler(event):
    print(f"Received: {event}")

sub_id = system.subscribe(
    name="my_handler",
    filter_dict={"event_type": {"$wildcard": "signal.input.*"}},
    callback=handler
)

# Get stats
stats = system.get_stats()
print(f"Total events: {stats['total_events']}")
```

---

## API Reference

### Class: `SignalSystem`

Main event processing coordinator.

#### Constructor

```python
system = SignalSystem()
```

Creates a new SignalSystem with default configuration.

**Returns:** `SignalSystem` instance

**Example:**
```python
system = SignalSystem()
print(f"Active subscribers: {system.subscriber_count()}")
```

---

### Methods

#### `emit(event_type, vector, priority=128, **kwargs)` → `dict`

Emits a signal event into the system.

**Parameters:**
- `event_type` (str): Event type identifier (e.g. "signal.input.text.chat")
- `vector` (List[float]): 8D semantic vector (must be length 8)
- `priority` (int, optional): Priority 0-255, default=128
- `**kwargs`: Additional event fields:
  - `confidence` (int): 0-255
  - `urgency` (int): 0-255
  - `magnitude` (int): -32768 to 32767
  - `layers` (List[float]): 8D layer affinities

**Returns:** `dict` with processing result:
```python
{
    "token_id": int,
    "energy_delta": float,
    "activation_spread": int,
    "is_novel": bool,
    "anomaly_score": float,
    "processing_time_us": int,
    "neighbors": [
        {
            "token_id": int,
            "distance": float,
            "resonance": float,
            "token_type": int,
            "layer_affinity": int
        },
        ...
    ],
    "triggered_actions": [int, ...]
}
```

**Raises:**
- `ValueError`: If vector length != 8

**Example:**
```python
result = system.emit(
    event_type="signal.input.text.user_message",
    vector=[0.5, 0.3, 0.7, 0.2, 0.8, 0.1, 0.4, 0.6],
    priority=200,
    confidence=180,
    urgency=150
)

print(f"Token ID: {result['token_id']}")
print(f"Is novel: {result['is_novel']}")
print(f"Processing time: {result['processing_time_us']}μs")
```

---

#### `subscribe(name, filter_dict, callback=None)` → `int`

Subscribes to events matching a filter.

**Parameters:**
- `name` (str): Human-readable subscriber name
- `filter_dict` (dict): Filter specification (see Filter Syntax below)
- `callback` (Callable[[dict], None], optional):
  - If provided: push mode (callback invoked on match)
  - If None: polling mode (use `poll()` - TODO in Phase 5)

**Returns:** `int` - Subscriber ID

**Raises:**
- `ValueError`: Invalid filter syntax
- `RuntimeError`: Subscription failed (e.g. too many subscribers)

**Example:**
```python
def event_handler(event):
    print(f"Event: {event['event']['event_type_id']}")
    print(f"Priority: {event['event']['priority']}")

sub_id = system.subscribe(
    name="high_priority_monitor",
    filter_dict={
        "priority": {"$gte": 200},
        "event_type": {"$wildcard": "signal.input.*"}
    },
    callback=event_handler
)

print(f"Subscribed with ID: {sub_id}")
```

---

#### `unsubscribe(subscriber_id)` → `None`

Removes a subscription.

**Parameters:**
- `subscriber_id` (int): ID returned from `subscribe()`

**Raises:**
- `RuntimeError`: Subscriber not found

**Example:**
```python
system.unsubscribe(sub_id)
```

---

#### `get_stats()` → `dict`

Returns current system statistics.

**Returns:** `dict`:
```python
{
    "total_events": int,
    "avg_processing_time_us": float,
    "subscriber_notifications": int,
    "filter_matches": int,
    "filter_misses": int,
    "events_by_type": {
        type_id (int): count (int),
        ...
    }
}
```

**Example:**
```python
stats = system.get_stats()
print(f"Total events: {stats['total_events']}")
print(f"Avg latency: {stats['avg_processing_time_us']:.2f}μs")
print(f"Hit rate: {stats['filter_matches'] / (stats['filter_matches'] + stats['filter_misses']) * 100:.1f}%")
```

---

#### `reset_stats()` → `None`

Resets all statistics to zero.

**Example:**
```python
system.reset_stats()
```

---

#### `subscriber_count()` → `int`

Returns number of active subscribers.

**Example:**
```python
count = system.subscriber_count()
print(f"Active subscribers: {count}")
```

---

## Filter Syntax

Filters are specified as Python dictionaries that compile to efficient Rust predicates.

### Event Type Filter

**Exact match:**
```python
{"event_type": "signal.input.text.chat"}
```

**Wildcard match:**
```python
{"event_type": {"$wildcard": "signal.input.*"}}
{"event_type": {"$wildcard": "signal.input.external.*"}}
{"event_type": {"$wildcard": "*.text.*"}}
```

### Numeric Filters

**Supported fields:**
- `priority` (0-255)
- `confidence` (0-255)
- `urgency` (0-255)
- `magnitude` (-32768 to 32767)
- `arousal` (0-255)
- `valence` (-128 to 127)
- `layer.physical`, `layer.spatial`, `layer.temporal`, `layer.causal`, `layer.emotional`, `layer.social`, `layer.abstract`, `layer.meta` (0-255, scaled from 0.0-1.0)

**Operators:**
- `$eq`: Equal
- `$ne`: Not equal
- `$gt`: Greater than
- `$gte`: Greater than or equal
- `$lt`: Less than
- `$lte`: Less than or equal

**Examples:**
```python
# High priority events
{"priority": {"$gte": 200}}

# Moderate confidence
{"confidence": {"$gte": 100, "$lte": 200}}

# High emotional layer
{"layer.emotional": {"$gt": 200}}
```

### Combined Filters

**Implicit AND:**
```python
{
    "event_type": {"$wildcard": "signal.input.*"},
    "priority": {"$gte": 150}
}
```

**Explicit AND:**
```python
{
    "$and": [
        {"event_type": "signal.input.text"},
        {"priority": {"$gte": 200}}
    ]
}
```

**OR:**
```python
{
    "$or": [
        {"priority": {"$gte": 250}},
        {"urgency": {"$gte": 240}}
    ]
}
```

---

## Performance Characteristics

| Operation | Typical Latency | Notes |
|-----------|----------------|-------|
| `emit()` | 30-50μs | Without Grid/Graph integration |
| `emit()` (with integration) | 80-120μs | Phase 5 target |
| Filter match | <1μs | Per filter, per event |
| Subscribe/Unsubscribe | 3-8μs | One-time cost |

**Throughput:** >10,000 events/sec on AMD Ryzen 5 3500U

---

## Thread Safety

- ✅ **Thread-safe:** All methods can be called from multiple Python threads
- ✅ **GIL release:** `emit()` releases GIL during Rust processing
- ✅ **Async-compatible:** Can be used in async contexts (though methods are sync)

---

## Error Handling

```python
from neurograph._core import SignalSystem

system = SignalSystem()

try:
    # Invalid vector length
    system.emit(
        event_type="test",
        vector=[0.1, 0.2]  # Wrong length!
    )
except ValueError as e:
    print(f"Error: {e}")  # "Vector must have 8 dimensions, got 2"

try:
    # Invalid filter
    system.subscribe(
        name="test",
        filter_dict={"unknown_field": 123},
        callback=lambda e: None
    )
except ValueError as e:
    print(f"Error: {e}")  # "Unknown field: unknown_field"
```

---

## Best Practices

### 1. Reuse SignalSystem instance

```python
# ✅ Good: Single instance
system = SignalSystem()

# ❌ Bad: Multiple instances
for i in range(100):
    system = SignalSystem()  # Wasteful!
```

### 2. Use wildcard filters efficiently

```python
# ✅ Good: Specific wildcard
{"event_type": {"$wildcard": "signal.input.external.*"}}

# ⚠️ Careful: Too broad
{"event_type": {"$wildcard": "*"}}  # Matches everything
```

### 3. Monitor statistics

```python
# Periodically check filter efficiency
stats = system.get_stats()
hit_rate = stats['filter_matches'] / (stats['filter_matches'] + stats['filter_misses'])
if hit_rate < 0.1:  # <10% hit rate
    print("Warning: Most events are not matching filters")
```

### 4. Clean up subscriptions

```python
# ✅ Good: Unsubscribe when done
sub_id = system.subscribe(...)
try:
    # ... use subscription ...
finally:
    system.unsubscribe(sub_id)
```

---

## Future Enhancements (Phase 5+)

- 📊 Grid/Graph integration (neighbor search, activation)
- 🛡️ Guardian validation hooks
- 📈 Performance profiling API
- 🔄 Batch emit for high throughput
- 📝 Polling mode for subscriptions

---

## See Also

- [SignalSystem v1.1 Specification](../specs/SignalSystem_v1_1.md)
- [Example Code](../../examples/signal_system_basic.py)
- [Architecture Documentation](../specs/SignalSystem_v1_1.md#architecture)

---

**Last Updated:** 2025-12-20
**API Version:** 1.1.0
