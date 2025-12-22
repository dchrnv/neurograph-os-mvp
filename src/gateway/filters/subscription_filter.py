"""
SubscriptionFilter - MongoDB-style event filtering

Provides flexible filtering for SignalEvent subscriptions using
MongoDB-like query operators.

Supported Operators:
- Comparison: $eq, $ne, $gt, $gte, $lt, $lte
- Collection: $in, $nin, $contains
- Pattern: $regex, $wildcard
- Logical: $and, $or, $not
"""

import re
from typing import Any, Dict, List, Union
from ..models import SignalEvent


class SubscriptionFilter:
    """
    MongoDB-style filter for SignalEvent subscriptions.

    Usage:
        # Simple equality
        filter = SubscriptionFilter({
            "event_type": "signal.input.external.text.chat"
        })

        # Wildcard pattern
        filter = SubscriptionFilter({
            "event_type": {"$wildcard": "signal.input.*"}
        })

        # Numeric comparison
        filter = SubscriptionFilter({
            "routing.priority": {"$gte": 150}
        })

        # Logical operators
        filter = SubscriptionFilter({
            "$and": [
                {"event_type": {"$wildcard": "signal.input.*"}},
                {"routing.priority": {"$gte": 150}}
            ]
        })

        # Check if event matches
        if filter.matches(event):
            print("Event matched!")
    """

    def __init__(self, filter_dict: Dict[str, Any]):
        """
        Create filter from dict.

        Args:
            filter_dict: MongoDB-style filter specification
        """
        self.filter_dict = filter_dict
        self._compiled_patterns = {}  # Cache for compiled regex/wildcards

    def matches(self, event: SignalEvent) -> bool:
        """
        Check if event matches this filter.

        Args:
            event: SignalEvent to test

        Returns:
            True if event matches filter
        """
        return self._evaluate_condition(self.filter_dict, event)

    # ═══════════════════════════════════════════════════════════════════════════════
    # INTERNAL: Condition Evaluation
    # ═══════════════════════════════════════════════════════════════════════════════

    def _evaluate_condition(self, condition: Any, event: SignalEvent) -> bool:
        """
        Recursively evaluate condition against event.

        Args:
            condition: Filter condition (dict, list, or value)
            event: Event to test

        Returns:
            True if condition matches
        """
        # Handle logical operators at top level
        if isinstance(condition, dict):
            # Check for logical operators
            if "$and" in condition:
                return self._evaluate_and(condition["$and"], event)
            elif "$or" in condition:
                return self._evaluate_or(condition["$or"], event)
            elif "$not" in condition:
                return not self._evaluate_condition(condition["$not"], event)

            # Field conditions
            return all(
                self._evaluate_field(field, value_condition, event)
                for field, value_condition in condition.items()
            )

        return False

    def _evaluate_field(
        self,
        field_path: str,
        value_condition: Any,
        event: SignalEvent
    ) -> bool:
        """
        Evaluate condition for a specific field.

        Args:
            field_path: Dot-separated field path (e.g. "routing.priority")
            value_condition: Condition to test (value or operator dict)
            event: Event to test

        Returns:
            True if field matches condition
        """
        # Extract field value from event
        field_value = self._get_field_value(event, field_path)

        # Simple equality check
        if not isinstance(value_condition, dict):
            return field_value == value_condition

        # Operator-based conditions
        for operator, operand in value_condition.items():
            if not self._evaluate_operator(operator, field_value, operand):
                return False

        return True

    # ═══════════════════════════════════════════════════════════════════════════════
    # OPERATORS
    # ═══════════════════════════════════════════════════════════════════════════════

    def _evaluate_operator(
        self,
        operator: str,
        field_value: Any,
        operand: Any
    ) -> bool:
        """
        Evaluate operator condition.

        Args:
            operator: Operator name (e.g. "$eq", "$gt")
            field_value: Actual field value from event
            operand: Expected value/pattern

        Returns:
            True if operator condition matches
        """
        # Comparison operators
        if operator == "$eq":
            return field_value == operand
        elif operator == "$ne":
            return field_value != operand
        elif operator == "$gt":
            return field_value > operand
        elif operator == "$gte":
            return field_value >= operand
        elif operator == "$lt":
            return field_value < operand
        elif operator == "$lte":
            return field_value <= operand

        # Collection operators
        elif operator == "$in":
            return field_value in operand
        elif operator == "$nin":
            return field_value not in operand
        elif operator == "$contains":
            # Check if field_value contains operand
            if isinstance(field_value, (list, tuple)):
                return operand in field_value
            elif isinstance(field_value, str):
                return operand in field_value
            return False

        # Pattern operators
        elif operator == "$regex":
            return self._match_regex(field_value, operand)
        elif operator == "$wildcard":
            return self._match_wildcard(field_value, operand)

        # Unknown operator
        else:
            raise ValueError(f"Unknown operator: {operator}")

    # ═══════════════════════════════════════════════════════════════════════════════
    # LOGICAL OPERATORS
    # ═══════════════════════════════════════════════════════════════════════════════

    def _evaluate_and(self, conditions: List[Dict], event: SignalEvent) -> bool:
        """Evaluate $and operator."""
        return all(
            self._evaluate_condition(cond, event)
            for cond in conditions
        )

    def _evaluate_or(self, conditions: List[Dict], event: SignalEvent) -> bool:
        """Evaluate $or operator."""
        return any(
            self._evaluate_condition(cond, event)
            for cond in conditions
        )

    # ═══════════════════════════════════════════════════════════════════════════════
    # PATTERN MATCHING
    # ═══════════════════════════════════════════════════════════════════════════════

    def _match_regex(self, value: Any, pattern: str) -> bool:
        """
        Match value against regex pattern.

        Args:
            value: Value to test
            pattern: Regex pattern

        Returns:
            True if matches
        """
        if not isinstance(value, str):
            return False

        # Compile and cache pattern
        if pattern not in self._compiled_patterns:
            self._compiled_patterns[pattern] = re.compile(pattern)

        regex = self._compiled_patterns[pattern]
        return regex.search(value) is not None

    def _match_wildcard(self, value: Any, pattern: str) -> bool:
        """
        Match value against wildcard pattern.

        Wildcards:
        - * matches any sequence (e.g. "signal.*" matches "signal.input.text")
        - ? matches single character

        Args:
            value: Value to test
            pattern: Wildcard pattern

        Returns:
            True if matches
        """
        if not isinstance(value, str):
            return False

        # Convert wildcard to regex
        # Cache the converted pattern
        cache_key = f"wildcard:{pattern}"
        if cache_key not in self._compiled_patterns:
            # Escape regex special chars except * and ?
            regex_pattern = re.escape(pattern)
            # Replace escaped wildcards with regex equivalents
            regex_pattern = regex_pattern.replace(r'\*', '.*')
            regex_pattern = regex_pattern.replace(r'\?', '.')
            # Anchor to match full string
            regex_pattern = f"^{regex_pattern}$"
            self._compiled_patterns[cache_key] = re.compile(regex_pattern)

        regex = self._compiled_patterns[cache_key]
        return regex.match(value) is not None

    # ═══════════════════════════════════════════════════════════════════════════════
    # FIELD ACCESS
    # ═══════════════════════════════════════════════════════════════════════════════

    def _get_field_value(self, event: SignalEvent, field_path: str) -> Any:
        """
        Extract field value from event using dot-separated path.

        Args:
            event: SignalEvent
            field_path: Dot-separated path (e.g. "routing.priority")

        Returns:
            Field value or None if not found
        """
        parts = field_path.split(".")
        current = event

        for part in parts:
            if hasattr(current, part):
                current = getattr(current, part)
            elif isinstance(current, dict) and part in current:
                current = current[part]
            else:
                return None

        return current

    # ═══════════════════════════════════════════════════════════════════════════════
    # UTILITIES
    # ═══════════════════════════════════════════════════════════════════════════════

    def __repr__(self) -> str:
        """String representation."""
        return f"SubscriptionFilter({self.filter_dict})"


__all__ = [
    "SubscriptionFilter",
]
