"""
Rich display formatters for NeuroGraph objects in Jupyter.
"""

from typing import Any, Dict, List
from IPython.display import HTML, display
import json


def format_query_result_html(result: Any) -> str:
    """
    Format QueryResult as rich HTML table.

    Args:
        result: QueryResult object

    Returns:
        HTML string for display
    """
    # Check if it's a QueryResult-like object
    if not hasattr(result, "nodes") and not hasattr(result, "edges"):
        return None

    html_parts = []

    # Header
    html_parts.append("""
    <style>
        .neurograph-result {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 10px 0;
        }
        .neurograph-header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 12px 16px;
            border-radius: 8px 8px 0 0;
            font-weight: bold;
            font-size: 14px;
        }
        .neurograph-table {
            width: 100%;
            border-collapse: collapse;
            border: 1px solid #e0e0e0;
            border-top: none;
        }
        .neurograph-table th {
            background: #f5f5f5;
            padding: 10px;
            text-align: left;
            font-weight: 600;
            border-bottom: 2px solid #667eea;
            font-size: 13px;
        }
        .neurograph-table td {
            padding: 8px 10px;
            border-bottom: 1px solid #e0e0e0;
            font-size: 12px;
        }
        .neurograph-table tr:hover {
            background: #f9f9f9;
        }
        .neurograph-count {
            color: #667eea;
            font-weight: bold;
        }
        .neurograph-id {
            font-family: 'Courier New', monospace;
            color: #764ba2;
        }
        .neurograph-props {
            font-family: 'Courier New', monospace;
            color: #555;
            font-size: 11px;
        }
    </style>
    """)

    # Nodes section
    if hasattr(result, "nodes") and result.nodes:
        html_parts.append('<div class="neurograph-result">')
        html_parts.append(
            f'<div class="neurograph-header">🔵 Nodes: '
            f'<span class="neurograph-count">{len(result.nodes)}</span></div>'
        )
        html_parts.append('<table class="neurograph-table">')
        html_parts.append(
            "<tr><th>ID</th><th>Type</th><th>Properties</th></tr>"
        )

        for node in result.nodes[:50]:  # Limit to 50 for display
            node_id = getattr(node, "id", "N/A")
            node_type = getattr(node, "type", "N/A")
            props = getattr(node, "properties", {})

            # Format properties
            props_str = json.dumps(props, indent=None) if props else "{}"
            if len(props_str) > 100:
                props_str = props_str[:97] + "..."

            html_parts.append(
                f'<tr>'
                f'<td class="neurograph-id">{node_id}</td>'
                f'<td>{node_type}</td>'
                f'<td class="neurograph-props">{props_str}</td>'
                f'</tr>'
            )

        if len(result.nodes) > 50:
            html_parts.append(
                f'<tr><td colspan="3" style="text-align: center; '
                f'font-style: italic; color: #999;">... and {len(result.nodes) - 50} more</td></tr>'
            )

        html_parts.append("</table>")
        html_parts.append("</div>")

    # Edges section
    if hasattr(result, "edges") and result.edges:
        html_parts.append('<div class="neurograph-result">')
        html_parts.append(
            f'<div class="neurograph-header">🔗 Edges: '
            f'<span class="neurograph-count">{len(result.edges)}</span></div>'
        )
        html_parts.append('<table class="neurograph-table">')
        html_parts.append(
            "<tr><th>Source</th><th>Target</th><th>Type</th><th>Properties</th></tr>"
        )

        for edge in result.edges[:50]:  # Limit to 50 for display
            source = getattr(edge, "source_id", "N/A")
            target = getattr(edge, "target_id", "N/A")
            edge_type = getattr(edge, "type", "N/A")
            props = getattr(edge, "properties", {})

            # Format properties
            props_str = json.dumps(props, indent=None) if props else "{}"
            if len(props_str) > 80:
                props_str = props_str[:77] + "..."

            html_parts.append(
                f'<tr>'
                f'<td class="neurograph-id">{source}</td>'
                f'<td class="neurograph-id">{target}</td>'
                f'<td>{edge_type}</td>'
                f'<td class="neurograph-props">{props_str}</td>'
                f'</tr>'
            )

        if len(result.edges) > 50:
            html_parts.append(
                f'<tr><td colspan="4" style="text-align: center; '
                f'font-style: italic; color: #999;">... and {len(result.edges) - 50} more</td></tr>'
            )

        html_parts.append("</table>")
        html_parts.append("</div>")

    # Empty result
    if (not hasattr(result, "nodes") or not result.nodes) and \
       (not hasattr(result, "edges") or not result.edges):
        html_parts.append('<div class="neurograph-result">')
        html_parts.append(
            '<div class="neurograph-header">📭 Empty Result</div>'
        )
        html_parts.append(
            '<div style="padding: 20px; text-align: center; color: #999;">'
            'No nodes or edges found'
            '</div>'
        )
        html_parts.append("</div>")

    return "".join(html_parts)


def install_display_formatters(ipython):
    """
    Install rich display formatters for NeuroGraph objects.

    This enables automatic rich HTML display when NeuroGraph objects
    are returned in Jupyter cells.

    Args:
        ipython: IPython instance
    """
    html_formatter = ipython.display_formatter.formatters["text/html"]

    # Register formatter for QueryResult
    # Note: We check for duck-typed QueryResult (has nodes/edges attributes)
    def query_result_formatter(result):
        if hasattr(result, "nodes") or hasattr(result, "edges"):
            return format_query_result_html(result)
        return None

    # Register for common result types
    # Since QueryResult might be dynamically created, we use a generic check
    html_formatter.for_type_by_name(
        "neurograph.query.result",
        "QueryResult",
        query_result_formatter
    )

    # Also register a catch-all formatter
    original_format = html_formatter.__call__

    def neurograph_aware_format(obj):
        # Try NeuroGraph formatting first
        if hasattr(obj, "nodes") or hasattr(obj, "edges"):
            html = format_query_result_html(obj)
            if html:
                return (html, {})

        # Fall back to original formatter
        return original_format(obj)

    # Note: This is a simplified approach
    # A production implementation would use proper IPython formatter registration


def render_graph_visualization(result: Any, layout: str = "spring") -> HTML:
    """
    Render graph visualization using networkx and matplotlib.

    Args:
        result: QueryResult with nodes and edges
        layout: Layout algorithm (spring, circular, kamada_kawai)

    Returns:
        HTML object with embedded SVG
    """
    try:
        import networkx as nx
        import matplotlib.pyplot as plt
        from io import BytesIO
        import base64

        # Create graph
        G = nx.DiGraph()

        # Add nodes
        if hasattr(result, "nodes"):
            for node in result.nodes:
                G.add_node(
                    node.id,
                    type=getattr(node, "type", "unknown"),
                    **getattr(node, "properties", {})
                )

        # Add edges
        if hasattr(result, "edges"):
            for edge in result.edges:
                G.add_edge(
                    edge.source_id,
                    edge.target_id,
                    type=getattr(edge, "type", "unknown")
                )

        # Create figure
        fig, ax = plt.subplots(figsize=(12, 8))

        # Choose layout
        if layout == "spring":
            pos = nx.spring_layout(G)
        elif layout == "circular":
            pos = nx.circular_layout(G)
        elif layout == "kamada_kawai":
            pos = nx.kamada_kawai_layout(G)
        else:
            pos = nx.spring_layout(G)

        # Draw
        nx.draw(
            G,
            pos,
            with_labels=True,
            node_color="lightblue",
            node_size=500,
            font_size=10,
            font_weight="bold",
            arrows=True,
            ax=ax
        )

        plt.title(f"Graph Visualization ({len(G.nodes)} nodes, {len(G.edges)} edges)")

        # Convert to base64
        buf = BytesIO()
        plt.savefig(buf, format="png", dpi=100, bbox_inches="tight")
        buf.seek(0)
        img_base64 = base64.b64encode(buf.read()).decode("utf-8")
        plt.close()

        # Return HTML
        html = f'<img src="data:image/png;base64,{img_base64}" />'
        return HTML(html)

    except ImportError as e:
        return HTML(
            f'<div style="color: red;">❌ Visualization requires networkx and matplotlib: {e}</div>'
        )
    except Exception as e:
        return HTML(
            f'<div style="color: red;">❌ Visualization failed: {e}</div>'
        )
