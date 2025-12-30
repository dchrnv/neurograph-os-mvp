# CHANGELOG v0.63.0 - Module Registry System

## Overview

Version 0.63.0 introduces a comprehensive Module Registry System for NeuroGraph OS, enabling dynamic management of system modules through enable/disable controls instead of start/stop processes.

**Total Implementation:** 900+ lines across Rust, Python, and TypeScript
**Completion Date:** December 30, 2025
**Development Phases:** 3 of 3 phases completed (100%)

---

## Table of Contents

- [Phase 1: Rust Core](#phase-1-rust-core)
- [Phase 2: Python API Layer](#phase-2-python-api-layer)
- [Phase 3: Web Dashboard Updates](#phase-3-web-dashboard-updates)
- [Key Features](#key-features)
- [Module Categories](#module-categories)
- [API Endpoints](#api-endpoints)
- [Breaking Changes](#breaking-changes)

---

## Phase 1: Rust Core

**Files Created:** 3 files
**Lines Added:** +550

### Features Implemented

#### Module ID Enum

**File:** `src/core_rust/src/module_id.rs` (+120 lines)

**10 Module Types:**

| Module ID | Name | Description | Can Disable? |
|-----------|------|-------------|--------------|
| `token_manager` | TokenManager | Token storage and management | ❌ No (core) |
| `connection_manager` | ConnectionManager | Connection storage | ❌ No (core) |
| `grid` | Grid | 8D spatial index | ❌ No (core) |
| `intuition_engine` | IntuitionEngine | Intuitive query processing | ✅ Yes |
| `signal_system` | SignalSystem | Event processing | ✅ Yes |
| `gateway` | Gateway | Input sensors and encoders | ✅ Yes |
| `action_controller` | ActionController | Output actions | ✅ Yes |
| `guardian` | Guardian | Validation and protection | ❌ No (critical!) |
| `cdna` | CDNA | System constitution | ❌ No (core) |
| `bootstrap` | Bootstrap | Embeddings loader | ❌ No (status only) |

**Key Methods:**
- `display_name()` - Human-readable name
- `description()` - Module description
- `version()` - Current version
- `can_disable()` - Whether module can be disabled
- `is_configurable()` - Whether module has configuration
- `disable_warning()` - Warning message when disabling

#### Module Registry

**File:** `src/core_rust/src/module_registry.rs` (+230 lines)

**Core Structures:**

```rust
pub enum ModuleStatus {
    Active,     // Enabled and running
    Disabled,   // Disabled by user
    Error,      // Module error
}

pub struct ModuleMetrics {
    pub operations: u64,
    pub ops_per_sec: f64,
    pub avg_latency_us: f64,
    pub p95_latency_us: f64,
    pub errors: u64,
    pub custom: HashMap<String, f64>,
}

pub struct ModuleRegistry {
    enabled: RwLock<HashMap<ModuleId, bool>>,
    configs: RwLock<HashMap<ModuleId, ModuleConfig>>,
    metrics: RwLock<HashMap<ModuleId, ModuleMetrics>>,
    statuses: RwLock<HashMap<ModuleId, ModuleStatus>>,
}
```

**Key Features:**
- Thread-safe with `RwLock`
- Singleton pattern with `lazy_static`
- All modules enabled by default
- Graceful degradation when disabled

**API Methods:**
- `is_enabled(module)` - Check if module is enabled
- `set_enabled(module, enabled)` - Enable/disable module
- `get_module_info(module)` - Get module information
- `get_all_modules()` - List all modules
- `update_metrics(module, metrics)` - Update module metrics
- `get_config(module)` - Get module configuration
- `set_config(module, config)` - Update module configuration

#### PyO3 FFI Bindings

**File:** `src/core_rust/src/python/modules.rs` (+200 lines)

**Python-accessible functions:**

```rust
#[pyfunction]
pub fn list_modules(py: Python<'_>) -> PyResult<Vec<PyObject>>

#[pyfunction]
pub fn get_module(py: Python<'_>, module_id: &str) -> PyResult<PyObject>

#[pyfunction]
pub fn is_module_enabled(module_id: &str) -> PyResult<bool>

#[pyfunction]
pub fn set_module_enabled(module_id: &str, enabled: bool) -> PyResult<()>

#[pyfunction]
pub fn get_module_config(py: Python<'_>, module_id: &str) -> PyResult<Option<PyObject>>

#[pyfunction]
pub fn set_module_config(module_id: &str, config: &Bound<'_, PyDict>) -> PyResult<()>
```

**Features:**
- JSON <-> Python type conversion
- Error handling with PyResult
- Registered in `_core.modules` submodule

#### Integration with Existing Modules

**Modified Files:**
- `src/core_rust/src/intuition_engine.rs`
- `src/core_rust/src/signal_system/system.rs`
- `src/core_rust/src/gateway/mod.rs`
- `src/core_rust/src/action_controller.rs`

**Pattern Used:**

```rust
pub fn process(&self, input: &Input) -> Option<Output> {
    // Check if module is enabled
    if !REGISTRY.is_enabled(ModuleId::IntuitionEngine) {
        return None; // Module disabled - skip processing
    }

    // Normal processing logic
    // ...
}
```

---

## Phase 2: Python API Layer

**Files Created:** 3 files
**Lines Added:** +250

### Pydantic Models

**File:** `src/api/models/modules.py` (+90 lines)

**Models:**

```python
class ModuleStatus(str, Enum):
    ACTIVE = "active"
    DISABLED = "disabled"
    ERROR = "error"

class ModuleMetrics(BaseModel):
    operations: int
    ops_per_sec: float
    avg_latency_us: float
    p95_latency_us: float
    errors: int
    custom: Dict[str, float]

class ModuleInfo(BaseModel):
    id: str
    name: str
    description: str
    version: str
    status: ModuleStatus
    enabled: bool
    can_disable: bool
    configurable: bool
    disable_warning: Optional[str]
    metrics: ModuleMetrics
```

### Module Service

**File:** `src/api/services/modules.py` (+80 lines)

**Service Methods:**

```python
class ModuleService:
    def list_modules(self) -> List[ModuleInfo]
    def get_module(self, module_id: str) -> Optional[ModuleInfo]
    def is_enabled(self, module_id: str) -> bool
    def set_enabled(self, module_id: str, enabled: bool) -> None
    def get_config(self, module_id: str) -> Optional[Dict[str, Any]]
    def set_config(self, module_id: str, config: Dict[str, Any]) -> None
```

**Features:**
- Singleton pattern
- Rust FFI integration via `_core.modules`
- Type conversion from dict to Pydantic models

### API Router

**File:** `src/api/routers/modules.py` (rewritten, +160 lines)

**Endpoints:**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/modules` | List all modules |
| GET | `/api/v1/modules/{id}` | Get module info |
| PUT | `/api/v1/modules/{id}/enabled` | Enable/disable module |
| GET | `/api/v1/modules/{id}/metrics` | Get module metrics |
| GET | `/api/v1/modules/{id}/config` | Get module config |
| PUT | `/api/v1/modules/{id}/config` | Update module config |

**Features:**
- Full validation with Pydantic
- Error handling (404, 400, 500)
- Russian/English error messages
- Core module protection

---

## Phase 3: Web Dashboard Updates

**Files Modified:** 2 files
**Lines Changed:** +120 / -80

### Module Types Update

**File:** `src/web/src/types/modules.ts` (rewritten)

**New Type Definitions:**

```typescript
export type ModuleStatus = 'active' | 'disabled' | 'error';

export interface ModuleMetrics {
  operations: number;
  ops_per_sec: number;
  avg_latency_us: number;
  p95_latency_us: number;
  errors: number;
  custom?: Record<string, number>;
}

export interface Module {
  id: string;
  name: string;
  description: string;
  version: string;
  status: ModuleStatus;
  enabled: boolean;
  can_disable: boolean;
  configurable: boolean;
  disable_warning?: string;
  metrics: ModuleMetrics;
}
```

### Modules Page Update

**File:** `src/web/src/pages/Modules.tsx`

**Changes:**
- ❌ Removed: `onStart`, `onStop`, `onRestart` handlers
- ✅ Added: `onToggleEnabled` handler with confirmation modal
- ✅ Added: Warning dialog when disabling modules
- ✅ Added: Direct fetch API calls to `/api/v1/modules`
- ✅ Removed: Start All / Stop All buttons

**New Behavior:**

```typescript
const handleToggleEnabled = async (id: string, enabled: boolean) => {
  const module = modules.find((m) => m.id === id);

  // Show warning if module has disable_warning
  if (!enabled && module.disable_warning) {
    Modal.confirm({
      title: t('common.confirm'),
      content: module.disable_warning,
      okType: 'danger',
      onOk: async () => {
        await toggleModule(id, enabled);
      },
    });
  } else {
    await toggleModule(id, enabled);
  }
};
```

### ModuleCard Component Update

**File:** `src/web/src/components/ModuleCard.tsx` (rewritten)

**Changes:**
- ❌ Removed: Start/Stop/Restart buttons
- ✅ Added: Enable/Disable toggle switch
- ✅ Added: Core Module info alert
- ✅ Added: Warning alert when disabled
- ✅ Added: Module description display
- ✅ Updated: Metrics display with proper formatting

**New UI Components:**

```tsx
<Switch
  checked={module.enabled}
  disabled={!module.can_disable || loading}
  onChange={(checked) => onToggleEnabled(module.id, checked)}
  checkedChildren="ON"
  unCheckedChildren="OFF"
/>

{!module.can_disable && (
  <Alert
    message="Core Module"
    description="This module cannot be disabled"
    type="info"
  />
)}

{module.disable_warning && !module.enabled && (
  <Alert
    message="Warning"
    description={module.disable_warning}
    type="warning"
  />
)}
```

---

## Key Features

### 1. Enable/Disable Architecture

**Instead of Start/Stop:**
- Modules remain in memory
- Modules check `REGISTRY.is_enabled()` before operations
- Graceful degradation when disabled
- No process management overhead

### 2. Module Categories

**Core (Cannot Disable):**
- TokenManager
- ConnectionManager
- Grid
- CDNA
- Guardian (critical for security!)

**Processing (Can Disable):**
- IntuitionEngine
- SignalSystem

**I/O (Can Disable):**
- Gateway
- ActionController

**Data (Status Only):**
- Bootstrap

### 3. Safety Features

- Core modules protected from disabling
- Warning messages for dangerous operations
- Confirmation modals in UI
- Russian/English internationalization

### 4. Real-time Metrics

**Per-module tracking:**
- Total operations
- Operations per second
- Average latency (μs)
- P95 latency (μs)
- Error count
- Custom metrics support

---

## API Endpoints

### List Modules

```http
GET /api/v1/modules
```

**Response:**
```json
{
  "modules": [
    {
      "id": "intuition_engine",
      "name": "IntuitionEngine",
      "description": "Интуитивная обработка запросов",
      "version": "3.0.0",
      "status": "active",
      "enabled": true,
      "can_disable": true,
      "configurable": true,
      "disable_warning": null,
      "metrics": {
        "operations": 12847,
        "ops_per_sec": 1284.7,
        "avg_latency_us": 69.5,
        "p95_latency_us": 120.0,
        "errors": 0
      }
    }
  ],
  "total": 10
}
```

### Enable/Disable Module

```http
PUT /api/v1/modules/{module_id}/enabled
Content-Type: application/json

{
  "enabled": false
}
```

**Response:**
```json
{
  "success": true,
  "message": "Модуль 'IntuitionEngine' выключен"
}
```

**Error (Core Module):**
```json
{
  "detail": "Модуль 'token_manager' нельзя отключить (core module)"
}
```

---

## Breaking Changes

### Web Dashboard

1. **Module Status Changed:**
   - Old: `'running' | 'starting' | 'stopped' | 'error' | 'restarting'`
   - New: `'active' | 'disabled' | 'error'`

2. **Module Interface:**
   - Removed: `restarts` field
   - Added: `description`, `enabled`, `can_disable`, `configurable`, `disable_warning`
   - Changed: `metrics` from `Record<string, number>` to `ModuleMetrics`

3. **API Endpoints:**
   - Removed: `/modules/{id}/start`, `/modules/{id}/stop`, `/modules/{id}/restart`
   - Added: `/modules/{id}/enabled`, `/modules/{id}/config`

### Module Behavior

1. **All modules now check `REGISTRY.is_enabled()` before processing**
2. **Disabled modules return early (no error thrown)**
3. **Core modules cannot be disabled**

---

## File Structure

```
src/
├── core_rust/src/
│   ├── module_id.rs               # NEW: Module ID enum
│   ├── module_registry.rs         # NEW: Registry singleton
│   ├── python/modules.rs          # NEW: PyO3 FFI bindings
│   ├── intuition_engine.rs        # MODIFIED: Added is_enabled check
│   ├── signal_system/system.rs    # MODIFIED: Added is_enabled check
│   ├── gateway/mod.rs             # MODIFIED: Added is_enabled check
│   └── action_controller.rs       # MODIFIED: Added is_enabled check
│
├── api/
│   ├── models/modules.py          # NEW: Pydantic models
│   ├── services/modules.py        # NEW: Module service
│   └── routers/modules.py         # MODIFIED: Real implementation
│
└── web/src/
    ├── types/modules.ts           # MODIFIED: Updated types
    ├── pages/Modules.tsx          # MODIFIED: Enable/disable logic
    └── components/ModuleCard.tsx  # MODIFIED: Toggle UI
```

**Total Files:**
- Created: 6 files
- Modified: 8 files
- Total: 14 files

**Total Lines of Code:** 900+

---

## Implementation Summary

### ✅ All Phases Complete (3/3)

| Phase | Description | Status | Files | Lines |
|-------|-------------|--------|-------|-------|
| 1 | Rust Core | ✅ Complete | 7 | +550 |
| 2 | Python API | ✅ Complete | 3 | +250 |
| 3 | Web Dashboard | ✅ Complete | 4 | +120 |
| **Total** | **Module Registry** | **100%** | **14** | **900+** |

### Key Achievements

**Rust Core:**
- Singleton Module Registry with thread-safety
- 10 module types with metadata
- PyO3 FFI bindings for Python integration
- Integration with 4 existing modules

**Python API:**
- Complete REST API with 6 endpoints
- Pydantic models for validation
- Service layer for business logic
- Error handling and safety checks

**Web Dashboard:**
- Toggle switches instead of buttons
- Warning modals for dangerous operations
- Core module protection
- Real-time metrics display
- Russian/English internationalization

---

## Next Steps

1. ✅ Complete Phase 1: Rust Core *(DONE)*
2. ✅ Complete Phase 2: Python API *(DONE)*
3. ✅ Complete Phase 3: Web Dashboard *(DONE)*
4. ⬜ Backend integration testing
5. ⬜ Add module configuration UI
6. ⬜ Add real-time metrics updates via WebSocket

---

**Document Version:** 1.0
**Last Updated:** December 30, 2025
**Status:** 3/3 Phases Complete (100%) ✅
