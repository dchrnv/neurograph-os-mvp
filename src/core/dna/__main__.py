"""
Запуск и инициализация DNA системы.
"""

from .binary.profiles import get_default_cdna_profile
from .guardian.guardian import DNAGuardian


def create_enhanced_dna_system():
    """Создаёт DNAGuardian с базовым CDNA профилем."""
    cdna = get_default_cdna_profile()
    guardian = DNAGuardian(cdna)
    print("[DNA] Enhanced DNA System initialized.")
    return guardian
