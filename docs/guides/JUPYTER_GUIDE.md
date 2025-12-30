# Jupyter Integration Guide

**Quick practical guide for using NeuroGraph in Jupyter notebooks**

Version: v0.61.0

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

```python
from neurograph_jupyter.display import render_graph_visualization

result = neurograph_db.query("find all nodes")
render_graph_visualization(result, layout="spring")
```

**Layouts:**
- `spring` - Force-directed (default)
- `circular` - Circular arrangement
- `kamada_kawai` - Energy minimization

---

## Export to DataFrame

```python
import pandas as pd

result = neurograph_db.query("find all nodes where type='user'")

# Convert to DataFrame
df = pd.DataFrame([
    {
        "id": node.id,
        "name": node.properties.get("name"),
        "age": node.properties.get("age"),
        "city": node.properties.get("city")
    }
    for node in result.nodes
])

# Analyze
df.describe()
df[df['age'] > 30]
df.to_csv("users.csv", index=False)
```

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

**Quick Reference Card:**

```python
# Setup
%load_ext neurograph_jupyter
%neurograph init --path ./db.db

# Query
%neurograph query "find all nodes"
result = neurograph_db.query("...")

# Real-time
%neurograph subscribe channel
%neurograph emit channel "data"

# Visualize
render_graph_visualization(result)

# Export
df = pd.DataFrame([...])
```

**Version:** v0.61.0
