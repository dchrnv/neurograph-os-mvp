"""
Filter Examples - common subscription filter patterns

Demonstrates typical filter configurations for different use cases:
- Telegram bot filters
- Dashboard priority filters
- Action selector filters
- Anomaly detector filters
- System monitoring filters
"""

from .subscription_filter import SubscriptionFilter


# ═══════════════════════════════════════════════════════════════════════════════
# TELEGRAM BOT FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def telegram_user_messages_filter():
    """
    Filter for Telegram user messages only.

    Matches:
    - External text messages
    - From text_chat sensor
    - Priority >= 150
    """
    return SubscriptionFilter({
        "$and": [
            {"event_type": {"$wildcard": "signal.input.external.text.*"}},
            {"source.sensor_type": "text_chat"},
            {"routing.priority": {"$gte": 150}}
        ]
    })


def telegram_high_priority_filter():
    """
    Filter for high-priority Telegram messages requiring immediate response.

    Matches:
    - Telegram messages
    - Priority >= 200
    - High urgency
    """
    return SubscriptionFilter({
        "$and": [
            {"source.sensor_type": "text_chat"},
            {"routing.priority": {"$gte": 200}},
            {"energy.urgency": {"$gte": 0.7}}
        ]
    })


def telegram_conversation_filter(sequence_id: str):
    """
    Filter for specific conversation thread.

    Args:
        sequence_id: Conversation ID to track

    Returns:
        Filter matching all events in this conversation
    """
    return SubscriptionFilter({
        "temporal.sequence_id": sequence_id
    })


# ═══════════════════════════════════════════════════════════════════════════════
# DASHBOARD FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def dashboard_all_events_filter():
    """
    Filter for dashboard showing all events.

    Matches:
    - All signal.input.* events
    """
    return SubscriptionFilter({
        "event_type": {"$wildcard": "signal.input.*"}
    })


def dashboard_external_only_filter():
    """
    Filter for dashboard showing only external inputs.

    Matches:
    - Domain: external
    """
    return SubscriptionFilter({
        "source.domain": "external"
    })


def dashboard_high_priority_filter():
    """
    Filter for dashboard showing high-priority events.

    Matches:
    - Priority >= 180
    """
    return SubscriptionFilter({
        "routing.priority": {"$gte": 180}
    })


# ═══════════════════════════════════════════════════════════════════════════════
# ACTION SELECTOR FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def action_selector_novel_signals_filter():
    """
    Filter for ActionSelector - novel signals requiring action.

    Matches:
    - Novel signals (is_novel = True in result)
    - Priority >= 150
    - External domain
    """
    return SubscriptionFilter({
        "$and": [
            {"result.is_novel": True},
            {"routing.priority": {"$gte": 150}},
            {"source.domain": "external"}
        ]
    })


def action_selector_triggered_actions_filter():
    """
    Filter for events that triggered actions.

    Matches:
    - Events with non-empty triggered_actions
    """
    return SubscriptionFilter({
        "result.triggered_actions": {"$ne": []}
    })


# ═══════════════════════════════════════════════════════════════════════════════
# ANOMALY DETECTOR FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def anomaly_detector_high_score_filter():
    """
    Filter for anomaly detector - high anomaly scores.

    Matches:
    - Anomaly score >= 0.7
    """
    return SubscriptionFilter({
        "result.anomaly_score": {"$gte": 0.7}
    })


def anomaly_detector_system_events_filter():
    """
    Filter for system anomalies.

    Matches:
    - System domain events
    - Anomaly score >= 0.5
    """
    return SubscriptionFilter({
        "$and": [
            {"source.domain": "system"},
            {"result.anomaly_score": {"$gte": 0.5}}
        ]
    })


# ═══════════════════════════════════════════════════════════════════════════════
# SYSTEM MONITORING FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def system_monitor_resource_alerts_filter():
    """
    Filter for resource alert events.

    Matches:
    - System domain
    - sensor_type: system_monitor
    - Priority >= 150 (alerts)
    """
    return SubscriptionFilter({
        "$and": [
            {"source.domain": "system"},
            {"source.sensor_type": "system_monitor"},
            {"routing.priority": {"$gte": 150}}
        ]
    })


def system_monitor_critical_filter():
    """
    Filter for critical system events.

    Matches:
    - System domain
    - Priority >= 200 (critical)
    - High urgency
    """
    return SubscriptionFilter({
        "$and": [
            {"source.domain": "system"},
            {"routing.priority": {"$gte": 200}},
            {"energy.urgency": {"$gte": 0.8}}
        ]
    })


# ═══════════════════════════════════════════════════════════════════════════════
# SENTIMENT FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def sentiment_positive_filter():
    """
    Filter for positive sentiment messages.

    Matches:
    - Polarity > 0.6 (positive)
    - Text modality
    """
    return SubscriptionFilter({
        "$and": [
            {"source.modality": "text"},
            {"semantic.vector.0": {"$gt": 0.6}}  # Polarity dimension
        ]
    })


def sentiment_negative_filter():
    """
    Filter for negative sentiment messages.

    Matches:
    - Polarity < 0.4 (negative)
    - Text modality
    """
    return SubscriptionFilter({
        "$and": [
            {"source.modality": "text"},
            {"semantic.vector.0": {"$lt": 0.4}}  # Polarity dimension
        ]
    })


# ═══════════════════════════════════════════════════════════════════════════════
# TAG-BASED FILTERS
# ═══════════════════════════════════════════════════════════════════════════════

def tag_contains_filter(tag: str):
    """
    Filter for events containing specific tag.

    Args:
        tag: Tag to search for

    Returns:
        Filter matching events with this tag
    """
    return SubscriptionFilter({
        "routing.tags": {"$contains": tag}
    })


def multi_tag_filter(tags: list):
    """
    Filter for events containing ANY of the specified tags.

    Args:
        tags: List of tags to match

    Returns:
        Filter matching events with any of these tags
    """
    return SubscriptionFilter({
        "$or": [
            {"routing.tags": {"$contains": tag}}
            for tag in tags
        ]
    })


# ═══════════════════════════════════════════════════════════════════════════════
# EXPORT ALL EXAMPLES
# ═══════════════════════════════════════════════════════════════════════════════

__all__ = [
    # Telegram
    "telegram_user_messages_filter",
    "telegram_high_priority_filter",
    "telegram_conversation_filter",

    # Dashboard
    "dashboard_all_events_filter",
    "dashboard_external_only_filter",
    "dashboard_high_priority_filter",

    # Action Selector
    "action_selector_novel_signals_filter",
    "action_selector_triggered_actions_filter",

    # Anomaly Detector
    "anomaly_detector_high_score_filter",
    "anomaly_detector_system_events_filter",

    # System Monitoring
    "system_monitor_resource_alerts_filter",
    "system_monitor_critical_filter",

    # Sentiment
    "sentiment_positive_filter",
    "sentiment_negative_filter",

    # Tags
    "tag_contains_filter",
    "multi_tag_filter",
]
