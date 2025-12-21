"""
Signal Gateway v2.0 - Sensory Interface for NeuroGraph OS

Gateway serves as the sensory layer, translating external signals
into the internal SignalSystem event format.

Architecture:
    External World → Gateway (Python) → SignalSystem (Rust Core)
"""

from .models import (
    SignalEvent,
    SignalSource,
    SemanticCore,
    EnergyProfile,
    TemporalBinding,
    RawPayload,
    ProcessingResult,
    RoutingInfo,
)
from .registry import SensorRegistry, SensorConfig
from .encoders import EncoderType
from .gateway import SignalGateway

__version__ = "2.0.0"
__all__ = [
    # Main Gateway
    "SignalGateway",

    # Models
    "SignalEvent",
    "SignalSource",
    "SemanticCore",
    "EnergyProfile",
    "TemporalBinding",
    "RawPayload",
    "ProcessingResult",
    "RoutingInfo",

    # Registry
    "SensorRegistry",
    "SensorConfig",

    # Encoders
    "EncoderType",
]
