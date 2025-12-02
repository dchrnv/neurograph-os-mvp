# Builder Pattern Usage Examples - v0.39.2

**Version:** 0.39.2
**Date:** 2025-01-28
**Purpose:** Simplify component initialization with builder patterns

---

## Overview

Version 0.39.2 introduces builder patterns for complex components to address API usability issues identified in [API_STATUS_v0.39.1.md](../API_STATUS_v0.39.1.md).

**Key Benefits:**
- ✅ **Simpler API** - No need to manually create all dependencies
- ✅ **Sensible defaults** - Components work out of the box
- ✅ **Flexible** - Override only what you need
- ✅ **Type-safe** - Compile-time validation
- ✅ **Backward compatible** - Old `new()` methods still work

---

## IntuitionEngine - Before & After

### ❌ Before (v0.39.1) - Complex

```rust
use neurograph::{
    IntuitionEngine, IntuitionConfig,
    ExperienceStream,
    ADNAReader, InMemoryADNAReader, AppraiserConfig,
};
use std::sync::Arc;
use tokio::sync::mpsc;

// User must create ALL dependencies manually
let (proposal_tx, proposal_rx) = mpsc::channel(100);
let experience = Arc::new(ExperienceStream::new(10_000, 1_000));
let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()));

let intuition = IntuitionEngine::new(
    IntuitionConfig::default(),
    experience,
    adna,
    proposal_tx,
);
```

**Problems:**
1. User must understand mpsc channels
2. User must create Arc wrappers
3. User must know about ExperienceStream capacity parameters
4. User must know about InMemoryADNAReader and AppraiserConfig
5. Total: 8 lines of boilerplate for simple use case

---

### ✅ After (v0.39.2) - Simple

#### Option 1: Absolute Simplest (One-liner)

```rust
use neurograph::IntuitionEngine;

// Just use defaults - that's it!
let intuition = IntuitionEngine::with_defaults();
```

**What you get:**
- ExperienceStream with capacity 10,000, channel size 1,000
- InMemoryADNAReader with default AppraiserConfig
- Internal proposal channel with capacity 100
- Default IntuitionConfig (60s analysis interval, 0.7 confidence threshold)

---

#### Option 2: Builder with Custom Config

```rust
use neurograph::{IntuitionEngine, IntuitionConfig};

let custom_config = IntuitionConfig {
    analysis_interval_secs: 30,
    min_confidence: 0.8,
    ..Default::default()
};

let intuition = IntuitionEngine::builder()
    .with_config(custom_config)
    .build()
    .expect("Should build successfully");
```

---

#### Option 3: Builder with Custom Capacity

```rust
use neurograph::IntuitionEngine;

// Large-scale deployment with high throughput
let intuition = IntuitionEngine::builder()
    .with_capacity(100_000)          // Experience capacity
    .with_channel_size(10_000)       // Broadcast channel size
    .build()
    .expect("Should build successfully");
```

---

#### Option 4: Builder with Shared Components

```rust
use neurograph::{IntuitionEngine, ExperienceStream};
use std::sync::Arc;

// Create shared ExperienceStream for multiple components
let shared_experience = Arc::new(ExperienceStream::new(50_000, 5_000));

// Multiple IntuitionEngines can share the same experience stream
let intuition1 = IntuitionEngine::builder()
    .with_experience(shared_experience.clone())
    .build()
    .unwrap();

let intuition2 = IntuitionEngine::builder()
    .with_experience(shared_experience.clone())
    .build()
    .unwrap();
```

---

#### Option 5: Advanced - Full Control

```rust
use neurograph::{
    IntuitionEngine, IntuitionConfig,
    ExperienceStream,
    InMemoryADNAReader, AppraiserConfig,
};
use std::sync::Arc;
use tokio::sync::mpsc;

// For advanced users who need full control
let (proposal_tx, proposal_rx) = mpsc::channel(500);
let experience = Arc::new(ExperienceStream::new(20_000, 2_000));
let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig {
    homeostasis: custom_homeostasis_params,
    ..Default::default()
}));

let intuition = IntuitionEngine::builder()
    .with_config(custom_config)
    .with_experience(experience)
    .with_adna_reader(adna)
    .with_proposal_sender(proposal_tx)
    .build()
    .unwrap();

// Now you have access to proposal_rx for custom processing
tokio::spawn(async move {
    while let Some(proposal) = proposal_rx.recv().await {
        // Custom proposal handling
    }
});
```

---

## Comparison Table

| Feature | Old API (v0.39.1) | New API (v0.39.2) |
|---------|-------------------|-------------------|
| **Minimum lines for default setup** | 8 lines | 1 line |
| **Must understand mpsc channels** | ✅ Yes | ❌ No (optional) |
| **Must understand Arc** | ✅ Yes | ❌ No (optional) |
| **Must know dependency constructors** | ✅ Yes | ❌ No |
| **Type-safe** | ✅ Yes | ✅ Yes |
| **Flexible for advanced users** | ✅ Yes | ✅ Yes (even better) |
| **Easy to test** | ⚠️ Medium | ✅ Easy |
| **Beginner-friendly** | ❌ No | ✅ Yes |

---

## Migration Guide

### If you're using the old API (v0.39.1 or earlier)

**Your existing code still works!** The old `new()` method is unchanged.

```rust
// OLD CODE - Still works in v0.39.2+
let (tx, _rx) = mpsc::channel(100);
let intuition = IntuitionEngine::new(
    config,
    experience,
    adna,
    tx,
);
```

**But you can simplify it to:**

```rust
// NEW CODE - Much simpler
let intuition = IntuitionEngine::builder()
    .with_config(config)
    .with_experience(experience)
    .with_adna_reader(adna)
    .build()
    .unwrap();
```

**Or even simpler if using defaults:**

```rust
// SIMPLEST - Just use defaults
let intuition = IntuitionEngine::with_defaults();
```

---

## Real-World Example: REST API Server

### Before (v0.39.1)

```rust
// api.rs - Complex setup
use std::sync::Arc;
use tokio::sync::mpsc;
use neurograph::*;

#[tokio::main]
async fn main() {
    // Create all dependencies manually
    let (proposal_tx, mut proposal_rx) = mpsc::channel(100);
    let experience = Arc::new(ExperienceStream::new(10_000, 1_000));
    let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()));

    // Create IntuitionEngine
    let intuition = IntuitionEngine::new(
        IntuitionConfig::default(),
        experience.clone(),
        adna.clone(),
        proposal_tx,
    );

    // Spawn background task
    tokio::spawn(async move {
        intuition.run().await;
    });

    // Start API server with experience and adna
    start_api_server(experience, adna).await;
}
```

---

### After (v0.39.2)

```rust
// api.rs - Simple setup
use neurograph::*;

#[tokio::main]
async fn main() {
    // One line to create IntuitionEngine with defaults
    let intuition = IntuitionEngine::with_defaults();

    // Spawn background task
    tokio::spawn(async move {
        intuition.run().await;
    });

    // Start API server (components created internally)
    start_api_server_simple().await;
}
```

---

## Testing Example

### Before (v0.39.1)

```rust
#[tokio::test]
async fn test_intuition_pattern_detection() {
    use std::sync::Arc;
    use tokio::sync::mpsc;

    // Setup boilerplate
    let (tx, _rx) = mpsc::channel(100);
    let experience = Arc::new(ExperienceStream::new(1000, 100));
    let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()));

    let intuition = IntuitionEngine::new(
        IntuitionConfig::default(),
        experience.clone(),
        adna,
        tx,
    );

    // Write test events
    experience.write_event(test_event_1).unwrap();
    experience.write_event(test_event_2).unwrap();

    // Test logic...
}
```

---

### After (v0.39.2)

```rust
#[tokio::test]
async fn test_intuition_pattern_detection() {
    // One line setup!
    let intuition = IntuitionEngine::with_defaults();

    // Test logic...
}
```

---

## Builder Pattern Cheat Sheet

### IntuitionEngine::builder()

| Method | Description | Default |
|--------|-------------|---------|
| `with_config(config)` | Set custom IntuitionConfig | `IntuitionConfig::default()` |
| `with_experience(exp)` | Set shared ExperienceStream | New with capacity 10k |
| `with_adna_reader(adna)` | Set custom ADNA reader | InMemoryADNAReader |
| `with_proposal_sender(tx)` | Set custom proposal channel | Internal channel |
| `with_capacity(n)` | Set experience capacity | 10,000 |
| `with_channel_size(n)` | Set broadcast channel size | 1,000 |
| `build()` | Build IntuitionEngine | Returns `Result<IntuitionEngine, String>` |

### IntuitionEngine::with_defaults()

Creates IntuitionEngine with all defaults in one call.
Equivalent to `IntuitionEngine::builder().build().unwrap()`.

---

## Performance Notes

**Q: Does the builder add overhead?**

**A: No.** The builder is zero-cost abstraction:
- Builder methods are simple setters (inlined by compiler)
- `build()` calls the same `new()` method as before
- No runtime overhead compared to manual construction

**Q: Should I use the builder for production?**

**A: Yes!** The builder is recommended for all use cases:
- Easier to maintain
- Easier to test
- Same performance as manual construction
- More flexible for future changes

---

## Best Practices

1. **Use `with_defaults()` for prototyping**
   ```rust
   let intuition = IntuitionEngine::with_defaults();
   ```

2. **Use builder for production with custom config**
   ```rust
   let intuition = IntuitionEngine::builder()
       .with_config(production_config)
       .with_capacity(100_000)
       .build()
       .unwrap();
   ```

3. **Use builder with shared components for complex systems**
   ```rust
   let shared_exp = Arc::new(ExperienceStream::new(50_000, 5_000));

   let intuition1 = IntuitionEngine::builder()
       .with_experience(shared_exp.clone())
       .build()
       .unwrap();
   ```

4. **Keep `new()` for custom dependency injection**
   ```rust
   // When you need full control over proposal channel
   let (tx, rx) = mpsc::channel(100);
   let intuition = IntuitionEngine::new(config, exp, adna, tx);
   // Now you can use rx for custom logic
   ```

---

## Next Steps

This builder pattern is being rolled out to other complex components:
- ✅ **IntuitionEngine** - Implemented in v0.39.2
- 🚧 **ActionController** - Planned for v0.39.2
- 🚧 **FeedbackProcessor** - Planned for v0.39.2
- 🚧 **SystemBuilder** - Full system setup - Planned for v0.40.0

---

**Maintainer:** Chernov Denys
**Contributors:** Claude Code (Anthropic)
**License:** AGPL-3.0
