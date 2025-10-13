
"""
Примеры использования системы конфигураций
"""
import os
import json
from pathlib import Path

# Предполагаем, что модуль установлен
from src.infrastructure.config import (
    config_manager,
    get_config,
    get_spec,
    with_config,
    ConfigGroup,
    initialize
)

def example_1_basic_usage():
    """Пример 1: Базовое использование"""
    print("=== Пример 1: Базовое использование ===")
    
    # Получение конфигурации
    token_config = get_config("token")
    print(f"Token config: {json.dumps(token_config, indent=2)[:200]}...")
    
    # Получение с дефолтным значением
    missing_config = get_config("non_existent", default={"empty": True})
    print(f"Missing config with default: {missing_config}")
    
    # Получение спецификации для ИИ
    token_spec = get_spec("token")
    if token_spec:
        print(f"Token spec version: {token_spec.get('version')}")

def example_2_group_usage():
    """Пример 2: Работа с группами"""
    print("\n=== Пример 2: Работа с группами ===")
    
    # Получение имен конфигов из группы (сам ConfigManager не имеет метода get_group)
    core_group = config_manager._groups.get("core")
    if core_group:
        print(f"Core group configs: {list(core_group.configs.keys())}")
    
    # Получение конфига из группы.
    # schema_name будет автоматически подставлен из определения группы,
    # но для наглядности можно указать его явно.
    grid_config = config_manager.get("grid", group="core")
    if grid_config:
        print(f"Grid dimensions: {grid_config.get('dimensions')}")

def example_3_environment_variables():
    """Пример 3: Использование переменных окружения"""
    print("\n=== Пример 3: Переменные окружения ===")
    
    # Установка переменной окружения
    os.environ["BATCH_SIZE"] = "64"
    os.environ["ENABLE_AUGMENTATION"] = "true"
    
    # Перезагрузка конкретного конфига для применения изменений
    config_manager.get("token", force_reload=True)
    
    token_config = get_config("token")
    if token_config:
        batch_size = token_config.get("properties", {}).get("processing", {}).get("batch_size")
        print(f"Batch size from env: {batch_size}")

def example_4_decorator_usage():
    """Пример 4: Использование декоратора"""
    print("\n=== Пример 4: Декоратор для инъекции конфигов ===")
    
    @with_config("processor", group="processing", arg_name="proc_config")
    def process_data(data, proc_config=None):
        """Функция с автоматической инъекцией конфига"""
        if proc_config:
            pipeline = proc_config.get("pipeline", [])
            workers = proc_config.get("workers", 1)
            print(f"Processing with {workers} workers")
            print(f"Pipeline stages: {pipeline}")
            return f"Processed: {data}"
        return "No config"
    
    # Вызов функции - конфиг будет инъецирован автоматически
    result = process_data("test_data")
    print(f"Result: {result}")

def example_5_hot_reload():
    """Пример 5: Горячая перезагрузка"""
    print("\n=== Пример 5: Горячая перезагрузка ===")
    
    # Включение горячей перезагрузки
    from src.infrastructure.config import setup_hot_reload
    setup_hot_reload()
    
    print("Hot reload enabled. Touch .reload_configs file to trigger reload")
    print("Or send SIGUSR1 signal on Unix systems")

def example_6_custom_group():
    """Пример 6: Создание кастомной группы"""
    print("\n=== Пример 6: Кастомная группа конфигов ===")
    
    # # Создание новой группы
    # ml_group = ConfigGroup(
    #     name="machine_learning",
    #     configs={
    #         "model": "ml/model",
    #         "training": "ml/training",
    #         "inference": "ml/inference"
    #     },
    #     specs={
    #         "model": "ml/model",
    #         "training": "ml/training"
    #     },
    #     schemas={
    #         "model": "model",
    #         "training": "training"
    #     }
    # )
    
    # # Регистрация группы (метод register_group не реализован в ConfigManager)
    # # config_manager.register_group(ml_group)
    # print(f"Registered group: {ml_group.name}")
    
    # # Использование
    # # model_config = config_manager.get("model", group="machine_learning")
    print("NOTE: 'register_group' method is not implemented in ConfigManager.")

def example_7_proxy_lazy_loading():
    """Пример 7: Ленивая загрузка через прокси"""
    print("\n=== Пример 7: Ленивая загрузка ===")
    
    # # Создание прокси - конфиг еще не загружен (метод get_proxy не реализован)
    # token_proxy = config_manager.get_proxy("token", schema_name="token")
    # print("Proxy created (config not loaded yet)")
    
    # # Конфиг загружается при первом обращении
    # token_type = token_proxy.data.get("token_type")
    # print(f"Token type (loaded on demand): {token_type}")
    
    # # Последующие обращения используют кэш
    # props = token_proxy["properties"]
    # print(f"Properties keys: {list(props.keys())}")
    print("NOTE: 'get_proxy' method is not implemented in ConfigManager.")

def example_8_environment_switch():
    """Пример 8: Переключение окружений"""
    print("\n=== Пример 8: Переключение окружений ===")
    
    # Текущее окружение
    current_env = config_manager.loader.environment
    print(f"Current environment: {current_env}")
    
    # Переключение на production
    config_manager.update_environment("production")
    print(f"Switched to '{config_manager.loader.environment}' environment")
    
    # Конфиги перезагружены с production настройками
    # В production.yaml нет batch_size, поэтому он будет None
    token_config_prod = get_config("token")
    if token_config_prod:
        batch_size = token_config_prod.get("properties", {}).get("processing", {}).get("batch_size")
        print(f"Production batch size (from token.yaml, as it's not in production.yaml): {batch_size}")
    
    # Возврат к development
    config_manager.update_environment(current_env)
    print(f"Switched back to '{config_manager.loader.environment}' environment")

def example_9_validation():
    """Пример 9: Валидация конфигураций"""
    print("\n=== Пример 9: Валидация ===")
    
    # Загрузка с валидацией по схеме. Метод get инкапсулирует эту логику.
    # Если конфиг невалиден, get вернет None и запишет warning в лог.
    grid_config = config_manager.get("grid", group="core")
    if grid_config:
        print("Grid config loaded successfully (validation is handled by 'get' method).")
    else:
        print("Failed to load grid config. Check logs for validation errors.")

def example_10_debug_export():
    """Пример 10: Экспорт отладочной информации"""
    print("\n=== Пример 10: Отладочная информация ===")
    
    # # Метод export_debug_info не реализован
    # debug_info = config_manager.export_debug_info()
    print("NOTE: 'export_debug_info' method is not implemented in ConfigManager.")

def example_11_quick_access():
    """Пример 11: Быстрый доступ к стандартным конфигам"""
    print("\n=== Пример 11: Быстрый доступ ===")
    
    # # Свойства для быстрого доступа не реализованы в ConfigManager
    # token = config_manager.token
    # grid = config_manager.grid
    # graph = config_manager.graph
    # processor = config_manager.processor
    print("NOTE: Property-based access (e.g., config_manager.token) is not implemented.")

def run_all_examples():
    """Запуск всех примеров"""
    examples = [
        example_1_basic_usage,
        example_2_group_usage,
        example_3_environment_variables,
        example_4_decorator_usage,
        example_5_hot_reload,
        example_6_custom_group,
        example_7_proxy_lazy_loading,
        example_8_environment_switch,
        example_9_validation,
        example_10_debug_export,
        example_11_quick_access
    ]
    
    for example in examples:
        try:
            example()
        except Exception as e:
            print(f"Error in {example.__name__}: {e}")
        print()

if __name__ == "__main__":
    # Инициализация с кастомными настройками
    # Пути в примере относительные, а автоинициализация использует абсолютные.
    # Для запуска этого скрипта напрямую, убедитесь, что папки 'configs' и 'specs' существуют в текущей директории.
    initialize(
        configs_dir=str(Path(__file__).parent.parent.parent.parent / 'config'),
        specs_dir=str(Path(__file__).parent.parent.parent.parent / 'config' / 'specs'),
        environment="development",
        enable_hot_reload=False # Отключим для простого запуска, т.к. watcher - бесконечный цикл
    )
    
    # Запуск примеров
    run_all_examples()
