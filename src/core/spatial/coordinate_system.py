#!/usr/bin/env python3
"""
Главная система координат NeuroGraph OS (с интеграцией DNA Guardian)

Это объединённая версия: сохранена исходная функциональность системы координат,
и добавлена необязательная интеграция с DNAGuardian / CDNA (hot-slices, масштабирование уровней,
обработка событий). Если модуль src/core/dna отсутствует — система работает в обычном режиме.
"""

import time
import threading
from typing import Any, Dict, List, Optional, Tuple
from functools import wraps

from .coordinates import (
    Point3D, Region3D, MultiCoordinate,
    CoordinateConfig, LevelConfig, IndexType,
    SystemStats, LevelStats
)
from .spatial_index import SpatialIndex, SparseGridIndex, SpatialHashIndex
from ..token.token import Token

# --- DNA integration (optional) ---
try:
    from ..dna.guardian import DNAGuardian
    from ..dna.integration import DNAIntegratedComponent
except Exception:
    DNAGuardian = None
    DNAIntegratedComponent = object

# --- Optional Experience integration (необязательно) ---
try:
    # проект иногда использует абсолютные импорты для experience
    from src.core.experience.stream import ExperienceStream  # type: ignore
    from src.core.experience.event import ExperienceEvent  # type: ignore
except Exception:
    ExperienceStream = None  # type: ignore
    ExperienceEvent = None  # type: ignore

# === Профилировщик операций ===
def _profile_operation(operation_name: str):
    """Декоратор для профилирования операций"""
    def decorator(method):
        @wraps(method)
        def wrapper(self, *args, **kwargs):
            if not getattr(self, "_enable_profiling", False):
                return method(self, *args, **kwargs)
            start_time = time.perf_counter()
            try:
                return method(self, *args, **kwargs)
            finally:
                duration = time.perf_counter() - start_time
                if hasattr(self, '_system_stats'):
                    self._system_stats.record_operation(operation_name, duration)
        return wrapper
    return decorator


class CoordinateSystem(DNAIntegratedComponent):
    """
    Многомерная система координат для токенов NeuroGraph OS.
    Поддерживает optional интеграцию с DNAGuardian (cdna slices, события).
    """

    def __init__(self, config: Optional[CoordinateConfig] = None,
                 dna_guardian: Optional[DNAGuardian] = None,
                 experience_stream: Optional[Any] = None):
        # DNA integration init if available
        if dna_guardian:
            # Инициализируем поведение DNAIntegratedComponent
            try:
                DNAIntegratedComponent.__init__(self, "coordinate_system", dna_guardian)
            except Exception:
                # если DNAIntegratedComponent требует другой init — просто сохраняем ссылку
                self.dna_guardian = dna_guardian
        else:
            self.dna_guardian = None

        # Initialize config: use provided or default
        if not hasattr(self, 'config') or self.config is None:
            self.config = CoordinateConfig()
        if config is not None:
            # merge values into new config to avoid mutating provided one
            new_cfg = CoordinateConfig()
            for key, value in config.__dict__.items():
                setattr(new_cfg, key, value)
            self.config = new_cfg

        # Ensure levels are present
        if not self.config.levels:
            self.config.levels = self.config.get_default_levels()

        # Indexes and registry
        self._indexes: Dict[int, SpatialIndex] = {}
        self._token_registry: Dict[int, MultiCoordinate] = {}

        for level, level_config in self.config.levels.items():
            if level_config.index_type == IndexType.SPARSE_GRID:
                self._indexes[level] = SparseGridIndex(level_config)
            elif level_config.index_type == IndexType.SPATIAL_HASH:
                self._indexes[level] = SpatialHashIndex(level_config)
            else:
                # Fallback: SparseGridIndex
                self._indexes[level] = SparseGridIndex(level_config)

        # Статистика и кэширование
        self._system_stats = SystemStats()
        self._query_cache = {}
        self._cache_hits = 0
        self._cache_misses = 0
        self._lock = threading.RLock()

        self._enable_profiling = bool(self.config.enable_statistics)

        # CDNA-driven dimension metadata (populated if guardian present)
        self._dimension_semantic_ids: List[int] = []
        self._dimension_flags: List[int] = []
        self._dimension_scales: List[float] = []

        # Если Guardian задан — загрузим spatial constraints
        if self.dna_guardian:
            try:
                self._load_spatial_constraints_from_cdna()
            except Exception as e:
                # Не ломаем систему, только логируем
                print(f"[DNA] CoordinateSystem: failed to load CDNA on init: {e}")

        # Optional experience stream (safely attached)
        # Поддерживаем как конкретный тип ExperienceStream, так и любые объекты
        # с методом write_event(event). Если поток отсутствует — ничего не делаем.
        self.experience = experience_stream

    # === DNA Integration methods ===
    def _load_spatial_constraints_from_cdna(self) -> None:
        """Загрузить пространственные ограничения из CDNA (GRID_PHYSICS block)."""
        # Prefer hot-slice API if available
        cdna_slice = None
        try:
            # try slice first (recommended)
            cdna_slice = self.get_cdna_slice("coordinate_system")
        except Exception:
            # fallback: some implementations expose get_cdna_data
            try:
                cdna_slice = self.get_cdna_data()
            except Exception:
                cdna_slice = None

        if not cdna_slice:
            return

        # Expect at least 32 bytes for GRID_PHYSICS block
        if len(cdna_slice) < 32:
            return

        try:
            # Формат: 8H (16 bytes) + 8B (8 bytes) + 8f (32 bytes) -> but original used '<8H8B8f'
            # Slice may contain only GRID_PHYSICS at start
            import struct
            # safe-unpack: ensure length
            fmt = '<8H8B8f'
            needed = struct.calcsize(fmt)
            if len(cdna_slice) < needed:
                # если срез короче — попробуем читать only available parts:
                # read first 8H and 8B if possible
                if len(cdna_slice) >= struct.calcsize('<8H8B'):
                    partial = struct.unpack('<8H8B', cdna_slice[:struct.calcsize('<8H8B')])
                    self._dimension_semantic_ids = list(partial[0:8])
                    self._dimension_flags = list(partial[8:16])
                return

            unpacked = struct.unpack(fmt, cdna_slice[:needed])
            # unpacked: [8H] + [8B] + [8f]
            self._dimension_semantic_ids = list(unpacked[0:8])
            self._dimension_flags = list(unpacked[8:16])
            self._dimension_scales = list(unpacked[16:24])

            print(f"[DNA] CoordinateSystem loaded CDNA: {len(self._dimension_scales)} dimensions")

            # Apply scales to known levels (best-effort)
            self._apply_dimension_scales_to_levels()

        except Exception as e:
            print(f"[DNA] CoordinateSystem: invalid GRID_PHYSICS block or parse error: {e}")

    def on_cdna_updated(self, event) -> None:
        """Обработка обновления CDNA: перезагрузить spatial constraints и (возможно) переиндексировать."""
        try:
            print("[DNA] CoordinateSystem: CDNA updated, reloading spatial constraints")
            old_scales = list(self._dimension_scales) if self._dimension_scales else []
            self._load_spatial_constraints_from_cdna()
            # Если масштабы изменились — попробуем переиндексацию, только если включено в конфиге
            if old_scales != self._dimension_scales and getattr(self.config, 'enable_auto_reindex', False):
                self._attempt_reindex_after_scale_change(old_scales, self._dimension_scales)
        except Exception as e:
            print(f"[DNA] CoordinateSystem.on_cdna_updated error: {e}")

    def on_adna_updated(self, event) -> None:
        """Обработка обновления ADNA — можно реагировать на параметры типа spatial_resolution и т.п."""
        try:
            key = event.metadata.get('key', '') if hasattr(event, 'metadata') else ''
            if any(word in key for word in ['spatial', 'coordinate', 'grid', 'resolution']):
                print(f"[DNA] CoordinateSystem: ADNA parameter changed: {key}")
                # при необходимости можно применить динамическую настройку уровня индексирования
        except Exception as e:
            print(f"[DNA] CoordinateSystem.on_adna_updated error: {e}")

    # --- Дополнения от DNA/DNAIntegration (из предложенного патча) ---
    def _load_spatial_constraints_from_cdna(self) -> None:
        """Загрузить пространственные ограничения из CDNA (GRID_PHYSICS block).

        Поддерживает частичные срезы и безопасно игнорирует несовместимые форматы.
        """
        # Попробуем получить срез через hot-slice API, затем fallback на get_cdna_data
        cdna_slice = None
        try:
            cdna_slice = self.get_cdna_slice("coordinate_system")
        except Exception:
            try:
                cdna_slice = self.get_cdna_data()
            except Exception:
                cdna_slice = None

        if not cdna_slice:
            return

        # Ожидаем по крайней мере 32 байта; формат может варьироваться
        if len(cdna_slice) < 32:
            return

        try:
            import struct
            fmt = '<8H8B8f'
            needed = struct.calcsize(fmt)
            if len(cdna_slice) < needed:
                # Частичное чтение: попробуем прочитать семантики и флаги
                min_fmt = '<8H8B'
                if len(cdna_slice) >= struct.calcsize(min_fmt):
                    partial = struct.unpack(min_fmt, cdna_slice[:struct.calcsize(min_fmt)])
                    self._dimension_semantic_ids = list(partial[0:8])
                    self._dimension_flags = list(partial[8:16])
                return

            unpacked = struct.unpack(fmt, cdna_slice[:needed])
            self._dimension_semantic_ids = list(unpacked[0:8])
            self._dimension_flags = list(unpacked[8:16])
            self._dimension_scales = list(unpacked[16:24])

            print(f"[DNA] CoordinateSystem loaded CDNA: {len(self._dimension_scales)} dimensions")

            # Применяем масштабы
            self._apply_dimension_scales_to_levels()

        except Exception as e:
            print(f"[DNA] CoordinateSystem: invalid GRID_PHYSICS block or parse error: {e}")

    def _apply_dimension_scales_to_levels(self) -> None:
        """
        Best-effort: применить масштабы (dimension_scales) к конфигам уровней / индексам.
        Если индекс поддерживает API set_scale/update_scale, вызвать его.
        """
        for level, scale in enumerate(self._dimension_scales):
            if level not in self.config.levels:
                continue
            level_cfg = self.config.levels[level]
            # Применяем логически — если в конфиге есть поле scale, обновим его
            try:
                if hasattr(level_cfg, 'scale'):
                    level_cfg.scale = scale
            except Exception:
                pass

            # Попробуем известные API индекса
            index = self._indexes.get(level)
            if index is None:
                continue
            try:
                if hasattr(index, 'set_scale'):
                    index.set_scale(scale)
                elif hasattr(index, 'update_scale'):
                    index.update_scale(scale)
                elif hasattr(index, 'dimension_scale'):
                    index.dimension_scale = scale
                # иначе — индекс не поддержует динамическое масштабирование; пропускаем
            except Exception as e:
                # не критично — логируем и продолжаем
                print(f"[DNA] CoordinateSystem: failed to apply scale for level {level}: {e}")

    def _attempt_reindex_after_scale_change(self, old_scales: List[float], new_scales: List[float]) -> None:
        """Best-effort переиндексация при изменении масштабов.

        Не делает радикальных изменений по-умолчанию; вызывает
        доступные методы индексов для пересчёта.
        """
        try:
            for level in set(range(len(old_scales))) | set(range(len(new_scales))):
                index = self._indexes.get(level)
                if not index:
                    continue
                if hasattr(index, 'rescale'):
                    try:
                        index.rescale(old_scales[level] if level < len(old_scales) else 1.0,
                                      new_scales[level] if level < len(new_scales) else 1.0)
                        print(f"[DNA] CoordinateSystem: rescaled index for level {level}")
                        continue
                    except Exception:
                        pass
                if hasattr(index, 'reindex_all'):
                    try:
                        index.reindex_all(self._token_registry)
                        print(f"[DNA] CoordinateSystem: reindexed level {level} using reindex_all")
                        continue
                    except Exception:
                        pass
                print(f"[DNA] CoordinateSystem: index at level {level} does not support reindexing API; manual reindex recommended")
        except Exception as e:
            print(f"[DNA] CoordinateSystem: reindex attempt failed: {e}")

    def _attempt_reindex_after_scale_change(self, old_scales: List[float], new_scales: List[float]) -> None:
        """
        Best-effort переиндексация при изменении масштабов:
        - если индексы поддерживают API для пересчёта, вызываем их;
        - иначе — оставляем как есть (слишком рискованно удалять/пересоздавать индексы автоматически).
        """
        # Если количество уровней стало больше/меньше — уведомляем (но не переписываем индексы)
        try:
            for level in set(range(len(old_scales))) | set(range(len(new_scales))):
                index = self._indexes.get(level)
                if not index:
                    continue
                # если есть специализированный API — используем
                if hasattr(index, 'rescale'):
                    try:
                        index.rescale(old_scales[level] if level < len(old_scales) else 1.0,
                                      new_scales[level] if level < len(new_scales) else 1.0)
                        print(f"[DNA] CoordinateSystem: rescaled index for level {level}")
                        continue
                    except Exception:
                        pass
                # если есть метод to_reindex / reindex_all — попробовать его
                if hasattr(index, 'reindex_all'):
                    try:
                        index.reindex_all(self._token_registry)
                        print(f"[DNA] CoordinateSystem: reindexed level {level} using reindex_all")
                        continue
                    except Exception:
                        pass
                # safe fallback: do nothing and warn
                print(f"[DNA] CoordinateSystem: index at level {level} does not support reindexing API; manual reindex recommended")
        except Exception as e:
            print(f"[DNA] CoordinateSystem: reindex attempt failed: {e}")

    # === БАЗОВЫЕ ОПЕРАЦИИ ===

    @_profile_operation("place_token")
    def place_token(self, token: Token, coordinates: MultiCoordinate) -> bool:
        """
        Размещает токен в многомерном пространстве координат
        """
        with self._lock:
            try:
                # Удаляем старую запись (если есть)
                if token.id in self._token_registry:
                    self.remove_token(token.id)
               
                # Размещаем в индексах по уровням
                for level, point in coordinates.coordinates.items():
                    if level not in self._indexes:
                        continue

                    # Валидация по уровню
                    if self.config.enable_validation:
                        level_config = self.config.levels[level]
                        if not (level_config.validate_coordinate(point.x) and
                                level_config.validate_coordinate(point.y) and
                                level_config.validate_coordinate(point.z)):
                            raise ValueError(f"Координаты вне допустимого диапазона для уровня {level}")

                    self._indexes[level].insert(point, token)

                # Регистрируем токен и обновляем статистику
                self._token_registry[token.id] = coordinates
                self._system_stats.total_tokens += 1

                for level in coordinates.get_active_levels():
                    self._system_stats.tokens_per_level.setdefault(level, 0)
                    self._system_stats.tokens_per_level[level] += 1

                # Записываем опыт (опционально). Если поток опыта присутствует и
                # доступен метод write_event — отправляем событие. Стараемся не
                # ломать основной поток при ошибках в модуле experience.
                if getattr(self, 'experience', None):
                    try:
                        event_payload = {
                            "event_id": f"token_place_{token.id}_{int(time.time()*1000)}",
                            "event_type": "token_placed",
                            "timestamp": time.time(),
                            "source_component": "coordinate_system",
                            "data": {
                                "token_id": token.id,
                                "coordinates": {
                                    level: {"x": p.x, "y": p.y, "z": p.z}
                                    for level, p in coordinates.coordinates.items()
                                }
                            },
                            "reward": self._calculate_placement_reward(token, coordinates)
                        }

                        # Если класс ExperienceEvent доступен — попробуем создать
                        # экземпляр, иначе передаём простой словарь.
                        if ExperienceEvent is not None:
                            try:
                                event_obj = ExperienceEvent(**event_payload)
                            except Exception:
                                event_obj = event_payload
                        else:
                            event_obj = event_payload

                        # Наконец пишем событие в поток — если метод падает, игнорируем
                        try:
                            write = getattr(self.experience, 'write_event', None)
                            if callable(write):
                                write(event_obj)
                        except Exception:
                            pass
                    except Exception:
                        # Любая ошибка в логике experience не должна ломать
                        # основную операцию размещения.
                        pass

                return True

            except Exception as e:
                # Откатываем частичные изменения
                if token.id in self._token_registry:
                    try:
                        self.remove_token(token.id)
                    except Exception:
                        pass
                raise e

    def get_token_at(self, coordinates: MultiCoordinate) -> List[Token]:
        tokens: List[Token] = []
        with self._lock:
            for level, point in coordinates.coordinates.items():
                if level in self._indexes:
                    level_tokens = self._indexes[level].query_point(point)
                    tokens.extend(level_tokens)

        # убираем дубликаты
        unique = {}
        for t in tokens:
            unique[t.id] = t
        return list(unique.values())

    def move_token(self, token_id: int, new_coordinates: MultiCoordinate) -> bool:
        with self._lock:
            if token_id not in self._token_registry:
                return False

            token = None
            old_coordinates = self._token_registry[token_id]

            for level, point in old_coordinates.coordinates.items():
                if level in self._indexes:
                    tokens_at_point = self._indexes[level].query_point(point)
                    for t in tokens_at_point:
                        if t.id == token_id:
                            token = t
                            break
                    if token:
                        break

            if not token:
                return False

            # remove old and place new
            self.remove_token(token_id)
            return self.place_token(token, new_coordinates)

    def remove_token(self, token_id: int) -> bool:
        with self._lock:
            if token_id not in self._token_registry:
                return False

            coordinates = self._token_registry[token_id]
            removed = False

            for level, point in coordinates.coordinates.items():
                if level in self._indexes:
                    try:
                        if self._indexes[level].remove(point, token_id):
                            removed = True
                            if level in self._system_stats.tokens_per_level:
                                self._system_stats.tokens_per_level[level] -= 1
                                if self._system_stats.tokens_per_level[level] <= 0:
                                    del self._system_stats.tokens_per_level[level]
                    except Exception:
                        # индекс может бросать — логируем и продолжаем
                        print(f"[Warn] CoordinateSystem: index.remove failed for level {level}, token {token_id}")

            if removed:
                try:
                    del self._token_registry[token_id]
                    self._system_stats.total_tokens -= 1
                except KeyError:
                    pass
                return True

            return False

    # === ПОИСК И НАВИГАЦИЯ ===

    @_profile_operation("find_tokens_in_region")
    def find_tokens_in_region(self, region: Region3D, level: int) -> List[Token]:
        if level not in self._indexes:
            return []
        results = self._indexes[level].query_region(region)
        return results[:self.config.max_search_results]

    @_profile_operation("find_nearest_neighbors")
    def find_nearest_neighbors(self, point: Point3D, level: int, k: int = 10) -> List[Tuple[Token, float]]:
        if level not in self._indexes:
            return []

        radius = 0.1
        max_radius = 10.0
        neighbors: List[Tuple[Token, float]] = []

        while len(neighbors) < k and radius <= max_radius:
            candidates = self._indexes[level].query_radius(point, radius)
            neighbors_with_distance: List[Tuple[Token, float]] = []

            for token in candidates:
                if token.id in self._token_registry:
                    token_coords = self._token_registry[token.id]
                    token_point = token_coords.get_point(level)
                    if token_point:
                        distance = point.distance_to(token_point)
                        neighbors_with_distance.append((token, distance))

            neighbors_with_distance.sort(key=lambda x: x[1])
            neighbors = neighbors_with_distance[:k]

            if len(neighbors) < k:
                radius *= 2

        return neighbors

    @_profile_operation("find_tokens_in_radius")
    def find_tokens_in_radius(self, center: Point3D, level: int, radius: float) -> List[Token]:
        if level not in self._indexes:
            return []
        results = self._indexes[level].query_radius(center, radius)
        return results[:self.config.max_search_results]

    # === МНОГОМЕРНЫЕ ОПЕРАЦИИ ===

    def find_tokens_across_levels(self, query: MultiCoordinate, tolerance: float = 0.1) -> List[Token]:
        if not query.coordinates:
            return []

        candidates_by_level = {}
        for level, point in query.coordinates.items():
            if level in self._indexes:
                tokens_in_radius = self.find_tokens_in_radius(point, level, tolerance)
                candidates_by_level[level] = set(t.id for t in tokens_in_radius)

        if not candidates_by_level:
            return []

        values = list(candidates_by_level.values())
        if not values:
            return []
        # start with first set and intersect remaining sets (safe for typing and empty cases)
        common_token_ids = values[0].intersection(*values[1:]) if len(values) > 1 else set(values[0])
        result: List[Token] = []
        for token_id in common_token_ids:
            if token_id in self._token_registry:
                coordinates = self._token_registry[token_id]
                for level, point in coordinates.coordinates.items():
                    if level in self._indexes:
                        tokens_at_point = self._indexes[level].query_point(point)
                        for token in tokens_at_point:
                            if token.id == token_id:
                                result.append(token)
                                break
                        break
        return result

    def project_to_level(self, tokens: List[Token], target_level: int) -> List[Point3D]:
        points: List[Point3D] = []
        for token in tokens:
            if token.id in self._token_registry:
                coordinates = self._token_registry[token.id]
                point = coordinates.get_point(target_level)
                if point:
                    points.append(point)
        return points

    # === СТАТИСТИКА И АНАЛИЗ ===

    def get_density_map(self, level: int, resolution: float = 0.1) -> Dict[Region3D, int]:
        if level not in self._indexes:
            return {}
        index = self._indexes[level]
        if hasattr(index, 'get_density_map'):
            return index.get_density_map(resolution)
        return {}

    def get_level_statistics(self, level: int) -> Optional[LevelStats]:
        if level not in self._indexes:
            return None
        index = self._indexes[level]
        if hasattr(index, 'get_stats'):
            return index.get_stats()
        return None

    def get_token_distribution(self) -> Dict[int, int]:
        return self._system_stats.tokens_per_level.copy()

    def get_system_statistics(self) -> SystemStats:
        # approximate memory usage
        memory_usage = 0
        memory_usage += len(self._token_registry) * 200
        for index in self._indexes.values():
            try:
                memory_usage += index.get_count() * 100
            except Exception:
                pass
        self._system_stats.memory_usage_bytes = memory_usage
        self._system_stats.last_updated = time.time()
        return self._system_stats

    # === СЛУЖЕБНЫЕ МЕТОДЫ ===

    def get_active_levels(self) -> List[int]:
        return list(self._indexes.keys())

    def get_token_coordinates(self, token_id: int) -> Optional[MultiCoordinate]:
        return self._token_registry.get(token_id)

    def get_tokens_count(self) -> int:
        return self._system_stats.total_tokens

    def clear_level(self, level: int) -> bool:
        if level not in self._indexes:
            return False

        with self._lock:
            bounds = self._indexes[level].get_bounds()
            if bounds:
                tokens_to_remove = self.find_tokens_in_region(bounds, level)
                for token in tokens_to_remove:
                    if token.id in self._token_registry:
                        coordinates = self._token_registry[token.id]
                        if level in coordinates.coordinates:
                            point = coordinates.coordinates[level]
                            try:
                                self._indexes[level].remove(point, token.id)
                            except Exception:
                                pass
                            del coordinates.coordinates[level]
                            if not coordinates.coordinates:
                                try:
                                    del self._token_registry[token.id]
                                    self._system_stats.total_tokens -= 1
                                except KeyError:
                                    pass

            if level in self._system_stats.tokens_per_level:
                del self._system_stats.tokens_per_level[level]

        return True

    def clear_all(self) -> None:
        with self._lock:
            self._token_registry.clear()
            self._system_stats = SystemStats()
            # recreate indexes to default configs
            for level, level_config in self.config.levels.items():
                if level_config.index_type == IndexType.SPARSE_GRID:
                    self._indexes[level] = SparseGridIndex(level_config)
                elif level_config.index_type == IndexType.SPATIAL_HASH:
                    self._indexes[level] = SpatialHashIndex(level_config)

    def __len__(self) -> int:
        return self._system_stats.total_tokens

    def __contains__(self, token_id: int) -> bool:
        return token_id in self._token_registry

    def __repr__(self) -> str:
        return (f"CoordinateSystem(levels={len(self._indexes)}, "
                f"tokens={self._system_stats.total_tokens})")

    # === Distance util with DNA scale support ===

    def calculate_weighted_distance(self, token1: Token, token2: Token, level: int) -> float:
        """Вычислить расстояние с учётом CDNA масштабов (если имеются)."""
        coords1 = self._token_registry.get(token1.id)
        coords2 = self._token_registry.get(token2.id)

        if not coords1 or not coords2:
            return float('inf')

        p1 = coords1.get_point(level)
        p2 = coords2.get_point(level)

        if not p1 or not p2:
            return float('inf')

        scale = 1.0
        if self._dimension_scales and level < len(self._dimension_scales):
            scale = self._dimension_scales[level]

        dx = (p1.x - p2.x) * scale
        dy = (p1.y - p2.y) * scale
        dz = (p1.z - p2.z) * scale

        return (dx*dx + dy*dy + dz*dz) ** 0.5

    # === Experience helpers ===
    def _calculate_placement_reward(self, token: Token, coordinates: MultiCoordinate) -> float:
        """Простейшая эвристика награды при размещении токена.

        Возвращает число float. Сделано очень просто: если у токена есть поле
        weight — используем его (нормализованно), иначе возвращаем 0.0.
        Метод сделан лёгким и безопасным; в будущем можно подключить ADNA/конфиг.
        """
        try:
            w = getattr(token, 'weight', 0.0)
            # Clamp to [-10, 10] и нормализовать в диапазон [-1,1]
            if w is None:
                return 0.0
            if w > 10:
                w = 10.0
            if w < -10:
                w = -10.0
            return float(w) / 10.0
        except Exception:
            return 0.0
