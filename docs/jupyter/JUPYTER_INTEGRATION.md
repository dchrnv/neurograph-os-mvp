# NeuroGraph Jupyter Integration

**Version:** v0.61.0
**Status:** ✅ Complete
**Date:** 2025-12-30

---

## Overview

The NeuroGraph Jupyter Integration provides a seamless IPython extension for interactive graph operations in Jupyter notebooks. It enables rapid prototyping, data exploration, and real-time signal processing with beautiful visualizations.

## Features

### 🪄 Magic Commands

**Line Magic (`%neurograph`):**
- Quick initialization and status checks
- One-line queries
- Real-time subscriptions and signals

**Cell Magic (`%%signal`):**
- Define signal handlers inline
- Process real-time events
- Custom data transformations

### 📊 Rich Display

- **HTML Tables** - Beautiful, color-coded result tables
- **Node/Edge Counts** - Quick statistics at a glance
- **Property Formatting** - JSON properties with truncation
- **Interactive Hover** - Hover effects for better UX

### 📡 Real-time Integration

- **Subscribe** - Listen to channels for updates
- **Emit** - Send signals to subscribers
- **Signal Handlers** - Define custom event processors
- **WebSocket Support** - Full integration with ConnectionManager

### 🎨 Visualizations

- **Graph Rendering** - NetworkX-based visualizations
- **Multiple Layouts** - Spring, circular, Kamada-Kawai
- **Node Attributes** - Display properties and types
- **Edge Relationships** - Show connections and types

### 📈 Data Export

- **Pandas DataFrames** - Convert results to DataFrames
- **CSV Export** - Save results for analysis
- **JSON Export** - Structured data export
- **Custom Formats** - Extensible export system

---

## Installation

### Basic Installation

```bash
pip install neurograph
```

### With Jupyter Support

```bash
pip install neurograph[jupyter]
```

This installs:
- `ipython>=8.0.0`
- `jupyter>=1.0.0`
- `notebook>=6.4.0`
- `jupyterlab>=3.0.0`
- `pandas>=1.3.0`
- `networkx>=2.6.0`
- `matplotlib>=3.5.0`
- `plotly>=5.0.0`

---

## Quick Start

### 1. Load Extension

```python
%load_ext neurograph_jupyter
```

Output:
```
✅ NeuroGraph Jupyter extension loaded
Try: %neurograph init --path ./my_graph.db
```

### 2. Initialize Database

```python
%neurograph init --path ./my_graph.db
```

Output:
```
✅ NeuroGraph initialized
📁 Database: /path/to/my_graph.db
🔗 Connection Manager: Ready
📡 Signal Engine: Ready

Available in namespace:
  - neurograph_db: GraphOperations
  - neurograph_signals: SignalEngine
  - neurograph_ws: ConnectionManager
```

### 3. Query Data

```python
%neurograph query "find all nodes where type='user'"
```

Displays rich HTML table with results.

---

## Magic Commands Reference

### Line Magic: `%neurograph`

#### Initialize

```python
%neurograph init --path <db_path>
```

**Arguments:**
- `--path` (required) - Path to database file

**Example:**
```python
%neurograph init --path ./production.db
```

#### Status

```python
%neurograph status
```

**Output:**
```
🟢 NeuroGraph Status
📁 Database: ./production.db
🔗 Active Connections: 5
📡 Signal Engine: Active
📬 Total Subscriptions: 12
📢 Active Channels: metrics, events, logs
```

#### Query

```python
%neurograph query "<query_string>"
```

**Arguments:**
- `query_string` - NeuroGraph query in quotes

**Examples:**
```python
%neurograph query "find all nodes"
%neurograph query "find all nodes where age > 30"
%neurograph query "find all nodes where type='project'"
```

#### Subscribe

```python
%neurograph subscribe <channel>
```

**Arguments:**
- `channel` - Channel name to subscribe to

**Example:**
```python
%neurograph subscribe metrics
```

**Output:**
```
✅ Subscribed to channel: metrics
👤 Client ID: jupyter_notebook
```

#### Emit

```python
%neurograph emit <channel> <data>
```

**Arguments:**
- `channel` - Channel name
- `data` - JSON string or simple value

**Examples:**
```python
%neurograph emit metrics "{'cpu': 42, 'memory': 68}"
%neurograph emit events "user_login"
%neurograph emit logs "{'level': 'info', 'message': 'Started'}"
```

**Output:**
```
✅ Signal emitted to channel: metrics
📊 Data: {'cpu': 42, 'memory': 68}
```

### Cell Magic: `%%signal`

Define a signal handler function.

**Syntax:**
```python
%%signal <signal_name>
def handler(data):
    # Process data
    return result
```

**Example:**
```python
%%signal process_metrics
def handler(data):
    cpu = data.get("cpu", 0)
    memory = data.get("memory", 0)

    if cpu > 80:
        print(f"⚠️ High CPU: {cpu}%")
    if memory > 90:
        print(f"⚠️ High Memory: {memory}%")

    return {"status": "processed", "cpu": cpu, "memory": memory}
```

**Output:**
```
✅ Signal handler registered: process_metrics
📡 Function: handler
```

The handler is stored as `signal_process_metrics` in the notebook namespace.

---

## Rich Display System

### Automatic HTML Rendering

Query results are automatically rendered as beautiful HTML tables:

```python
result = %neurograph query "find all nodes"
```

**Features:**
- Color-coded headers (gradient purple/blue)
- Node and edge counts
- Formatted properties (JSON with truncation)
- Interactive hover effects
- Limited display (first 50 items)
- "... and N more" indicator

### Manual Display Control

Access raw results:

```python
result = neurograph_db.query("find all nodes")

# Access nodes
for node in result.nodes:
    print(f"{node.id}: {node.type}")

# Access edges
for edge in result.edges:
    print(f"{edge.source_id} -> {edge.target_id}")
```

---

## Visualization

### Graph Rendering

```python
from neurograph_jupyter.display import render_graph_visualization

result = neurograph_db.query("find all nodes")
render_graph_visualization(result, layout="spring")
```

**Layout Options:**
- `spring` - Force-directed layout (default)
- `circular` - Nodes in a circle
- `kamada_kawai` - Energy-minimization layout

**Output:**
Embedded PNG image showing graph structure.

### Custom Visualizations

```python
import networkx as nx
import matplotlib.pyplot as plt

# Build custom graph
G = nx.DiGraph()
for node in result.nodes:
    G.add_node(node.id, **node.properties)
for edge in result.edges:
    G.add_edge(edge.source_id, edge.target_id)

# Custom rendering
pos = nx.spring_layout(G)
nx.draw(G, pos, with_labels=True, node_color='lightblue')
plt.show()
```

---

## Data Export

### Pandas DataFrames

```python
import pandas as pd

result = neurograph_db.query("find all nodes where type='user'")

# Convert to DataFrame
data = []
for node in result.nodes:
    data.append({
        "id": node.id,
        "name": node.properties.get("name"),
        "age": node.properties.get("age"),
        "city": node.properties.get("city")
    })

df = pd.DataFrame(data)
df.head()
```

### CSV Export

```python
df.to_csv("users.csv", index=False)
```

### JSON Export

```python
import json

# Export nodes
nodes_data = [
    {
        "id": node.id,
        "type": node.type,
        "properties": node.properties
    }
    for node in result.nodes
]

with open("nodes.json", "w") as f:
    json.dump(nodes_data, f, indent=2)
```

---

## Real-time Workflows

### Monitoring Pattern

```python
# Subscribe to metrics
%neurograph subscribe metrics

# Define handler
%%signal process_metrics
def handler(data):
    cpu = data["cpu"]
    memory = data["memory"]

    # Alert on thresholds
    if cpu > 80 or memory > 90:
        print(f"⚠️ Resource alert: CPU={cpu}%, MEM={memory}%")

    return {"status": "ok"}

# Emit test signal
%neurograph emit metrics "{'cpu': 85, 'memory': 70}"
```

### Event Processing

```python
# Subscribe to events
%neurograph subscribe events

# Define event processor
%%signal process_event
def handler(event):
    event_type = event.get("type")

    if event_type == "user_login":
        print(f"👤 User logged in: {event['user_id']}")
    elif event_type == "error":
        print(f"❌ Error: {event['message']}")

    return {"processed": True}

# Emit events
%neurograph emit events "{'type': 'user_login', 'user_id': 'alice'}"
```

---

## Advanced Usage

### Direct API Access

The extension exposes three objects in the notebook namespace:

```python
# GraphOperations API
result = neurograph_db.query("find all nodes")
node = neurograph_db.create_node("user", {"name": "Alice"})
neurograph_db.delete_node(node.id)

# SignalEngine API
neurograph_signals.register_handler("my_signal", my_handler)
neurograph_signals.emit("my_signal", {"data": "value"})

# ConnectionManager API
neurograph_ws.register_connection("client_123")
neurograph_ws.subscribe("client_123", ["channel_1"])
await neurograph_ws.broadcast_to_channel("channel_1", {"msg": "hello"})
```

### Performance Monitoring

```python
import time

# Benchmark query
start = time.perf_counter()
result = neurograph_db.query("find all nodes")
duration = time.perf_counter() - start

print(f"Nodes: {len(result.nodes)}")
print(f"Edges: {len(result.edges)}")
print(f"Duration: {duration*1000:.2f} ms")
print(f"Throughput: {len(result.nodes)/duration:.0f} nodes/sec")
```

### Batch Operations

```python
# Create multiple nodes
nodes = []
for i in range(100):
    node = neurograph_db.create_node(
        "user",
        {"name": f"user_{i}", "index": i}
    )
    nodes.append(node)

# Create edges
for i in range(len(nodes) - 1):
    neurograph_db.create_edge(
        nodes[i].id,
        nodes[i+1].id,
        "follows"
    )

print(f"Created {len(nodes)} nodes and {len(nodes)-1} edges")
```

---

## Troubleshooting

### Extension Not Loading

**Problem:**
```python
%load_ext neurograph_jupyter
# ModuleNotFoundError: No module named 'neurograph_jupyter'
```

**Solution:**
```bash
pip install neurograph[jupyter]
```

### Database Not Initialized

**Problem:**
```python
%neurograph query "find all nodes"
# ❌ NeuroGraph not initialized
```

**Solution:**
```python
%neurograph init --path ./my_graph.db
```

### Import Errors

**Problem:**
```python
from neurograph_jupyter.display import render_graph_visualization
# ImportError: cannot import name 'render_graph_visualization'
```

**Solution:**
Install optional dependencies:
```bash
pip install networkx matplotlib
```

### Rich Display Not Working

**Problem:**
Query results show as plain text instead of HTML tables.

**Solution:**
The rich display formatter is automatically installed when the extension loads. Try:
```python
%reload_ext neurograph_jupyter
```

---

## Examples

### Example 1: User Network Analysis

```python
# Load extension
%load_ext neurograph_jupyter

# Initialize
%neurograph init --path ./social_network.db

# Create users
alice = neurograph_db.create_node("user", {"name": "Alice", "age": 30})
bob = neurograph_db.create_node("user", {"name": "Bob", "age": 25})
charlie = neurograph_db.create_node("user", {"name": "Charlie", "age": 35})

# Create friendships
neurograph_db.create_edge(alice.id, bob.id, "follows")
neurograph_db.create_edge(bob.id, charlie.id, "follows")
neurograph_db.create_edge(charlie.id, alice.id, "follows")

# Query and visualize
result = %neurograph query "find all nodes"
render_graph_visualization(result, layout="circular")
```

### Example 2: Real-time Metrics Dashboard

```python
# Setup
%neurograph init --path ./metrics.db
%neurograph subscribe metrics

# Define handler with visualization
%%signal visualize_metrics
def handler(data):
    import matplotlib.pyplot as plt

    metrics = data.get("metrics", [])

    plt.figure(figsize=(10, 4))
    plt.plot([m["timestamp"] for m in metrics], [m["cpu"] for m in metrics])
    plt.xlabel("Time")
    plt.ylabel("CPU %")
    plt.title("CPU Usage")
    plt.show()

    return {"rendered": True}

# Simulate metrics
import random
for i in range(10):
    %neurograph emit metrics "{'timestamp': {i}, 'cpu': {random.randint(20, 90)}}"
```

### Example 3: Knowledge Graph Explorer

```python
# Initialize
%neurograph init --path ./knowledge_graph.db

# Create ontology
concept1 = neurograph_db.create_node("concept", {"name": "Machine Learning"})
concept2 = neurograph_db.create_node("concept", {"name": "Neural Networks"})
concept3 = neurograph_db.create_node("concept", {"name": "Deep Learning"})

neurograph_db.create_edge(concept2.id, concept1.id, "is_a")
neurograph_db.create_edge(concept3.id, concept2.id, "is_a")

# Export to DataFrame for analysis
result = neurograph_db.query("find all nodes")
df = pd.DataFrame([
    {
        "id": n.id,
        "name": n.properties["name"],
        "type": n.type
    }
    for n in result.nodes
])

df.head()
```

---

## API Reference

### NeuroGraphMagics

IPython magic commands class.

**Methods:**
- `neurograph(line: str)` - Main magic command router
- `signal(line: str, cell: str)` - Cell magic for signal handlers

**Instance Variables:**
- `connection_manager: ConnectionManager` - WebSocket manager
- `graph_ops: GraphOperations` - Graph database operations
- `signal_engine: SignalEngine` - Signal processing engine
- `db_path: Path` - Current database path

### Display Functions

#### `format_query_result_html(result) -> str`

Format QueryResult as HTML table.

**Args:**
- `result` - QueryResult object with nodes/edges

**Returns:**
- HTML string for Jupyter display

#### `render_graph_visualization(result, layout='spring') -> HTML`

Render graph visualization.

**Args:**
- `result` - QueryResult with graph data
- `layout` - Layout algorithm name

**Returns:**
- HTML object with embedded image

---

## Performance Considerations

### Large Result Sets

Query results are limited to 50 items in rich display to prevent notebook slowdown. Access full results via:

```python
result = neurograph_db.query("find all nodes")
print(f"Total nodes: {len(result.nodes)}")  # Full count
```

### Memory Usage

For large graphs (>10K nodes), consider:

```python
# Query in batches
offset = 0
batch_size = 1000

while True:
    result = neurograph_db.query(
        f"find all nodes limit {batch_size} offset {offset}"
    )
    if not result.nodes:
        break

    # Process batch
    process_batch(result.nodes)
    offset += batch_size
```

### Visualization Limits

For graphs with >1000 nodes, visualization may be slow. Consider:

```python
# Sample for visualization
result = neurograph_db.query("find all nodes limit 100")
render_graph_visualization(result)
```

---

## Best Practices

### 1. Initialize Once

Initialize at the start of your notebook:

```python
%load_ext neurograph_jupyter
%neurograph init --path ./my_graph.db
```

### 2. Use Magic Commands for Quick Queries

For one-off queries, use magic commands:

```python
%neurograph query "find all nodes where type='user'"
```

### 3. Use Direct API for Complex Operations

For batch operations or complex logic, use direct API:

```python
for i in range(100):
    neurograph_db.create_node("user", {"index": i})
```

### 4. Export for Analysis

Export to DataFrame for statistical analysis:

```python
df = pd.DataFrame([...])
df.describe()
df.plot()
```

### 5. Monitor Performance

Track query performance for optimization:

```python
import time
start = time.perf_counter()
result = neurograph_db.query("...")
print(f"Duration: {(time.perf_counter() - start)*1000:.2f}ms")
```

---

## Version History

### v0.61.0 (2025-12-30)

Initial release with:
- ✅ IPython extension with magic commands
- ✅ Rich HTML display for QueryResult
- ✅ Real-time subscriptions and signals
- ✅ Cell magic for signal handlers
- ✅ Graph visualizations (networkx)
- ✅ Pandas DataFrame export
- ✅ Tutorial notebook
- ✅ Comprehensive documentation

---

## Next Steps

- **v0.62.0** - Interactive Plotly visualizations
- **v0.63.0** - Auto-completion for queries
- **v0.64.0** - Query builder UI widget
- **v0.65.0** - Real-time graph animations

---

## Support

- **Documentation:** [docs/jupyter/](../jupyter/)
- **Examples:** [notebooks/](../../notebooks/)
- **Issues:** [GitHub Issues](https://github.com/chrnv/neurograph-os/issues)

---

**Version:** v0.61.0
**Last Updated:** 2025-12-30
