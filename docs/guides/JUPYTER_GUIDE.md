# Jupyter Integration Guide

**Quick practical guide for using NeuroGraph in Jupyter notebooks**

Version: v0.61.1

---

## What's New in v0.61.1

- **Auto-completion**: Tab completion for magic commands and query syntax
- **DataFrame Helpers**: Built-in methods for pandas export and analysis
- **Interactive Visualizations**: Plotly 2D/3D graphs with zoom and pan
- **Query Builder**: Fluent API for building queries programmatically
- **Live Widgets**: Real-time metrics, logs, and graph exploration
- **Unit Tests**: Comprehensive test coverage for all features

---

## Quick Start

### 1. Install

```bash
pip install neurograph[jupyter]
```

### 2. Load Extension

```python
%load_ext neurograph_jupyter
```

### 3. Initialize

```python
%neurograph init --path ./my_graph.db
```

---

## Magic Commands

### Auto-completion (NEW in v0.61.1)

Press **TAB** after `%neurograph` for auto-completion:
- Command names (init, query, status, subscribe, emit)
- Channel names for subscribe/emit
- Query templates and node types
- Property names from database

```python
%neurograph <TAB>          # Shows available commands
%neurograph query <TAB>    # Shows query templates
%neurograph subscribe <TAB> # Shows available channels
```

### Initialize Database

```python
%neurograph init --path ./my_graph.db
```

Creates:
- `neurograph_db` - GraphOperations
- `neurograph_signals` - SignalEngine
- `neurograph_ws` - ConnectionManager

### Check Status

```python
%neurograph status
```

Shows:
- Database path
- Active connections
- Total subscriptions
- Active channels

### Query Data

```python
%neurograph query "find all nodes"
%neurograph query "find all nodes where type='user'"
%neurograph query "find all nodes where age > 30"
```

Results display as beautiful HTML tables automatically.

### Real-time Signals

**Subscribe:**
```python
%neurograph subscribe metrics
```

**Emit:**
```python
%neurograph emit metrics "{'cpu': 85, 'memory': 70}"
```

### Define Signal Handler

```python
%%signal process_metrics
def handler(data):
    cpu = data.get("cpu", 0)
    if cpu > 80:
        print(f"⚠️ High CPU: {cpu}%")
    return {"status": "ok"}
```

---

## Working with Data

### Create Nodes

```python
# Using direct API
user = neurograph_db.create_node(
    "user",
    {"name": "Alice", "age": 30, "city": "San Francisco"}
)

project = neurograph_db.create_node(
    "project",
    {"name": "NeuroGraph", "status": "active"}
)
```

### Create Edges

```python
neurograph_db.create_edge(
    user.id,
    project.id,
    "works_on",
    {"role": "lead", "since": "2024-01-01"}
)
```

### Query

```python
# Magic command (quick)
%neurograph query "find all nodes where type='user'"

# Direct API (for processing)
result = neurograph_db.query("find all nodes")
for node in result.nodes:
    print(f"{node.id}: {node.properties}")
```

---

## Visualization

### Static Matplotlib Visualization

```python
from neurograph_jupyter.display import render_graph_visualization

result = neurograph_db.query("find all nodes")
render_graph_visualization(result, layout="spring")
```

**Layouts:**
- `spring` - Force-directed (default)
- `circular` - Circular arrangement
- `kamada_kawai` - Energy minimization

### Interactive Plotly Visualizations (NEW in v0.61.1)

**2D Interactive Graph:**
```python
result = neurograph_db.query("find all nodes")

# Interactive 2D graph with zoom/pan
fig = result.plot_interactive(layout="spring")
fig.show()

# Customize
fig = result.plot_interactive(
    layout="circular",
    node_size=15,
    node_color="lightblue",
    edge_color="gray",
    show_labels=True,
    height=700,
    title="My Graph"
)
fig.show()
```

**3D Interactive Graph:**
```python
# 3D rotatable graph
fig = result.plot_3d()
fig.show()

# Customize
fig = result.plot_3d(
    node_size=10,
    node_color="orange",
    edge_color="gray",
    height=800
)
fig.show()
```

**Features:**
- Zoom, pan, rotate controls
- Hover tooltips with node/edge details
- Export to PNG/SVG
- Full interactivity in Jupyter

---

## Query Builder (NEW in v0.61.1)

Build queries programmatically with a fluent API:

```python
from neurograph_jupyter.query_builder import QueryBuilder

# Simple query
q = QueryBuilder(neurograph_db)
q.find("nodes").where("type", "=", "user").execute()

# Multiple conditions
q = QueryBuilder(neurograph_db)
result = (q.find("nodes")
           .where("type", "=", "user")
           .where("age", ">", 30)
           .limit(10)
           .execute())

# Build query string without executing
q = QueryBuilder()
query_string = q.find("nodes").where("type", "=", "user").build()
print(query_string)  # "find all nodes where type = 'user'"

# Convenience function
from neurograph_jupyter.query_builder import query

result = query("nodes").where("status", "in", ["active", "pending"]).build()
```

---

## DataFrame Helpers (NEW in v0.61.1)

QueryResult now has built-in DataFrame methods:

```python
result = neurograph_db.query("find all nodes where type='user'")

# Convert to pandas DataFrame (automatic property expansion)
df = result.to_dataframe()
print(df.head())

# Keep properties as dict column
df = result.to_dataframe(include_properties=False)

# Export to CSV
result.export_csv("users.csv")

# Export to JSON
result.export_json("users.json", indent=2)

# Get summary statistics
stats = result.summary()
print(f"Total nodes: {stats['node_count']}")
print(f"Node types: {stats['node_types']}")
print(f"Properties: {stats['property_names']}")

# Plot distribution
result.plot_distribution("age", bins=20)
```

**Old way (still works):**
```python
import pandas as pd

result = neurograph_db.query("find all nodes where type='user'")
df = pd.DataFrame([
    {"id": node.id, "name": node.properties.get("name"), "age": node.properties.get("age")}
    for node in result.nodes
])
df.to_csv("users.csv", index=False)
```

---

## Live Widgets (NEW in v0.61.1)

Interactive widgets for real-time monitoring and exploration:

### Metrics Widget

Live-updating metrics display with auto-refresh:

```python
from neurograph_jupyter.widgets import MetricsWidget

# Create widget
widget = MetricsWidget(neurograph_ws, refresh_interval=2.0)
widget.subscribe("metrics")
display(widget)

# Add metrics programmatically
widget.add_metric({"cpu": 75, "memory": 1024, "requests": 1500})

# Start/stop live updates with button
```

### Graph Explorer Widget

Interactive graph browsing with search and filtering:

```python
from neurograph_jupyter.widgets import GraphExplorerWidget

# Create explorer
explorer = GraphExplorerWidget(neurograph_db)
display(explorer)

# Use UI to:
# - Search by node type
# - Filter by property values
# - View results in real-time
```

### Log Viewer Widget

Real-time log display with level filtering:

```python
from neurograph_jupyter.widgets import LogViewerWidget

# Create log viewer
logs = LogViewerWidget(neurograph_ws, max_logs=100)
logs.subscribe("logs")
display(logs)

# Add logs programmatically
logs.add_log("INFO", "System started")
logs.add_log("WARNING", "High CPU usage")
logs.add_log("ERROR", "Connection failed")

# Filter by level with dropdown
# Clear logs with button
```

**Widget Features:**
- Start/Stop controls
- Auto-refresh
- Level filtering (logs)
- Search and filter (explorer)
- Clean, interactive UI
- Real-time updates

---

## Real-time Monitoring Example

```python
# Subscribe
%neurograph subscribe metrics

# Define handler
%%signal process_metrics
def handler(data):
    import time

    # Store metric
    metric = neurograph_db.create_node(
        "metric",
        {
            "timestamp": time.time(),
            "cpu": data.get("cpu", 0),
            "memory": data.get("memory", 0)
        }
    )

    # Alert on threshold
    if data.get("cpu", 0) > 80:
        print(f"🔴 HIGH CPU: {data['cpu']}%")

    return {"stored": metric.id}

# Test
%neurograph emit metrics "{'cpu': 85, 'memory': 70}"
```

---

## Performance Monitoring

```python
import time

# Measure query performance
start = time.perf_counter()
result = neurograph_db.query("find all nodes")
duration = time.perf_counter() - start

print(f"Nodes: {len(result.nodes)}")
print(f"Edges: {len(result.edges)}")
print(f"Duration: {duration*1000:.2f} ms")
print(f"Throughput: {len(result.nodes)/duration:.0f} nodes/sec")
```

---

## Common Patterns

### Pattern 1: Data Exploration

```python
# Load extension
%load_ext neurograph_jupyter

# Initialize
%neurograph init --path ./data.db

# Quick queries
%neurograph query "find all nodes"
%neurograph query "find all nodes where type='user'"

# Visualize
result = neurograph_db.query("find all nodes")
render_graph_visualization(result)
```

### Pattern 2: Batch Operations

```python
# Create multiple nodes
users = []
for i in range(100):
    user = neurograph_db.create_node(
        "user",
        {"name": f"user_{i}", "index": i}
    )
    users.append(user)

# Create edges
for i in range(len(users) - 1):
    neurograph_db.create_edge(
        users[i].id,
        users[i+1].id,
        "knows"
    )

print(f"✅ Created {len(users)} users and {len(users)-1} edges")
```

### Pattern 3: Statistical Analysis

```python
import pandas as pd

# Query data
result = neurograph_db.query("find all nodes where type='user'")

# Convert to DataFrame
df = pd.DataFrame([
    {"id": n.id, **n.properties}
    for n in result.nodes
])

# Statistics
print(f"Total users: {len(df)}")
print(f"Average age: {df['age'].mean():.1f}")
print(f"Age distribution:\n{df['age'].describe()}")

# Visualization
df['age'].hist(bins=20)
plt.xlabel("Age")
plt.ylabel("Count")
plt.title("User Age Distribution")
plt.show()
```

### Pattern 4: Time-series Data

```python
import time
import matplotlib.pyplot as plt

# Collect metrics over time
for i in range(30):
    neurograph_db.create_node(
        "metric",
        {
            "timestamp": time.time(),
            "value": random.randint(0, 100)
        }
    )
    time.sleep(1)

# Query and visualize
result = neurograph_db.query("find all nodes where type='metric'")

timestamps = [n.properties["timestamp"] for n in result.nodes]
values = [n.properties["value"] for n in result.nodes]

plt.plot(timestamps, values)
plt.xlabel("Timestamp")
plt.ylabel("Value")
plt.title("Metrics Over Time")
plt.show()
```

---

## Troubleshooting

### Extension not loading

**Problem:**
```
ModuleNotFoundError: No module named 'neurograph_jupyter'
```

**Solution:**
```bash
pip install neurograph[jupyter]
```

### Database not initialized

**Problem:**
```
❌ NeuroGraph not initialized
```

**Solution:**
```python
%neurograph init --path ./my_graph.db
```

### Rich display not working

**Problem:** Results show as plain text

**Solution:**
```python
%reload_ext neurograph_jupyter
```

### Visualization missing

**Problem:**
```
ImportError: cannot import name 'render_graph_visualization'
```

**Solution:**
```bash
pip install networkx matplotlib
```

---

## Tips & Best Practices

### 1. Initialize Once

Put at the start of your notebook:
```python
%load_ext neurograph_jupyter
%neurograph init --path ./my_graph.db
```

### 2. Use Magic Commands for Quick Queries

```python
# Good for exploration
%neurograph query "find all nodes where type='user'"
```

### 3. Use Direct API for Complex Logic

```python
# Good for processing
result = neurograph_db.query("find all nodes")
for node in result.nodes:
    # Complex processing
    ...
```

### 4. Export for Analysis

```python
# Use pandas for statistical analysis
df = pd.DataFrame([...])
df.describe()
df.plot()
```

### 5. Monitor Performance

```python
# Track slow queries
import time
start = time.perf_counter()
result = neurograph_db.query("...")
duration = time.perf_counter() - start
if duration > 1.0:
    print(f"⚠️ Slow query: {duration:.2f}s")
```

---

## Next Steps

- **Full Tutorial:** [jupyter_integration_tutorial.ipynb](../../notebooks/jupyter_integration_tutorial.ipynb)
- **Complete Documentation:** [JUPYTER_INTEGRATION.md](../jupyter/JUPYTER_INTEGRATION.md)
- **Examples:** [examples/jupyter/](../../examples/jupyter/)

---

**Quick Reference Card (v0.61.1):**

```python
# Setup
%load_ext neurograph_jupyter
%neurograph init --path ./db.db

# Auto-completion (NEW)
%neurograph <TAB>

# Query
%neurograph query "find all nodes"
result = neurograph_db.query("...")

# Query Builder (NEW)
from neurograph_jupyter.query_builder import query
result = query("nodes").where("type", "=", "user").execute()

# DataFrame Helpers (NEW)
df = result.to_dataframe()
result.export_csv("data.csv")
stats = result.summary()

# Visualization
render_graph_visualization(result)  # Static
result.plot_interactive()            # 2D Interactive (NEW)
result.plot_3d()                     # 3D Interactive (NEW)

# Real-time
%neurograph subscribe channel
%neurograph emit channel "data"

# Widgets (NEW)
from neurograph_jupyter.widgets import MetricsWidget, LogViewerWidget
widget = MetricsWidget(neurograph_ws)
display(widget)
```

**Version:** v0.61.1
