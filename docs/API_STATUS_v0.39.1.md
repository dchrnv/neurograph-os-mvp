# NeuroGraph OS - API Status & Compatibility Report

**Version:** v0.39.1
**Date:** 2025-01-28
**Status:** 🟡 **API Stabilization Needed**

---

## 📊 Executive Summary

The NeuroGraph OS API has evolved significantly through v0.35-v0.39, introducing powerful new features but also **breaking changes** in component initialization. While the core architecture is sound, **ease of use has decreased** due to increased constructor complexity.

**Key Issues:**
1. ❌ **Complex constructors** - Components require many dependencies
2. ❌ **No builder patterns** - Hard to create components correctly
3. ❌ **Breaking changes** not documented
4. ✅ **Public APIs** properly exported
5. ✅ **Type safety** maintained

**Recommendation:** Add builder patterns and convenience constructors before v1.0.0

---

## 🔍 Current API State

### ✅ Gateway (v1.0) - GOOD API

**Constructor:**
```rust
pub fn new(
    sender: mpsc::Sender<ProcessedSignal>,
    bootstrap: Arc<RwLock<BootstrapLibrary>>,
    config: GatewayConfig,
) -> Self
```

**Status:** ✅ **Well-designed**
- Clear dependencies
- Configuration separated
- Easy to test

**Example:**
```rust
let (tx, rx) = mpsc::channel(1000);
let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
let gateway = Gateway::new(tx, bootstrap, GatewayConfig::default());
```

---

### ⚠️ ExperienceStream (v2.1) - CHANGED API

**Constructor:**
```rust
pub fn new(capacity: usize, channel_size: usize) -> Self
```

**Breaking Change:** Now requires 2 arguments (was 1 in older versions)

**Status:** ⚠️ **Changed but reasonable**

**Example:**
```rust
let experience = ExperienceStream::new(
    10_000,  // capacity (hot buffer)
    1_000,   // channel size (broadcast)
);
```

**Migration:**
```rust
// OLD (v0.30)
let experience = ExperienceStream::new(10_000);

// NEW (v0.39.1)
let experience = ExperienceStream::new(10_000, 1_000);
```

---

### ❌ IntuitionEngine (v3.0) - COMPLEX API

**Constructor:**
```rust
pub fn new(
    config: IntuitionConfig,
    experience_stream: Arc<ExperienceStream>,
    dna_reader: Arc<dyn ADNAReader>,
    proposal_sender: mpsc::Sender<Proposal>,
) -> Self
```

**Breaking Change:** Requires 4 arguments, complex dependencies

**Status:** ❌ **Too complex for users**

**Problems:**
1. Requires `Arc<ExperienceStream>` - user must manage sharing
2. Requires `Arc<dyn ADNAReader>` - must implement trait
3. Requires `mpsc::Sender<Proposal>` - must set up channel
4. No simple default constructor

**Example (current):**
```rust
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

**Proposed:**
```rust
// Simple constructor for common case
let mut intuition = IntuitionEngine::builder()
    .with_capacity(10_000)
    .with_default_adna()
    .build();

// Or with custom components
let intuition = IntuitionEngine::builder()
    .with_experience(experience)
    .with_adna_reader(custom_adna)
    .with_proposal_channel(tx)
    .build();
```

---

### ❌ Guardian - INCONSISTENT API

**Constructor:**
```rust
pub fn new() -> Self  // Creates with default CDNA

pub fn with_cdna(cdna: CDNA) -> Self  // Custom CDNA
```

**Status:** ⚠️ **Inconsistent**

**Problem:** `new()` doesn't take CDNA, but `with_cdna()` does

**Current Usage:**
```rust
// Default
let guardian = Guardian::new();

// Custom CDNA
let guardian = Guardian::with_cdna(custom_cdna);
```

**Proposed:**
```rust
// Keep both, but make it clear
let guardian = Guardian::default();  // Or Guardian::new()
let guardian = Guardian::with_cdna(custom_cdna);
```

---

### ⚠️ ActionController (v2.0) - COMPLEX BUT ACCEPTABLE

**Constructor:**
```rust
pub fn new(
    adna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    intuition: Arc<RwLock<IntuitionEngine>>,
    guardian: Arc<Guardian>,
    config: ActionControllerConfig,
    arbiter_config: ArbiterConfig,
) -> Self
```

**Status:** ⚠️ **Complex but necessary**

**Justification:** ActionController is a high-level coordinator, complexity is expected.

**Example:**
```rust
let controller = ActionController::new(
    adna,
    experience,
    intuition,
    guardian,
    ActionControllerConfig::default(),
    ArbiterConfig::default(),
);
```

**Could be improved with builder:**
```rust
let controller = ActionController::builder()
    .with_adna(adna)
    .with_experience(experience)
    .with_intuition(intuition)
    .with_guardian(guardian)
    .build();
```

---

### ✅ Bootstrap Library (v1.3) - EXCELLENT API

**Constructor:**
```rust
pub fn new(config: BootstrapConfig) -> Self
```

**Status:** ✅ **Perfect**

**Example:**
```rust
let bootstrap = BootstrapLibrary::new(BootstrapConfig::default());

// Or with custom config
let config = BootstrapConfig {
    enable_multimodal: true,
    pca_dimensions: 3,
    ..Default::default()
};
let bootstrap = BootstrapLibrary::new(config);
```

---

### ✅ CuriosityDrive (v1.0) - GOOD API

**Constructor:**
```rust
pub fn new(config: CuriosityConfig) -> Self
```

**Status:** ✅ **Simple and clear**

**Example:**
```rust
let curiosity = CuriosityDrive::new(CuriosityConfig::default());
```

---

### ⚠️ FeedbackProcessor (v1.0) - ACCEPTABLE

**Constructor:**
```rust
pub fn new(
    bootstrap: Arc<RwLock<BootstrapLibrary>>,
    experience_stream: Arc<RwLock<ExperienceStream>>,
    intuition_engine: Arc<RwLock<IntuitionEngine>>,
) -> Self
```

**Status:** ⚠️ **Requires Arc<RwLock<>> wrappers**

**Example:**
```rust
let processor = FeedbackProcessor::new(
    Arc::new(RwLock::new(bootstrap)),
    Arc::new(RwLock::new(experience)),
    Arc::new(RwLock::new(intuition)),
);
```

---

## 🔧 Breaking Changes History

### v0.30 → v0.39.1

| Component | Old API | New API | Breaking? |
|-----------|---------|---------|-----------|
| ExperienceStream | `new(capacity)` | `new(capacity, channel_size)` | ✅ YES |
| IntuitionEngine | `new(config)` | `new(config, exp, adna, tx)` | ✅ YES |
| Guardian | `new(cdna)` | `new()` | ✅ YES |
| ActionController | Similar | Added `arbiter_config` | ⚠️ MINOR |
| Bootstrap | Unchanged | Unchanged | ❌ NO |
| Curiosity | Unchanged | Unchanged | ❌ NO |

---

## 📋 Proposed Solutions

### 1. Add Builder Patterns (Recommended)

**Benefits:**
- ✅ Clear API
- ✅ Optional parameters
- ✅ Type-safe
- ✅ Backward compatible (keep `new()`)

**Example Implementation:**

```rust
// IntuitionEngine Builder
pub struct IntuitionEngineBuilder {
    config: IntuitionConfig,
    experience: Option<Arc<ExperienceStream>>,
    adna: Option<Arc<dyn ADNAReader>>,
    proposal_sender: Option<mpsc::Sender<Proposal>>,
}

impl IntuitionEngineBuilder {
    pub fn new() -> Self {
        Self {
            config: IntuitionConfig::default(),
            experience: None,
            adna: None,
            proposal_sender: None,
        }
    }

    pub fn with_config(mut self, config: IntuitionConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_experience(mut self, exp: Arc<ExperienceStream>) -> Self {
        self.experience = Some(exp);
        self
    }

    pub fn build(self) -> Result<IntuitionEngine, BuildError> {
        // Create defaults if not provided
        let experience = self.experience.unwrap_or_else(|| {
            Arc::new(ExperienceStream::new(10_000, 1_000))
        });

        let adna = self.adna.unwrap_or_else(|| {
            Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()))
        });

        let (tx, _rx) = mpsc::channel(100);
        let proposal_sender = self.proposal_sender.unwrap_or(tx);

        Ok(IntuitionEngine::new(
            self.config,
            experience,
            adna,
            proposal_sender,
        ))
    }
}

impl IntuitionEngine {
    pub fn builder() -> IntuitionEngineBuilder {
        IntuitionEngineBuilder::new()
    }
}
```

**Usage:**
```rust
// Simple case
let intuition = IntuitionEngine::builder().build()?;

// Custom config
let intuition = IntuitionEngine::builder()
    .with_config(custom_config)
    .with_experience(shared_experience)
    .build()?;
```

---

### 2. Add Convenience Constructors

**Benefits:**
- ✅ Quick to implement
- ✅ Easy to use for common cases
- ⚠️ Still have complex `new()` for advanced users

**Example:**

```rust
impl IntuitionEngine {
    /// Create with defaults (convenience)
    pub fn with_defaults() -> Self {
        let (tx, _rx) = mpsc::channel(100);
        Self::new(
            IntuitionConfig::default(),
            Arc::new(ExperienceStream::new(10_000, 1_000)),
            Arc::new(InMemoryADNAReader::new(AppraiserConfig::default())),
            tx,
        )
    }

    /// Create with custom config only
    pub fn with_config(config: IntuitionConfig) -> Self {
        let (tx, _rx) = mpsc::channel(100);
        Self::new(
            config,
            Arc::new(ExperienceStream::new(10_000, 1_000)),
            Arc::new(InMemoryADNAReader::new(AppraiserConfig::default())),
            tx,
        )
    }
}
```

---

### 3. Add "SystemBuilder" (Full System Setup)

**For complex integration scenarios:**

```rust
pub struct NeuroGraphSystemBuilder {
    bootstrap_config: BootstrapConfig,
    gateway_config: GatewayConfig,
    controller_config: ActionControllerConfig,
    // ... etc
}

impl NeuroGraphSystemBuilder {
    pub fn build(self) -> Result<NeuroGraphSystem, BuildError> {
        // Create all components in correct order
        // Return a struct with all components
    }
}

pub struct NeuroGraphSystem {
    pub bootstrap: Arc<RwLock<BootstrapLibrary>>,
    pub gateway: Arc<Gateway>,
    pub controller: Arc<ActionController>,
    pub curiosity: Arc<CuriosityDrive>,
    pub feedback: Arc<FeedbackProcessor>,
}

// Usage
let system = NeuroGraphSystemBuilder::new()
    .with_bootstrap_config(custom_config)
    .build()?;

// Now all components are ready and wired together
let result = system.gateway.inject(signal).await?;
```

---

## 🎯 Recommendations for v0.40.0 (Python Bindings)

**Before implementing Python bindings, we should:**

1. ✅ **Keep current `new()` methods** - Don't break existing code
2. ✅ **Add builder patterns** for IntuitionEngine, ActionController
3. ✅ **Add `with_defaults()` convenience constructors**
4. ✅ **Add `SystemBuilder`** for full system setup
5. ✅ **Document all breaking changes** since v0.30

**Why?**
- Python users will need **simple APIs**
- PyO3 bindings are easier with simple constructors
- Current API complexity will be magnified in Python

---

## 📝 Migration Guide Template

### From v0.30 to v0.39.1

**ExperienceStream:**
```rust
// Before
let exp = ExperienceStream::new(10_000);

// After
let exp = ExperienceStream::new(10_000, 1_000);
```

**IntuitionEngine:**
```rust
// Before
let intuition = IntuitionEngine::new(config);

// After (complex)
let (tx, _rx) = mpsc::channel(100);
let intuition = IntuitionEngine::new(
    config,
    Arc::new(experience),
    Arc::new(adna),
    tx,
);

// After (with convenience - if added)
let intuition = IntuitionEngine::with_defaults();
```

**Guardian:**
```rust
// Before
let guardian = Guardian::new(cdna);

// After
let guardian = Guardian::with_cdna(cdna);  // Or Guardian::new() for default
```

---

## 🚀 Action Items for v1.0.0

### High Priority (Before v0.40.0 Python Bindings)

- [ ] Add `IntuitionEngine::builder()`
- [ ] Add `ActionController::builder()`
- [ ] Add `with_defaults()` to all complex components
- [ ] Document all breaking changes

### Medium Priority (Before v1.0.0)

- [ ] Add `NeuroGraphSystemBuilder`
- [ ] Create comprehensive examples
- [ ] Write migration guide
- [ ] Add API stability guarantees

### Low Priority (Post v1.0.0)

- [ ] Deprecate old complex constructors
- [ ] Add proc macros for automatic builder generation
- [ ] Add typed builders with compile-time validation

---

## ✅ Conclusion

**Current Status:** API is **functional but not user-friendly**

**Root Cause:** Architecture matured faster than API design

**Solution:** Add **builder patterns** and **convenience constructors** without breaking existing code

**Timeline:**
- v0.39.2 (optional patch): Add builders
- v0.40.0: Python bindings use new builders
- v1.0.0: API stability guarantee

---

**Maintainer:** Chernov Denys
**Contributors:** Claude Code (Anthropic)
**License:** AGPL-3.0
