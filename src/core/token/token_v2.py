
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.
    

"""
Token v2.0 - Official Implementation
64-byte token structure with 8 semantic coordinate spaces

Based on: docs/token_extended_spec.md
Version: 2.0.0
Date: 2025-10-13
"""

import struct
import time
from typing import Optional, Tuple, Dict, Any
from dataclasses import dataclass, field
import numpy as np


# ═══════════════════════════════════════════════════════
# CONSTANTS
# ═══════════════════════════════════════════════════════

# Binary format: <24h I H f 2B I = 64 bytes
# 24h   = 24× int16 (coordinates: 8 levels × 3 axes)
# I     = uint32 (id)
# H     = uint16 (flags)
# f     = float32 (weight)
# 2B    = 2× uint8 (field_radius, field_strength)
# I     = uint32 (timestamp)
PACK_FORMAT = '<24h I H f 2B I'
TOKEN_SIZE = 64

# Coordinate space definitions
COORDINATE_SPACES = {
    0: "L1_PHYSICAL",    # Physical 3D space
    1: "L2_SENSORY",     # Sensory perception
    2: "L3_MOTOR",       # Motor/movement
    3: "L4_EMOTIONAL",   # Emotional (VAD model)
    4: "L5_COGNITIVE",   # Cognitive processing
    5: "L6_SOCIAL",      # Social interaction
    6: "L7_TEMPORAL",    # Temporal location
    7: "L8_ABSTRACT"     # Abstract/semantic
}

# Coordinate scales for each level
COORDINATE_SCALES = {
    0: 100,    # L1: ±327.67 meters
    1: 10000,  # L2: 0.0-1.0 normalized
    2: 1000,   # L3: ±32.767 m/s, m/s², rad/s
    3: 10000,  # L4: ±1.0 VAD model
    4: 10000,  # L5: 0.0-1.0 normalized
    5: 10000,  # L6: ±1.0 social metrics
    6: 100,    # L7: ±327 seconds (X,Y), Z uses 1000
    7: 10000   # L8: ±1.0 semantic
}

# Special value for undefined coordinates
COORD_UNDEFINED = 127

# ═══════════════════════════════════════════════════════
# FLAGS (16 bits)
# ═══════════════════════════════════════════════════════

# System flags (bits 0-7)
FLAG_ACTIVE       = 0x0001  # Bit 0: Token is active
FLAG_PERSISTENT   = 0x0002  # Bit 1: Should be persisted
FLAG_MUTABLE      = 0x0004  # Bit 2: Can be modified
FLAG_SYNCHRONIZED = 0x0008  # Bit 3: Synchronized
FLAG_COMPRESSED   = 0x0010  # Bit 4: Data compressed
FLAG_ENCRYPTED    = 0x0020  # Bit 5: Data encrypted
FLAG_DIRTY        = 0x0040  # Bit 6: Modified, not saved
FLAG_LOCKED       = 0x0080  # Bit 7: Locked

# Entity type mask (bits 8-11)
ENTITY_TYPE_MASK = 0x0F00

# Entity types
TYPE_UNDEFINED   = 0x0000  # 0000: Undefined
TYPE_OBJECT      = 0x0100  # 0001: Physical object
TYPE_EVENT       = 0x0200  # 0010: Event
TYPE_STATE       = 0x0300  # 0011: State
TYPE_PROCESS     = 0x0400  # 0100: Process
TYPE_CONCEPT     = 0x0500  # 0101: Concept/idea
TYPE_RELATION    = 0x0600  # 0110: Relation
TYPE_PATTERN     = 0x0700  # 0111: Pattern
TYPE_RULE        = 0x0800  # 1000: Rule
TYPE_GOAL        = 0x0900  # 1001: Goal
TYPE_MEMORY      = 0x0A00  # 1010: Memory
TYPE_SENSOR      = 0x0B00  # 1011: Sensor
TYPE_ACTUATOR    = 0x0C00  # 1100: Actuator
TYPE_CONTROLLER  = 0x0D00  # 1101: Controller
TYPE_BUFFER      = 0x0E00  # 1110: Buffer
TYPE_RESERVED    = 0x0F00  # 1111: Reserved

# User flags (bits 12-15)
FLAG_USER_1 = 0x1000
FLAG_USER_2 = 0x2000
FLAG_USER_3 = 0x4000
FLAG_USER_4 = 0x8000


# ═══════════════════════════════════════════════════════
# ID STRUCTURE (32 bits)
# ═══════════════════════════════════════════════════════

def create_token_id(local_id: int, entity_type: int = 0, domain: int = 0) -> int:
    """
    Create token ID from components.

    Args:
        local_id: Local ID (0-16777215)
        entity_type: Entity type (0-15)
        domain: Domain (0-15)

    Returns:
        Complete token ID
    """
    return (domain << 28) | (entity_type << 24) | (local_id & 0xFFFFFF)


def extract_local_id(token_id: int) -> int:
    """Extract local ID from token ID (bits 0-23)"""
    return token_id & 0xFFFFFF


def extract_entity_type(token_id: int) -> int:
    """Extract entity type from token ID (bits 24-27)"""
    return (token_id >> 24) & 0xF


def extract_domain(token_id: int) -> int:
    """Extract domain from token ID (bits 28-31)"""
    return (token_id >> 28) & 0xF


# ═══════════════════════════════════════════════════════
# TOKEN CLASS
# ═══════════════════════════════════════════════════════

@dataclass
class Token:
    """
    Token v2.0 - 64-byte atomic unit of information

    A token exists simultaneously in 8 semantic coordinate spaces,
    with metadata encoded in ID and flags.

    Structure:
        - 48 bytes: coordinates (8 levels × 3 axes × int16)
        - 4 bytes: id (uint32)
        - 2 bytes: flags (uint16)
        - 4 bytes: weight (float32)
        - 1 byte: field_radius (uint8)
        - 1 byte: field_strength (uint8)
        - 4 bytes: timestamp (uint32)
    """

    # Core fields
    id: int = 0
    coordinates: np.ndarray = field(default_factory=lambda: np.full((8, 3), COORD_UNDEFINED, dtype=np.int16))
    flags: int = FLAG_ACTIVE
    weight: float = 0.5
    field_radius: float = 1.0
    field_strength: float = 1.0
    timestamp: int = field(default_factory=lambda: int(time.time()))

    def __post_init__(self):
        """Initialize token after creation"""
        if not isinstance(self.coordinates, np.ndarray):
            self.coordinates = np.full((8, 3), COORD_UNDEFINED, dtype=np.int16)

        if self.timestamp == 0:
            self.timestamp = int(time.time())

    # ═══════════════════════════════════════════════════════
    # COORDINATE OPERATIONS
    # ═══════════════════════════════════════════════════════

    def set_coordinates(self, level: int, x: Optional[float], y: Optional[float], z: Optional[float]) -> None:
        """
        Set coordinates for a specific level.

        Args:
            level: Level (0-7)
            x, y, z: Coordinate values (None for undefined)
        """
        if not 0 <= level < 8:
            raise ValueError(f"Level must be 0-7, got {level}")

        scale = COORDINATE_SCALES[level]

        # L7 (Temporal) has different scale for Z axis
        if level == 6:
            self.coordinates[level] = [
                self._encode_coord(x, 100),
                self._encode_coord(y, 100),
                self._encode_coord(z, 1000)  # Frequency in Hz
            ]
        else:
            self.coordinates[level] = [
                self._encode_coord(x, scale),
                self._encode_coord(y, scale),
                self._encode_coord(z, scale)
            ]

    def get_coordinates(self, level: int) -> Optional[Tuple[float, float, float]]:
        """
        Get coordinates for a specific level.

        Args:
            level: Level (0-7)

        Returns:
            (x, y, z) tuple or None if undefined
        """
        if not 0 <= level < 8:
            raise ValueError(f"Level must be 0-7, got {level}")

        coords = self.coordinates[level]

        # Check if undefined
        if coords[0] == COORD_UNDEFINED:
            return None

        scale = COORDINATE_SCALES[level]

        # L7 has different scale for Z
        if level == 6:
            return (
                self._decode_coord(coords[0], 100),
                self._decode_coord(coords[1], 100),
                self._decode_coord(coords[2], 1000)
            )
        else:
            return tuple(self._decode_coord(c, scale) for c in coords)

    def _encode_coord(self, value: Optional[float], scale: int) -> int:
        """Encode float coordinate to int16"""
        if value is None:
            return COORD_UNDEFINED
        scaled = int(value * scale)
        return max(-32767, min(32767, scaled))

    def _decode_coord(self, encoded: int, scale: int) -> Optional[float]:
        """Decode int16 coordinate to float"""
        if encoded == COORD_UNDEFINED:
            return None
        return encoded / scale

    # ═══════════════════════════════════════════════════════
    # FLAG OPERATIONS
    # ═══════════════════════════════════════════════════════

    def set_flag(self, flag: int) -> None:
        """Set a flag"""
        self.flags |= flag

    def clear_flag(self, flag: int) -> None:
        """Clear a flag"""
        self.flags &= ~flag

    def has_flag(self, flag: int) -> bool:
        """Check if flag is set"""
        return bool(self.flags & flag)

    def get_entity_type(self) -> int:
        """Get entity type from flags (bits 8-11)"""
        return (self.flags & ENTITY_TYPE_MASK) >> 8

    def set_entity_type(self, entity_type: int) -> None:
        """Set entity type in flags"""
        self.flags = (self.flags & ~ENTITY_TYPE_MASK) | (entity_type << 8)

    # ═══════════════════════════════════════════════════════
    # SERIALIZATION
    # ═══════════════════════════════════════════════════════

    def pack(self) -> bytes:
        """
        Serialize token to 64 bytes.

        Returns:
            64-byte binary representation
        """
        # Flatten coordinates (8×3 = 24 int16 values)
        coords_flat = self.coordinates.flatten().tolist()

        # Encode field values
        field_radius_encoded = int(max(0, min(255, self.field_radius * 100)))
        field_strength_encoded = int(max(0, min(255, self.field_strength * 255)))

        # Pack everything
        data = struct.pack(
            PACK_FORMAT,
            *coords_flat,              # 24× int16
            self.id,                   # uint32
            self.flags,                # uint16
            self.weight,               # float32
            field_radius_encoded,      # uint8
            field_strength_encoded,    # uint8
            self.timestamp             # uint32
        )

        assert len(data) == TOKEN_SIZE, f"Token size mismatch: {len(data)} != {TOKEN_SIZE}"
        return data

    @classmethod
    def unpack(cls, data: bytes) -> 'Token':
        """
        Deserialize token from 64 bytes.

        Args:
            data: 64-byte binary data

        Returns:
            Token instance
        """
        if len(data) != TOKEN_SIZE:
            raise ValueError(f"Expected {TOKEN_SIZE} bytes, got {len(data)}")

        # Unpack
        unpacked = struct.unpack(PACK_FORMAT, data)

        # Create token
        token = cls()

        # Coordinates (first 24 values)
        token.coordinates = np.array(unpacked[:24], dtype=np.int16).reshape(8, 3)

        # Other fields
        token.id = unpacked[24]
        token.flags = unpacked[25]
        token.weight = unpacked[26]
        token.field_radius = unpacked[27] / 100.0
        token.field_strength = unpacked[28] / 255.0
        token.timestamp = unpacked[29]

        return token

    # ═══════════════════════════════════════════════════════
    # EXPORT / DICT
    # ═══════════════════════════════════════════════════════

    def to_dict(self) -> Dict[str, Any]:
        """Export token to dictionary"""
        return {
            'id': self.id,
            'id_components': {
                'local_id': extract_local_id(self.id),
                'entity_type': extract_entity_type(self.id),
                'domain': extract_domain(self.id)
            },
            'coordinates': {
                f'L{i+1}_{COORDINATE_SPACES[i]}': self.get_coordinates(i)
                for i in range(8)
            },
            'flags': {
                'value': self.flags,
                'active': self.has_flag(FLAG_ACTIVE),
                'persistent': self.has_flag(FLAG_PERSISTENT),
                'mutable': self.has_flag(FLAG_MUTABLE),
                'dirty': self.has_flag(FLAG_DIRTY),
                'entity_type': self.get_entity_type()
            },
            'weight': self.weight,
            'field': {
                'radius': self.field_radius,
                'strength': self.field_strength
            },
            'timestamp': self.timestamp,
            'age_seconds': int(time.time()) - self.timestamp
        }

    def __repr__(self) -> str:
        """String representation"""
        entity_type = self.get_entity_type()
        active_levels = sum(1 for i in range(8) if self.get_coordinates(i) is not None)
        return (
            f"Token(id=0x{self.id:08X}, "
            f"type={entity_type}, "
            f"levels={active_levels}/8, "
            f"weight={self.weight:.2f}, "
            f"age={int(time.time()) - self.timestamp}s)"
        )


# ═══════════════════════════════════════════════════════
# VALIDATION
# ═══════════════════════════════════════════════════════

def validate_token(token: Token) -> bool:
    """
    Validate token structure.

    Returns:
        True if valid
    """
    # ID range
    if not (0 <= token.id <= 0xFFFFFFFF):
        return False

    # Coordinates range
    for level in range(8):
        coords = token.coordinates[level]
        if not all(-32767 <= c <= 32767 for c in coords):
            return False

    # Weight range
    if not (0.0 <= token.weight <= 1.0):
        return False

    # Field radius range
    if not (0.0 <= token.field_radius <= 2.55):
        return False

    # Field strength range
    if not (0.0 <= token.field_strength <= 1.0):
        return False

    # Timestamp range
    if not (0 <= token.timestamp <= 0xFFFFFFFF):
        return False

    return True
