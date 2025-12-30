"""
Plotly-based interactive graph visualizations for NeuroGraph.

Provides interactive, zoomable, and clickable graph visualizations.
"""

from typing import Any, Dict, List, Optional


def render_interactive_graph(
    result: Any,
    layout: str = "spring",
    node_size: int = 20,
    node_color: str = "lightblue",
    edge_color: str = "gray",
    show_labels: bool = True,
    height: int = 600,
    title: Optional[str] = None
):
    """
    Render interactive graph visualization using Plotly.

    Args:
        result: QueryResult with nodes and edges
        layout: Layout algorithm (spring, circular, kamada_kawai, hierarchical)
        node_size: Size of nodes
        node_color: Color of nodes (or property name for color mapping)
        edge_color: Color of edges
        show_labels: Show node labels
        height: Figure height in pixels
        title: Graph title

    Returns:
        Plotly Figure object

    Example:
        result = %neurograph query "find all nodes"
        from neurograph_jupyter.plotly_viz import render_interactive_graph
        fig = render_interactive_graph(result, layout="spring")
        fig.show()
    """
    try:
        import plotly.graph_objects as go
        import networkx as nx
    except ImportError:
        raise ImportError(
            "plotly and networkx are required. "
            "Install with: pip install plotly networkx"
        )

    # Build NetworkX graph
    G = nx.DiGraph()

    # Add nodes
    node_info = {}
    if hasattr(result, "nodes") and result.nodes:
        for node in result.nodes:
            node_id = node.id
            node_type = getattr(node, "type", "unknown")
            properties = getattr(node, "properties", {})

            G.add_node(node_id, type=node_type, **properties)
            node_info[node_id] = {
                "type": node_type,
                "properties": properties
            }

    # Add edges
    if hasattr(result, "edges") and result.edges:
        for edge in result.edges:
            G.add_edge(
                edge.source_id,
                edge.target_id,
                type=getattr(edge, "type", "unknown"),
                **getattr(edge, "properties", {})
            )

    # Calculate layout
    if layout == "spring":
        pos = nx.spring_layout(G, k=1/np.sqrt(len(G.nodes())), iterations=50)
    elif layout == "circular":
        pos = nx.circular_layout(G)
    elif layout == "kamada_kawai":
        pos = nx.kamada_kawai_layout(G)
    elif layout == "hierarchical":
        pos = nx.spring_layout(G, k=2, iterations=50)
    else:
        pos = nx.spring_layout(G)

    # Create edge traces
    edge_traces = []
    for edge in G.edges():
        x0, y0 = pos[edge[0]]
        x1, y1 = pos[edge[1]]

        edge_trace = go.Scatter(
            x=[x0, x1, None],
            y=[y0, y1, None],
            mode='lines',
            line=dict(width=1, color=edge_color),
            hoverinfo='none',
            showlegend=False
        )
        edge_traces.append(edge_trace)

    # Create node trace
    node_x = []
    node_y = []
    node_text = []
    node_colors = []

    for node in G.nodes():
        x, y = pos[node]
        node_x.append(x)
        node_y.append(y)

        # Build hover text
        info = node_info.get(node, {})
        hover_text = f"<b>ID:</b> {node}<br>"
        hover_text += f"<b>Type:</b> {info.get('type', 'unknown')}<br>"

        props = info.get('properties', {})
        if props:
            hover_text += "<b>Properties:</b><br>"
            for key, value in list(props.items())[:5]:  # First 5 properties
                hover_text += f"  {key}: {value}<br>"
            if len(props) > 5:
                hover_text += f"  ... and {len(props) - 5} more"

        node_text.append(hover_text)

        # Color by type or property
        if node_color in props:
            node_colors.append(props[node_color])
        else:
            node_colors.append(node_color)

    node_trace = go.Scatter(
        x=node_x,
        y=node_y,
        mode='markers+text' if show_labels else 'markers',
        hovertemplate='%{text}<extra></extra>',
        text=node_text,
        textposition="top center",
        marker=dict(
            size=node_size,
            color=node_colors if isinstance(node_colors[0], (int, float)) else node_color,
            colorscale='Viridis' if isinstance(node_colors[0], (int, float)) else None,
            showscale=isinstance(node_colors[0], (int, float)),
            line=dict(width=2, color='white')
        ),
        showlegend=False
    )

    # Create figure
    fig = go.Figure(data=edge_traces + [node_trace])

    # Update layout
    fig.update_layout(
        title=title or f"Graph Visualization ({len(G.nodes())} nodes, {len(G.edges())} edges)",
        titlefont_size=16,
        showlegend=False,
        hovermode='closest',
        margin=dict(b=0, l=0, r=0, t=40),
        xaxis=dict(showgrid=False, zeroline=False, showticklabels=False),
        yaxis=dict(showgrid=False, zeroline=False, showticklabels=False),
        height=height,
        plot_bgcolor='white'
    )

    return fig


def render_3d_graph(
    result: Any,
    node_size: int = 10,
    node_color: str = "lightblue",
    edge_color: str = "gray",
    height: int = 700,
    title: Optional[str] = None
):
    """
    Render 3D interactive graph visualization.

    Args:
        result: QueryResult with nodes and edges
        node_size: Size of nodes
        node_color: Color of nodes
        edge_color: Color of edges
        height: Figure height in pixels
        title: Graph title

    Returns:
        Plotly 3D Figure object

    Example:
        result = %neurograph query "find all nodes"
        from neurograph_jupyter.plotly_viz import render_3d_graph
        fig = render_3d_graph(result)
        fig.show()
    """
    try:
        import plotly.graph_objects as go
        import networkx as nx
        import numpy as np
    except ImportError:
        raise ImportError(
            "plotly, networkx, and numpy are required. "
            "Install with: pip install plotly networkx numpy"
        )

    # Build graph
    G = nx.DiGraph()

    node_info = {}
    if hasattr(result, "nodes") and result.nodes:
        for node in result.nodes:
            node_id = node.id
            node_type = getattr(node, "type", "unknown")
            properties = getattr(node, "properties", {})
            G.add_node(node_id, type=node_type, **properties)
            node_info[node_id] = {"type": node_type, "properties": properties}

    if hasattr(result, "edges") and result.edges:
        for edge in result.edges:
            G.add_edge(edge.source_id, edge.target_id)

    # 3D spring layout
    pos = nx.spring_layout(G, dim=3, k=1/np.sqrt(len(G.nodes())), iterations=50)

    # Extract 3D positions
    node_xyz = np.array([pos[node] for node in G.nodes()])
    edge_xyz = []

    for edge in G.edges():
        edge_xyz.append([
            [pos[edge[0]][0], pos[edge[1]][0]],
            [pos[edge[0]][1], pos[edge[1]][1]],
            [pos[edge[0]][2], pos[edge[1]][2]]
        ])

    # Create edge traces
    edge_trace = go.Scatter3d(
        x=sum([[e[0][0], e[0][1], None] for e in edge_xyz], []),
        y=sum([[e[1][0], e[1][1], None] for e in edge_xyz], []),
        z=sum([[e[2][0], e[2][1], None] for e in edge_xyz], []),
        mode='lines',
        line=dict(color=edge_color, width=2),
        hoverinfo='none'
    )

    # Create node trace
    node_text = []
    for node in G.nodes():
        info = node_info.get(node, {})
        hover = f"<b>ID:</b> {node}<br><b>Type:</b> {info.get('type', 'unknown')}"
        node_text.append(hover)

    node_trace = go.Scatter3d(
        x=node_xyz[:, 0],
        y=node_xyz[:, 1],
        z=node_xyz[:, 2],
        mode='markers',
        marker=dict(
            size=node_size,
            color=node_color,
            line=dict(color='white', width=0.5)
        ),
        text=node_text,
        hoverinfo='text'
    )

    # Create figure
    fig = go.Figure(data=[edge_trace, node_trace])

    fig.update_layout(
        title=title or f"3D Graph ({len(G.nodes())} nodes, {len(G.edges())} edges)",
        showlegend=False,
        height=height,
        scene=dict(
            xaxis=dict(showbackground=False, showticklabels=False, title=''),
            yaxis=dict(showbackground=False, showticklabels=False, title=''),
            zaxis=dict(showbackground=False, showticklabels=False, title='')
        )
    )

    return fig


# Add numpy import at top for layouts
try:
    import numpy as np
except ImportError:
    np = None
