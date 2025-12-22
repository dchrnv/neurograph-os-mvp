"""
ActionController - Response generation and action execution.

The ActionController receives ProcessingResult from SignalSystem
and generates appropriate responses/actions.

Architecture:
- ActionController: Main orchestrator
- ActionSelector: Chooses which actions to execute
- ActionRegistry: Manages available actions
- Action Executors: Implement specific action types

Flow:
1. SignalSystem processes event → ProcessingResult (with triggered_actions)
2. ActionController receives ProcessingResult
3. ActionSelector decides which actions to execute (hot/cold path)
4. Action executors perform the actions
5. Results returned to caller
"""

from .controller import ActionController
from .selector import ActionSelector, PathType
from .registry import ActionRegistry, ActionConfig
from .base import Action, ActionResult, ActionPriority, ActionStatus

__all__ = [
    "ActionController",
    "ActionSelector",
    "PathType",
    "ActionRegistry",
    "ActionConfig",
    "Action",
    "ActionResult",
    "ActionPriority",
    "ActionStatus",
]
