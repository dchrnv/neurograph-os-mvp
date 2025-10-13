"""
Предустановленные профили CDNA для разных режимов системы.
"""

from .cdna_format import CDNABinaryFormat

def get_default_cdna_profile() -> CDNABinaryFormat:
    """Создаёт стандартный профиль CDNA (базовая конфигурация)."""
    profile_data = {
        "GRID_PHYSICS": {
            "semantic_ids": list(range(8)),
            "flags": [1, 1, 1, 1, 0, 0, 0, 0],
            "scales": [1.0, 1.0, 0.8, 0.8, 0.5, 0.5, 0.25, 0.25],
        },
        "TOKEN_PROPERTIES": {
            "values": [
                -1.0, 1.0,     # weight range
                0xFFFF, 8, 0, 0,  # base flags, levels, reserved
                0, 0, 0, 0,      # padding
                0, 0, 0, 0, 0, 0, 0, 0
            ],
        },
        "META": {"signature": b"NGOS-CDNA-DEFAULT"},
    }
    return CDNABinaryFormat.from_dict(profile_data)


def get_rl_cdna_profile() -> CDNABinaryFormat:
    """Создаёт профиль CDNA, оптимизированный для RL-агентов."""
    profile_data = {
        "GRID_PHYSICS": {
            "semantic_ids": list(range(8)),
            "flags": [1, 1, 1, 1, 0, 0, 0, 0],
            "scales": [1.0, 1.0, 0.9, 0.9, 0.7, 0.7, 0.5, 0.5],
        },
        "TOKEN_PROPERTIES": {
            "values": [
                -1.0, 1.0,     # weight range
                0xFFFF, 8, 0, 0,  # base flags, levels, reserved
                0, 0, 0, 0,      # padding
                0, 0, 0, 0, 0, 0, 0, 0
            ],
        },
        "META": {"signature": b"NGOS-CDNA-RL"},
    }
    return CDNABinaryFormat.from_dict(profile_data)