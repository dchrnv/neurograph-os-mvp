"""
ActionSelector - Chooses which actions to execute based on context.

Implements Hot Path / Cold Path routing:
- Hot Path: Immediate, synchronous actions (e.g., chat responses)
- Cold Path: Deferred, background actions (e.g., analytics, logging)
"""

from enum import Enum
from typing import List, Dict, Any, Optional, Callable
from dataclasses import dataclass
import logging

from .base import Action, ActionPriority


logger = logging.getLogger(__name__)


class PathType(Enum):
    """Execution path for actions."""
    HOT = "hot"      # Immediate, synchronous execution
    COLD = "cold"    # Deferred, background execution
    BOTH = "both"    # Can execute on either path


@dataclass
class SelectionRule:
    """
    Rule for selecting actions.

    Attributes:
        name: Rule name
        condition: Callable that returns True if rule applies
        action_types: Action types to select
        path: Execution path
        priority_boost: Boost priority by this amount
    """
    name: str
    condition: Callable[[Dict[str, Any]], bool]
    action_types: List[str]
    path: PathType = PathType.BOTH
    priority_boost: int = 0


class ActionSelector:
    """
    Selects which actions to execute based on processing results.

    Responsibilities:
    1. Analyze ProcessingResult from SignalSystem
    2. Determine which actions should be triggered
    3. Route actions to Hot Path (immediate) or Cold Path (background)
    4. Apply selection rules and filters
    """

    def __init__(self):
        """Initialize selector."""
        self._rules: List[SelectionRule] = []
        self._default_hot_path_actions: List[str] = []
        self._default_cold_path_actions: List[str] = []
        logger.info("ActionSelector initialized")

    def add_rule(
        self,
        name: str,
        condition: Callable[[Dict[str, Any]], bool],
        action_types: List[str],
        path: PathType = PathType.BOTH,
        priority_boost: int = 0
    ):
        """
        Add a selection rule.

        Args:
            name: Rule name
            condition: Function that takes context and returns bool
            action_types: Actions to select if condition is true
            path: Execution path (hot/cold/both)
            priority_boost: Boost priority by this amount
        """
        rule = SelectionRule(
            name=name,
            condition=condition,
            action_types=action_types,
            path=path,
            priority_boost=priority_boost
        )
        self._rules.append(rule)
        logger.info(f"Added selection rule: {name} (path={path.value})")

    def remove_rule(self, name: str) -> bool:
        """Remove a rule by name."""
        for i, rule in enumerate(self._rules):
            if rule.name == name:
                del self._rules[i]
                logger.info(f"Removed selection rule: {name}")
                return True
        return False

    def set_default_hot_path(self, action_types: List[str]):
        """Set default actions for hot path."""
        self._default_hot_path_actions = action_types
        logger.info(f"Default hot path actions: {action_types}")

    def set_default_cold_path(self, action_types: List[str]):
        """Set default actions for cold path."""
        self._default_cold_path_actions = action_types
        logger.info(f"Default cold path actions: {action_types}")

    def select(
        self,
        context: Dict[str, Any]
    ) -> Dict[PathType, List[Dict[str, Any]]]:
        """
        Select actions based on context.

        Args:
            context: Execution context containing:
                - signal_event: SignalEvent that triggered processing
                - processing_result: ProcessingResult from SignalSystem
                - metadata: Additional context

        Returns:
            Dictionary mapping PathType to list of action specifications:
            {
                PathType.HOT: [
                    {
                        "action_type": "text_response",
                        "priority": ActionPriority.HIGH,
                        "config": {...}
                    },
                    ...
                ],
                PathType.COLD: [...]
            }
        """
        selected = {
            PathType.HOT: [],
            PathType.COLD: []
        }

        # Apply rules
        for rule in self._rules:
            try:
                if rule.condition(context):
                    logger.debug(f"Rule '{rule.name}' matched")

                    for action_type in rule.action_types:
                        spec = {
                            "action_type": action_type,
                            "rule": rule.name,
                            "priority_boost": rule.priority_boost
                        }

                        # Route to appropriate path
                        if rule.path == PathType.HOT:
                            selected[PathType.HOT].append(spec)
                        elif rule.path == PathType.COLD:
                            selected[PathType.COLD].append(spec)
                        elif rule.path == PathType.BOTH:
                            # Check if it's a hot or cold action by default
                            if action_type in self._default_hot_path_actions:
                                selected[PathType.HOT].append(spec)
                            elif action_type in self._default_cold_path_actions:
                                selected[PathType.COLD].append(spec)
                            else:
                                # Default to cold path if unknown
                                selected[PathType.COLD].append(spec)

            except Exception as e:
                logger.error(f"Error evaluating rule '{rule.name}': {e}")

        # Add defaults if no rules matched
        if not selected[PathType.HOT] and not selected[PathType.COLD]:
            logger.debug("No rules matched, using defaults")

            # Check processing result for hints
            processing_result = context.get("processing_result", {})
            is_novel = processing_result.get("is_novel", False)
            triggered_actions = processing_result.get("triggered_actions", [])

            # If novel or has triggered actions, use hot path
            if is_novel or triggered_actions:
                for action_type in self._default_hot_path_actions:
                    selected[PathType.HOT].append({
                        "action_type": action_type,
                        "rule": "default_hot"
                    })

            # Always add cold path defaults
            for action_type in self._default_cold_path_actions:
                selected[PathType.COLD].append({
                    "action_type": action_type,
                    "rule": "default_cold"
                })

        logger.info(f"Selected {len(selected[PathType.HOT])} hot, {len(selected[PathType.COLD])} cold actions")
        return selected

    def get_stats(self) -> Dict[str, Any]:
        """Get selector statistics."""
        return {
            "total_rules": len(self._rules),
            "default_hot_actions": len(self._default_hot_path_actions),
            "default_cold_actions": len(self._default_cold_path_actions),
        }


# Pre-built selection rules

def is_high_priority(context: Dict[str, Any]) -> bool:
    """Check if signal has high priority."""
    event = context.get("signal_event")
    if event:
        priority = getattr(event.routing, "priority", 0)
        return priority >= 200
    return False


def is_novel(context: Dict[str, Any]) -> bool:
    """Check if signal is novel."""
    result = context.get("processing_result", {})
    return result.get("is_novel", False)


def has_triggered_actions(context: Dict[str, Any]) -> bool:
    """Check if any actions were triggered."""
    result = context.get("processing_result", {})
    return len(result.get("triggered_actions", [])) > 0


def is_user_message(context: Dict[str, Any]) -> bool:
    """Check if signal is from user (Telegram, chat, etc.)."""
    event = context.get("signal_event")
    if event:
        tags = getattr(event.routing, "tags", [])
        return "user_message" in tags or "telegram" in tags
    return False


def is_system_event(context: Dict[str, Any]) -> bool:
    """Check if signal is system event."""
    event = context.get("signal_event")
    if event:
        domain = getattr(event.source, "domain", "")
        return domain == "internal"
    return False
