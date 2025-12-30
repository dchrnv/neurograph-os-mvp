from enum import Enum
from typing import Optional, Dict, Any
from pydantic import BaseModel, Field


class ModuleStatus(str, Enum):
    ACTIVE = "active"
    DISABLED = "disabled"
    ERROR = "error"


class ModuleMetrics(BaseModel):
    """Метрики модуля"""
    operations: int = Field(default=0, description="Количество операций")
    ops_per_sec: float = Field(default=0.0, description="Операций в секунду")
    avg_latency_us: float = Field(default=0.0, description="Средняя задержка (мкс)")
    p95_latency_us: float = Field(default=0.0, description="P95 задержка (мкс)")
    errors: int = Field(default=0, description="Количество ошибок")
    custom: Dict[str, float] = Field(default_factory=dict, description="Дополнительные метрики")


class ModuleInfo(BaseModel):
    """Информация о модуле"""
    id: str = Field(..., description="Идентификатор модуля")
    name: str = Field(..., description="Название модуля")
    description: str = Field(..., description="Описание модуля")
    version: str = Field(..., description="Версия модуля")
    status: ModuleStatus = Field(..., description="Текущий статус")
    enabled: bool = Field(..., description="Включен ли модуль")
    can_disable: bool = Field(..., description="Можно ли отключить")
    configurable: bool = Field(..., description="Есть ли конфигурация")
    disable_warning: Optional[str] = Field(None, description="Предупреждение при отключении")
    metrics: ModuleMetrics = Field(default_factory=ModuleMetrics)

    class Config:
        json_schema_extra = {
            "example": {
                "id": "intuition_engine",
                "name": "IntuitionEngine",
                "description": "Интуитивная обработка запросов",
                "version": "3.0.0",
                "status": "active",
                "enabled": True,
                "can_disable": True,
                "configurable": True,
                "disable_warning": None,
                "metrics": {
                    "operations": 12847,
                    "ops_per_sec": 1284.7,
                    "avg_latency_us": 69.5,
                    "p95_latency_us": 120.0,
                    "errors": 0
                }
            }
        }


class ModuleConfig(BaseModel):
    """Конфигурация модуля"""
    values: Dict[str, Any] = Field(default_factory=dict)


class SetEnabledRequest(BaseModel):
    """Запрос на включение/выключение модуля"""
    enabled: bool = Field(..., description="Включить (true) или выключить (false)")


class SetConfigRequest(BaseModel):
    """Запрос на обновление конфигурации"""
    config: Dict[str, Any] = Field(..., description="Новая конфигурация")


class ModuleListResponse(BaseModel):
    """Ответ со списком модулей"""
    modules: list[ModuleInfo]
    total: int


class ModuleResponse(BaseModel):
    """Ответ с информацией о модуле"""
    module: ModuleInfo


class SuccessResponse(BaseModel):
    """Успешный ответ"""
    success: bool = True
    message: str = ""
