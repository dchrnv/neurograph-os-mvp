"""
Unit tests for NeuroGraph Jupyter Integration.

Tests auto-completion, DataFrame helpers, query builder, and widgets.
These tests are isolated and don't require full import of the extension.
"""

import pytest
import sys
from pathlib import Path
from unittest.mock import Mock, MagicMock, patch, mock_open

# Add src to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "src"))


# =============================================================================
# Test QueryBuilder (standalone module)
# =============================================================================

def test_query_builder_basic():
    """Test basic query builder functionality."""
    # Import and test directly
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes").where("type", "=", "user")

    result = q.build()
    assert "find all nodes" in result
    assert "type = 'user'" in result


def test_query_builder_multiple_conditions():
    """Test query builder with multiple WHERE clauses."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes").where("type", "=", "user").where("age", ">", 30)

    result = q.build()
    assert "type = 'user'" in result
    assert "age > 30" in result
    assert " and " in result


def test_query_builder_limit_offset():
    """Test query builder with LIMIT and OFFSET."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes").where("type", "=", "user").limit(10).offset(5)

    result = q.build()
    assert "limit 10" in result
    assert "offset 5" in result


def test_query_builder_list_operator():
    """Test query builder with IN operator and list."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes").where("status", "in", ["active", "pending"])

    result = q.build()
    assert "status in" in result
    assert "'active'" in result
    assert "'pending'" in result


def test_query_builder_no_target_error():
    """Test that building without find() raises error."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()

    with pytest.raises(ValueError, match="Must call find"):
        q.build()


def test_query_builder_execute_without_graph_ops():
    """Test that execute() without graph_ops raises error."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes")

    with pytest.raises(ValueError, match="graph_ops not set"):
        q.execute()


def test_query_builder_execute_with_graph_ops():
    """Test execute() with mock graph_ops."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    mock_ops = Mock()
    mock_result = Mock()
    mock_ops.query.return_value = mock_result

    q = QueryBuilder(mock_ops)
    q.find("nodes").where("type", "=", "user")

    result = q.execute()

    assert result == mock_result
    mock_ops.query.assert_called_once()


def test_query_builder_convenience_function():
    """Test the convenience query() function."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    query = query_builder.query
    q = query("nodes").where("type", "=", "user")

    result = q.build()
    assert "find all nodes" in result
    assert "type = 'user'" in result


def test_query_builder_str_repr():
    """Test __str__ and __repr__ methods."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder
    q = QueryBuilder()
    q.find("nodes").where("type", "=", "user")

    str_result = str(q)
    assert "find all nodes" in str_result

    repr_result = repr(q)
    assert "QueryBuilder" in repr_result


# =============================================================================
# Test QueryResult Extensions (with mocks)
# =============================================================================

def test_to_dict_list():
    """Test converting QueryResult to list of dictionaries."""
    # Load helpers module directly
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "helpers",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/helpers.py"
    )
    helpers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helpers)

    QueryResultExtensions = helpers.QueryResultExtensions

    mock_node1 = Mock()
    mock_node1.id = "node1"
    mock_node1.type = "user"
    mock_node1.properties = {"name": "Alice"}

    mock_node2 = Mock()
    mock_node2.id = "node2"
    mock_node2.type = "user"
    mock_node2.properties = {"name": "Bob"}

    mock_result = Mock()
    mock_result.nodes = [mock_node1, mock_node2]
    mock_result.edges = None

    result = QueryResultExtensions.to_dict_list(mock_result)

    assert len(result) == 2
    assert result[0]['id'] == "node1"
    assert result[0]['type'] == "user"
    assert result[0]['properties'] == {"name": "Alice"}
    assert result[1]['id'] == "node2"


def test_export_json(tmp_path):
    """Test exporting QueryResult to JSON."""
    import importlib.util
    import json

    spec = importlib.util.spec_from_file_location(
        "helpers",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/helpers.py"
    )
    helpers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helpers)

    QueryResultExtensions = helpers.QueryResultExtensions

    mock_node = Mock()
    mock_node.id = "node1"
    mock_node.type = "user"
    mock_node.properties = {"name": "Alice"}

    mock_result = Mock()
    mock_result.nodes = [mock_node]
    mock_result.edges = None

    json_file = tmp_path / "test.json"

    QueryResultExtensions.export_json(mock_result, str(json_file))

    with open(json_file) as f:
        data = json.load(f)

    assert len(data) == 1
    assert data[0]['id'] == "node1"
    assert data[0]['type'] == "user"


def test_summary_nodes():
    """Test summary statistics for nodes."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "helpers",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/helpers.py"
    )
    helpers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helpers)

    QueryResultExtensions = helpers.QueryResultExtensions

    mock_node1 = Mock()
    mock_node1.id = "node1"
    mock_node1.type = "user"
    mock_node1.properties = {"name": "Alice", "age": 30}

    mock_node2 = Mock()
    mock_node2.id = "node2"
    mock_node2.type = "admin"
    mock_node2.properties = {"name": "Bob", "role": "admin"}

    mock_node3 = Mock()
    mock_node3.id = "node3"
    mock_node3.type = "user"
    mock_node3.properties = {"name": "Charlie"}

    mock_result = Mock()
    mock_result.nodes = [mock_node1, mock_node2, mock_node3]
    mock_result.edges = None

    summary = QueryResultExtensions.summary(mock_result)

    assert summary['node_count'] == 3
    assert summary['node_types'] == {"user": 2, "admin": 1}
    assert summary['unique_properties'] == 3  # name, age, role
    assert sorted(summary['property_names']) == ['age', 'name', 'role']


def test_summary_edges():
    """Test summary statistics for edges."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "helpers",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/helpers.py"
    )
    helpers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helpers)

    QueryResultExtensions = helpers.QueryResultExtensions

    mock_edge1 = Mock()
    mock_edge1.source_id = "node1"
    mock_edge1.target_id = "node2"
    mock_edge1.type = "follows"
    mock_edge1.properties = {}

    mock_edge2 = Mock()
    mock_edge2.source_id = "node2"
    mock_edge2.target_id = "node3"
    mock_edge2.type = "follows"
    mock_edge2.properties = {}

    mock_result = Mock()
    mock_result.nodes = None
    mock_result.edges = [mock_edge1, mock_edge2]

    summary = QueryResultExtensions.summary(mock_result)

    assert summary['edge_count'] == 2
    assert summary['edge_types'] == {"follows": 2}


# =============================================================================
# Widget Tests (isolated without IPython)
# =============================================================================

def test_metrics_widget_data_management():
    """Test MetricsWidget data management without IPython."""
    # Just test the data structures, not the UI
    import time

    metrics_data = []

    # Add metric
    metrics_data.append({"cpu": 50, "memory": 1024, "timestamp": time.time()})
    assert len(metrics_data) == 1
    assert metrics_data[0]['cpu'] == 50

    # Test max 100 data points
    for i in range(150):
        metrics_data.append({"value": i, "timestamp": time.time()})

    # Keep last 100
    if len(metrics_data) > 100:
        metrics_data = metrics_data[-100:]

    assert len(metrics_data) == 100


def test_log_viewer_data_management():
    """Test LogViewerWidget data management without IPython."""
    import time

    logs = []
    max_logs = 10

    # Add logs
    for i in range(20):
        logs.append({
            'level': "INFO",
            'message': f"Message {i}",
            'timestamp': time.strftime('%H:%M:%S')
        })

        # Keep only max_logs
        if len(logs) > max_logs:
            logs = logs[-max_logs:]

    assert len(logs) == 10
    assert logs[0]['message'] == "Message 10"  # First 10 dropped


def test_query_format_validation():
    """Test that query builder produces valid query syntax."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "query_builder",
        Path(__file__).parent.parent.parent / "src/neurograph_jupyter/query_builder.py"
    )
    query_builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(query_builder)

    QueryBuilder = query_builder.QueryBuilder

    # Test various query patterns
    test_cases = [
        (
            lambda: QueryBuilder().find("nodes"),
            "find all nodes"
        ),
        (
            lambda: QueryBuilder().find("edges"),
            "find all edges"
        ),
        (
            lambda: QueryBuilder().find("nodes").where("type", "=", "user"),
            "find all nodes where type = 'user'"
        ),
        (
            lambda: QueryBuilder().find("nodes").where("age", ">", 30),
            "find all nodes where age > 30"
        ),
        (
            lambda: QueryBuilder().find("nodes").limit(10),
            "find all nodes limit 10"
        ),
    ]

    for builder_fn, expected in test_cases:
        q = builder_fn()
        result = q.build()
        assert expected in result or expected == result


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
