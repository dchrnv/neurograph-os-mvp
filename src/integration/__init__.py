"""
Integration Layer - Connects Gateway, Core, and ActionController.

This layer orchestrates the full signal processing pipeline:
1. Gateway receives signal → creates SignalEvent
2. Core processes signal → returns ProcessingResult
3. ActionController executes actions → generates responses

Flow:
    Signal → Gateway → Core → ActionController → Response
"""

from .pipeline import SignalPipeline

__all__ = ["SignalPipeline"]
