"""
Base classes and types for ActionController.
"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from enum import Enum
from typing import Any, Dict, Optional
from datetime import datetime


class ActionPriority(Enum):
    """Action execution priority levels."""
    CRITICAL = 1000  # Immediate execution required
    HIGH = 500       # Execute as soon as possible
    NORMAL = 200     # Standard priority
    LOW = 100        # Can be delayed
    BACKGROUND = 50  # Execute when idle


class ActionStatus(Enum):
    """Action execution status."""
    PENDING = "pending"
    EXECUTING = "executing"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"


@dataclass
class ActionResult:
    """
    Result of action execution.

    Attributes:
        action_id: Unique action identifier
        status: Execution status
        success: Whether action succeeded
        data: Result data (optional)
        error: Error message if failed
        execution_time_ms: Time taken to execute
        metadata: Additional metadata
    """
    action_id: str
    status: ActionStatus
    success: bool
    data: Optional[Any] = None
    error: Optional[str] = None
    execution_time_ms: float = 0.0
    metadata: Dict[str, Any] = None

    def __post_init__(self):
        if self.metadata is None:
            self.metadata = {}


class Action(ABC):
    """
    Base class for all actions.

    An Action is a unit of work that can be executed in response
    to a triggered activation in the signal processing pipeline.

    Actions can be:
    - Text responses (Telegram, chat)
    - System operations (logging, metrics)
    - External API calls
    - State updates
    """

    def __init__(
        self,
        action_id: str,
        action_type: str,
        priority: ActionPriority = ActionPriority.NORMAL,
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Initialize action.

        Args:
            action_id: Unique identifier for this action
            action_type: Type of action (e.g., "text_response", "telegram_send")
            priority: Execution priority
            metadata: Additional metadata
        """
        self.action_id = action_id
        self.action_type = action_type
        self.priority = priority
        self.metadata = metadata or {}
        self.created_at = datetime.now()
        self.status = ActionStatus.PENDING

    @abstractmethod
    async def execute(self, context: Dict[str, Any]) -> ActionResult:
        """
        Execute the action.

        Args:
            context: Execution context (includes SignalEvent, ProcessingResult, etc.)

        Returns:
            ActionResult with execution status and data
        """
        pass

    @abstractmethod
    def can_execute(self, context: Dict[str, Any]) -> bool:
        """
        Check if action can be executed given the context.

        Args:
            context: Execution context

        Returns:
            True if action can execute
        """
        pass

    def __repr__(self) -> str:
        return f"Action(id={self.action_id}, type={self.action_type}, priority={self.priority.name})"
