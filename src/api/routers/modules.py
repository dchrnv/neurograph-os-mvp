"""
Modules Endpoints

Endpoints for module management and information.
"""

from typing import Optional
from fastapi import APIRouter, HTTPException, status

from ..models.modules import (
    ModuleInfo,
    ModuleListResponse,
    ModuleResponse,
    SetEnabledRequest,
    SetConfigRequest,
    SuccessResponse,
)
from ..services.modules import module_service


router = APIRouter()


@router.get(
    "",
    response_model=ModuleListResponse,
    summary="Список модулей",
    description="Получить список всех модулей системы с их статусами и метриками",
)
async def list_modules():
    """Получить список всех модулей"""
    modules = module_service.list_modules()
    return ModuleListResponse(modules=modules, total=len(modules))


@router.get(
    "/{module_id}",
    response_model=ModuleResponse,
    summary="Информация о модуле",
    description="Получить детальную информацию о конкретном модуле",
)
async def get_module(module_id: str):
    """Получить информацию о модуле"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    return ModuleResponse(module=module)


@router.put(
    "/{module_id}/enabled",
    response_model=SuccessResponse,
    summary="Включить/выключить модуль",
    description="Включить или выключить функциональность модуля",
)
async def set_module_enabled(module_id: str, request: SetEnabledRequest):
    """Включить/выключить модуль"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )

    if not request.enabled and not module.can_disable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' нельзя отключить (core module)",
        )

    try:
        module_service.set_enabled(module_id, request.enabled)
        action = "включен" if request.enabled else "выключен"
        return SuccessResponse(
            success=True,
            message=f"Модуль '{module.name}' {action}",
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e),
        )


@router.get(
    "/{module_id}/metrics",
    summary="Метрики модуля",
    description="Получить текущие метрики модуля",
)
async def get_module_metrics(module_id: str):
    """Получить метрики модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    return {"metrics": module.metrics}


@router.get(
    "/{module_id}/config",
    summary="Конфигурация модуля",
    description="Получить текущую конфигурацию модуля",
)
async def get_module_config(module_id: str):
    """Получить конфигурацию модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )

    if not module.configurable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' не поддерживает конфигурацию",
        )

    config = module_service.get_config(module_id)
    return {"config": config or {}}


@router.put(
    "/{module_id}/config",
    response_model=SuccessResponse,
    summary="Обновить конфигурацию",
    description="Обновить конфигурацию модуля",
)
async def set_module_config(module_id: str, request: SetConfigRequest):
    """Обновить конфигурацию модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )

    if not module.configurable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' не поддерживает конфигурацию",
        )

    try:
        module_service.set_config(module_id, request.config)
        return SuccessResponse(
            success=True,
            message=f"Конфигурация модуля '{module.name}' обновлена",
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e),
        )
