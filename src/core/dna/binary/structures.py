"""
Базовые структуры и константы CDNA-блоков.
"""

from dataclasses import dataclass
from typing import List
import struct


@dataclass
class GridPhysicsConstants:
    """Параметры физики сетки CDNA"""
    dimension_semantic_ids: List[int]
    dimension_flags: List[int]
    dimension_scales: List[float]

    FORMAT = '<8H8B8f'

    def pack(self) -> bytes:
        # exact 32-byte block
        packed = struct.pack(self.FORMAT,
                             *(self.dimension_semantic_ids or [0]*8),
                             *(self.dimension_flags or [0]*8),
                             *(self.dimension_scales or [1.0]*8))
        return packed[:32]

    @classmethod
    def unpack(cls, data: bytes) -> 'GridPhysicsConstants':
        src = data[:32].ljust(32, b'\x00')
        unpacked = struct.unpack(cls.FORMAT, src)
        return cls(
            dimension_semantic_ids=list(unpacked[0:8]),
            dimension_flags=list(unpacked[8:16]),
            dimension_scales=list(unpacked[16:24])
        )


@dataclass
class GraphTopologyRules:
    """Правила топологии нейрографа"""
    max_connections: int
    decay_rate: float
    symmetry_tolerance: float

    FORMAT = '<4f4I4H8B'

    def pack(self) -> bytes:
        floats = [float(self.max_connections), float(self.decay_rate), float(self.symmetry_tolerance), 0.0]
        uints = [0, 0, 0, 0]
        ushorts = [0, 0, 0, 0]
        bytes_r = [0]*8
        packed = struct.pack(self.FORMAT, *(floats + uints + ushorts + bytes_r))
        return packed[:32]

    @classmethod
    def unpack(cls, data: bytes) -> 'GraphTopologyRules':
        src = data[:32].ljust(32, b'\x00')
        unpacked = struct.unpack(cls.FORMAT, src)
        return cls(
            max_connections=int(unpacked[0]),
            decay_rate=float(unpacked[1]),
            symmetry_tolerance=float(unpacked[2])
        )


@dataclass
class TokenBaseProperties:
    """Базовые свойства токенов (32 байта)"""
    weight_min: float = 0.0
    weight_max: float = 1.0
    base_flags_allowed: int = 0xFFFF
    max_coordinate_levels: int = 8

    FORMAT = '<2f4I4H8B'

    def pack(self) -> bytes:
        floats = [self.weight_min, self.weight_max]
        uints = [self.base_flags_allowed, 0, 0, self.max_coordinate_levels]
        ushorts = [0, 0, 0, 0]
        bytes_r = [0]*8
        packed = struct.pack(self.FORMAT, *(floats + uints + ushorts + bytes_r))
        return packed[:32]

    @classmethod
    def unpack(cls, data: bytes) -> 'TokenBaseProperties':
        src = data[:32].ljust(32, b'\x00')
        unpacked = struct.unpack(cls.FORMAT, src)
        weight_min = float(unpacked[0])
        weight_max = float(unpacked[1])
        base_flags_allowed = int(unpacked[2])
        max_levels = int(unpacked[5]) if len(unpacked) > 5 else 8
        return cls(weight_min=weight_min, weight_max=weight_max, base_flags_allowed=base_flags_allowed, max_coordinate_levels=max_levels)


@dataclass
class EvolutionConstraints:
    mutation_rate_base: float = 0.01
    mutation_rate_max: float = 0.1
    crossover_rate: float = 0.05
    selection_pressure: float = 0.8

    FORMAT = '<4f4I4H8B'

    def pack(self) -> bytes:
        floats = [self.mutation_rate_base, self.mutation_rate_max, self.crossover_rate, self.selection_pressure]
        uints = [0, 0, 0, 0]
        ushorts = [0, 0, 0, 0]
        bytes_r = [0]*8
        packed = struct.pack(self.FORMAT, *(floats + uints + ushorts + bytes_r))
        return packed[:32]

    @classmethod
    def unpack(cls, data: bytes) -> 'EvolutionConstraints':
        src = data[:32].ljust(32, b'\x00')
        unpacked = struct.unpack(cls.FORMAT, src)
        return cls(
            mutation_rate_base=float(unpacked[0]),
            mutation_rate_max=float(unpacked[1]),
            crossover_rate=float(unpacked[2]),
            selection_pressure=float(unpacked[3])
        )
