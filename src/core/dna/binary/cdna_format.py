"""
CDNA Binary Format — определение структуры кодировки ДНК.
Используется для сериализации и чтения блоков CDNA.
"""

import struct
from typing import Dict, Any
from .structures import (
    GridPhysicsConstants,
    GraphTopologyRules,
    TokenBaseProperties,
    EvolutionConstraints,
)
import hashlib


class CDNAStructure:
    """Полная структура CDNA (128 байт) с pack/unpack и checksum"""

    TOTAL_SIZE = 128
    BLOCK_SIZE = 32

    def __init__(self):
        self.grid_physics = GridPhysicsConstants([0]*8, [0]*8, [1.0]*8)
        self.graph_topology = GraphTopologyRules(0, 0.0, 0.0)
        self.token_properties = TokenBaseProperties()
        self.evolution_constraints = EvolutionConstraints()
        self._checksum = 0

    def pack(self) -> bytes:
        b1 = self.grid_physics.pack()[:32]
        b2 = self.graph_topology.pack()[:32]
        b3 = self.token_properties.pack()[:32]
        b4 = self.evolution_constraints.pack()[:32]
        data = b1 + b2 + b3 + b4
        # compute checksum
        self._checksum = int(hashlib.sha256(data).hexdigest()[:8], 16)
        return data

    @classmethod
    def unpack(cls, data: bytes) -> 'CDNAStructure':
        if len(data) != cls.TOTAL_SIZE:
            raise ValueError(f"CDNA data must be exactly {cls.TOTAL_SIZE} bytes")
        obj = cls()
        obj.grid_physics = GridPhysicsConstants.unpack(data[0:32])
        obj.graph_topology = GraphTopologyRules.unpack(data[32:64])
        obj.token_properties = TokenBaseProperties.unpack(data[64:96])
        obj.evolution_constraints = EvolutionConstraints.unpack(data[96:128])
        obj._checksum = int(hashlib.sha256(data).hexdigest()[:8], 16)
        return obj

    def get_hot_slice(self, component: str) -> bytes:
        full = self.pack()
        if component == 'graph':
            return full[32:96]
        if component == 'coordinate_system':
            return full[0:32] + full[64:96]
        if component == 'evolution':
            return full[32:128]
        if component == 'token':
            return full[64:96]
        return full

    def checksum(self) -> int:
        return self._checksum


class CDNABinaryFormat:
    """
    Класс для чтения и записи бинарной структуры CDNA (128 байт).
    Делится на логические секции (GRID_PHYSICS, TOKEN_PROPERTIES и др.)
    """

    SIZE = 128

    SECTIONS = {
        "GRID_PHYSICS": (0, 32),
        "NEURO_RULES": (32, 32),
        "TOKEN_PROPERTIES": (64, 32),
        "META": (96, 32),
    }

    def __init__(self, raw_bytes: bytes):
        if len(raw_bytes) < self.SIZE:
            raise ValueError(f"Invalid CDNA size: {len(raw_bytes)} < {self.SIZE}")
        self.raw = raw_bytes

    def get_section(self, name: str) -> bytes:
        """Возвращает срез данных по имени секции."""
        if name not in self.SECTIONS:
            raise KeyError(f"Unknown CDNA section: {name}")
        start, size = self.SECTIONS[name]
        return self.raw[start:start + size]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CDNABinaryFormat":
        """Создаёт бинарное представление CDNA из словаря параметров."""
        buf = bytearray(cls.SIZE)

        # GRID_PHYSICS — примерный формат: 8H 8B 8f
        if "GRID_PHYSICS" in data:
            section = struct.pack(
                "<8H8B8f",
                *(data["GRID_PHYSICS"].get("semantic_ids", [0]*8)),
                *(data["GRID_PHYSICS"].get("flags", [0]*8)),
                *(data["GRID_PHYSICS"].get("scales", [1.0]*8))
            )
            buf[0:32] = section[:32]

        # TOKEN_PROPERTIES — формат: 2f 4I 4H 8B
        if "TOKEN_PROPERTIES" in data:
            section = struct.pack(
                "<2f4I4H8B",
                *data["TOKEN_PROPERTIES"].get("values", [0]*18)
            )
            buf[64:96] = section[:32]

        # META
        if "META" in data:
            meta = bytes(data["META"].get("signature", b"NGOS-CDNA-V1")).ljust(32, b"\x00")
            buf[96:128] = meta

        return cls(bytes(buf))
