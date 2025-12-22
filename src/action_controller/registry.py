"""
ActionRegistry - Manages available actions and their configurations.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Optional, Type, Callable, Any
import logging

from .base import Action, ActionPriority


logger = logging.getLogger(__name__)


@dataclass
class ActionConfig:
    """
    Configuration for a registered action.

    Attributes:
        action_type: Type identifier (e.g., "text_response")
        action_class: Class that implements this action
        enabled: Whether action is currently enabled
        priority: Default priority for this action type
        config: Type-specific configuration
        tags: Tags for filtering/categorization
    """
    action_type: str
    action_class: Type[Action]
    enabled: bool = True
    priority: ActionPriority = ActionPriority.NORMAL
    config: Dict[str, Any] = field(default_factory=dict)
    tags: List[str] = field(default_factory=list)


class ActionRegistry:
    """
    Registry for available action types.

    Manages registration, lookup, and instantiation of actions.
    """

    def __init__(self):
        """Initialize registry."""
        self._actions: Dict[str, ActionConfig] = {}
        self._tags_index: Dict[str, List[str]] = {}  # tag -> [action_types]
        logger.info("ActionRegistry initialized")

    def register(
        self,
        action_type: str,
        action_class: Type[Action],
        enabled: bool = True,
        priority: ActionPriority = ActionPriority.NORMAL,
        config: Optional[Dict[str, Any]] = None,
        tags: Optional[List[str]] = None
    ) -> bool:
        """
        Register a new action type.

        Args:
            action_type: Unique type identifier
            action_class: Class implementing the action
            enabled: Whether action is enabled
            priority: Default priority
            config: Type-specific configuration
            tags: Tags for categorization

        Returns:
            True if registered successfully
        """
        if action_type in self._actions:
            logger.warning(f"Action type '{action_type}' already registered, overwriting")

        config_obj = ActionConfig(
            action_type=action_type,
            action_class=action_class,
            enabled=enabled,
            priority=priority,
            config=config or {},
            tags=tags or []
        )

        self._actions[action_type] = config_obj

        # Update tags index
        for tag in config_obj.tags:
            if tag not in self._tags_index:
                self._tags_index[tag] = []
            if action_type not in self._tags_index[tag]:
                self._tags_index[tag].append(action_type)

        logger.info(f"Registered action type: {action_type} (enabled={enabled}, priority={priority.name})")
        return True

    def unregister(self, action_type: str) -> bool:
        """
        Unregister an action type.

        Args:
            action_type: Type to unregister

        Returns:
            True if unregistered successfully
        """
        if action_type not in self._actions:
            logger.warning(f"Action type '{action_type}' not found")
            return False

        config = self._actions[action_type]

        # Remove from tags index
        for tag in config.tags:
            if tag in self._tags_index and action_type in self._tags_index[tag]:
                self._tags_index[tag].remove(action_type)
                if not self._tags_index[tag]:
                    del self._tags_index[tag]

        del self._actions[action_type]
        logger.info(f"Unregistered action type: {action_type}")
        return True

    def get_config(self, action_type: str) -> Optional[ActionConfig]:
        """
        Get configuration for an action type.

        Args:
            action_type: Type to get config for

        Returns:
            ActionConfig or None if not found
        """
        return self._actions.get(action_type)

    def create_action(
        self,
        action_type: str,
        action_id: str,
        **kwargs
    ) -> Optional[Action]:
        """
        Create an instance of an action.

        Args:
            action_type: Type of action to create
            action_id: Unique ID for the action instance
            **kwargs: Additional arguments for action constructor

        Returns:
            Action instance or None if type not found/disabled
        """
        config = self.get_config(action_type)

        if not config:
            logger.error(f"Action type '{action_type}' not found")
            return None

        if not config.enabled:
            logger.warning(f"Action type '{action_type}' is disabled")
            return None

        try:
            # Merge default config with instance kwargs
            merged_kwargs = {**config.config, **kwargs}

            # Extract priority separately to avoid duplicate
            priority = merged_kwargs.pop('priority', config.priority)

            action = config.action_class(
                action_id=action_id,
                action_type=action_type,
                priority=priority,
                **merged_kwargs
            )

            logger.debug(f"Created action: {action_id} (type={action_type})")
            return action

        except Exception as e:
            logger.error(f"Failed to create action {action_id}: {e}")
            return None

    def list_actions(
        self,
        enabled_only: bool = True,
        tags: Optional[List[str]] = None
    ) -> List[str]:
        """
        List available action types.

        Args:
            enabled_only: Only return enabled actions
            tags: Filter by tags (if provided)

        Returns:
            List of action type identifiers
        """
        actions = []

        # Filter by tags first
        if tags:
            candidates = set()
            for tag in tags:
                if tag in self._tags_index:
                    candidates.update(self._tags_index[tag])
        else:
            candidates = set(self._actions.keys())

        # Filter by enabled status
        for action_type in candidates:
            config = self._actions[action_type]
            if not enabled_only or config.enabled:
                actions.append(action_type)

        return sorted(actions)

    def enable_action(self, action_type: str) -> bool:
        """Enable an action type."""
        config = self.get_config(action_type)
        if config:
            config.enabled = True
            logger.info(f"Enabled action type: {action_type}")
            return True
        return False

    def disable_action(self, action_type: str) -> bool:
        """Disable an action type."""
        config = self.get_config(action_type)
        if config:
            config.enabled = False
            logger.info(f"Disabled action type: {action_type}")
            return True
        return False

    def get_stats(self) -> Dict[str, Any]:
        """
        Get registry statistics.

        Returns:
            Dictionary with registry stats
        """
        enabled_count = sum(1 for c in self._actions.values() if c.enabled)

        return {
            "total_actions": len(self._actions),
            "enabled_actions": enabled_count,
            "disabled_actions": len(self._actions) - enabled_count,
            "total_tags": len(self._tags_index),
        }
