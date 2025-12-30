from typing import Optional, Dict, Any, List
from neurograph import _core  # PyO3 bindings

from ..models.modules import (
    ModuleInfo,
    ModuleMetrics,
    ModuleStatus,
    ModuleConfig,
)


class ModuleService:
    """Сервис для работы с модулями"""

    def __init__(self):
        pass

    def list_modules(self) -> List[ModuleInfo]:
        """Получить список всех модулей"""
        raw_modules = _core.modules.list_modules()
        return [self._convert_module_info(m) for m in raw_modules]

    def get_module(self, module_id: str) -> Optional[ModuleInfo]:
        """Получить информацию о модуле"""
        try:
            raw = _core.modules.get_module(module_id)
            return self._convert_module_info(raw)
        except ValueError:
            return None

    def is_enabled(self, module_id: str) -> bool:
        """Проверить, включен ли модуль"""
        return _core.modules.is_module_enabled(module_id)

    def set_enabled(self, module_id: str, enabled: bool) -> None:
        """Включить/выключить модуль"""
        _core.modules.set_module_enabled(module_id, enabled)

    def get_config(self, module_id: str) -> Optional[Dict[str, Any]]:
        """Получить конфигурацию модуля"""
        return _core.modules.get_module_config(module_id)

    def set_config(self, module_id: str, config: Dict[str, Any]) -> None:
        """Обновить конфигурацию модуля"""
        _core.modules.set_module_config(module_id, config)

    def _convert_module_info(self, raw: dict) -> ModuleInfo:
        """Конвертация из dict в Pydantic модель"""
        metrics_raw = raw.get("metrics", {})
        metrics = ModuleMetrics(
            operations=metrics_raw.get("operations", 0),
            ops_per_sec=metrics_raw.get("ops_per_sec", 0.0),
            avg_latency_us=metrics_raw.get("avg_latency_us", 0.0),
            p95_latency_us=metrics_raw.get("p95_latency_us", 0.0),
            errors=metrics_raw.get("errors", 0),
            custom=metrics_raw.get("custom", {}),
        )

        return ModuleInfo(
            id=raw["id"],
            name=raw["name"],
            description=raw["description"],
            version=raw["version"],
            status=ModuleStatus(raw["status"]),
            enabled=raw["enabled"],
            can_disable=raw["can_disable"],
            configurable=raw["configurable"],
            disable_warning=raw.get("disable_warning"),
            metrics=metrics,
        )


# Singleton instance
module_service = ModuleService()
