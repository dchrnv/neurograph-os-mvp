// FFI bridge to neurograph-core
// Direct integration without HTTP overhead
// Based on DESKTOP_UI_SPEC_V3.md section 17

// TODO: Implement CoreBridge with methods:
// - status() → SystemStatus
// - query(text) → QueryResponse
// - execute_command(cmd) → CommandOutput
// - get_modules() → Vec<ModuleInfo>
// - module_action(id, action) → Result
// - get_metrics() → SystemMetrics
// - get_logs(filter) → Vec<LogEntry>
// - get_config() → Config
// - update_config(changes) → Result
// - get_integrations() → Vec<Integration>
//
// Subscriptions:
// - subscribe_metrics(interval) → Stream
// - subscribe_logs(filter) → Stream
// - subscribe_module_status() → Stream
