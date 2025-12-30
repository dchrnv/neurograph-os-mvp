use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::module_id::ModuleId;
use crate::module_registry::{ModuleConfig, REGISTRY};

/// Конвертация ModuleId из строки
fn parse_module_id(id: &str) -> PyResult<ModuleId> {
    match id {
        "token_manager" => Ok(ModuleId::TokenManager),
        "connection_manager" => Ok(ModuleId::ConnectionManager),
        "grid" => Ok(ModuleId::Grid),
        "intuition_engine" => Ok(ModuleId::IntuitionEngine),
        "signal_system" => Ok(ModuleId::SignalSystem),
        "gateway" => Ok(ModuleId::Gateway),
        "action_controller" => Ok(ModuleId::ActionController),
        "guardian" => Ok(ModuleId::Guardian),
        "cdna" => Ok(ModuleId::Cdna),
        "bootstrap" => Ok(ModuleId::Bootstrap),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unknown module id: {}", id)
        )),
    }
}

/// Получить список всех модулей
#[pyfunction]
pub fn list_modules(py: Python<'_>) -> PyResult<Vec<PyObject>> {
    let modules = REGISTRY.get_all_modules();

    modules
        .into_iter()
        .map(|info| {
            let dict = PyDict::new(py);
            dict.set_item("id", format!("{:?}", info.id).to_lowercase())?;
            dict.set_item("name", info.name)?;
            dict.set_item("description", info.description)?;
            dict.set_item("version", info.version)?;
            dict.set_item("status", format!("{:?}", info.status).to_lowercase())?;
            dict.set_item("enabled", info.enabled)?;
            dict.set_item("can_disable", info.can_disable)?;
            dict.set_item("configurable", info.configurable)?;
            dict.set_item("disable_warning", info.disable_warning)?;

            // Метрики
            let metrics = PyDict::new(py);
            metrics.set_item("operations", info.metrics.operations)?;
            metrics.set_item("ops_per_sec", info.metrics.ops_per_sec)?;
            metrics.set_item("avg_latency_us", info.metrics.avg_latency_us)?;
            metrics.set_item("p95_latency_us", info.metrics.p95_latency_us)?;
            metrics.set_item("errors", info.metrics.errors)?;
            dict.set_item("metrics", metrics)?;

            Ok(dict.into())
        })
        .collect()
}

/// Получить информацию о модуле
#[pyfunction]
pub fn get_module(py: Python<'_>, module_id: &str) -> PyResult<PyObject> {
    let id = parse_module_id(module_id)?;
    let info = REGISTRY.get_module_info(id);

    let dict = PyDict::new(py);
    dict.set_item("id", module_id)?;
    dict.set_item("name", info.name)?;
    dict.set_item("description", info.description)?;
    dict.set_item("version", info.version)?;
    dict.set_item("status", format!("{:?}", info.status).to_lowercase())?;
    dict.set_item("enabled", info.enabled)?;
    dict.set_item("can_disable", info.can_disable)?;
    dict.set_item("configurable", info.configurable)?;
    dict.set_item("disable_warning", info.disable_warning)?;

    // Метрики
    let metrics = PyDict::new(py);
    metrics.set_item("operations", info.metrics.operations)?;
    metrics.set_item("ops_per_sec", info.metrics.ops_per_sec)?;
    metrics.set_item("avg_latency_us", info.metrics.avg_latency_us)?;
    metrics.set_item("p95_latency_us", info.metrics.p95_latency_us)?;
    metrics.set_item("errors", info.metrics.errors)?;
    dict.set_item("metrics", metrics)?;

    Ok(dict.into())
}

/// Проверить, включен ли модуль
#[pyfunction]
pub fn is_module_enabled(module_id: &str) -> PyResult<bool> {
    let id = parse_module_id(module_id)?;
    Ok(REGISTRY.is_enabled(id))
}

/// Включить/выключить модуль
#[pyfunction]
pub fn set_module_enabled(module_id: &str, enabled: bool) -> PyResult<()> {
    let id = parse_module_id(module_id)?;
    REGISTRY
        .set_enabled(id, enabled)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

/// Получить конфигурацию модуля
#[pyfunction]
pub fn get_module_config(py: Python<'_>, module_id: &str) -> PyResult<Option<PyObject>> {
    let id = parse_module_id(module_id)?;

    match REGISTRY.get_config(id) {
        Some(config) => {
            let dict = PyDict::new(py);
            for (key, value) in config.values {
                // Конвертируем serde_json::Value в Python объект
                let py_value = json_to_py(py, &value)?;
                dict.set_item(key, py_value)?;
            }
            Ok(Some(dict.into()))
        }
        None => Ok(None),
    }
}

/// Обновить конфигурацию модуля
#[pyfunction]
pub fn set_module_config(module_id: &str, config: &Bound<'_, PyDict>) -> PyResult<()> {
    let id = parse_module_id(module_id)?;

    let mut values = std::collections::HashMap::new();
    for (key, value) in config.iter() {
        let key_str: String = key.extract()?;
        let json_value = py_to_json(&value)?;
        values.insert(key_str, json_value);
    }

    let module_config = ModuleConfig { values };

    REGISTRY
        .set_config(id, module_config)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

// Вспомогательные функции для конвертации JSON <-> Python
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let list: Vec<PyObject> = arr
                .iter()
                .map(|v| json_to_py(py, v))
                .collect::<PyResult<_>>()?;
            Ok(list.into_py(py))
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new_bound(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}

fn py_to_json(obj: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(serde_json::Value::Number(i.into()))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(serde_json::json!(f))
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = obj.extract::<Vec<Bound<'_, PyAny>>>() {
        let arr: Result<Vec<_>, _> = list.iter().map(|v| py_to_json(v)).collect();
        Ok(serde_json::Value::Array(arr?))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            map.insert(key, py_to_json(&v)?);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported type for JSON conversion"
        ))
    }
}

/// Регистрация модуля в PyO3
pub fn register_module(py: Python<'_>, parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let module = PyModule::new_bound(py, "modules")?;
    module.add_function(wrap_pyfunction!(list_modules, &module)?)?;
    module.add_function(wrap_pyfunction!(get_module, &module)?)?;
    module.add_function(wrap_pyfunction!(is_module_enabled, &module)?)?;
    module.add_function(wrap_pyfunction!(set_module_enabled, &module)?)?;
    module.add_function(wrap_pyfunction!(get_module_config, &module)?)?;
    module.add_function(wrap_pyfunction!(set_module_config, &module)?)?;
    parent.add_submodule(&module)?;
    Ok(())
}
