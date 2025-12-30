"""
Quick Start Example for NeuroGraph Jupyter Integration

This script demonstrates basic usage of NeuroGraph in Jupyter notebooks.
Copy these snippets into Jupyter cells to get started.
"""

# ============================================================================
# CELL 1: Load Extension
# ============================================================================
# %load_ext neurograph_jupyter


# ============================================================================
# CELL 2: Initialize Database
# ============================================================================
# %neurograph init --path ./my_graph.db


# ============================================================================
# CELL 3: Create Sample Data (Direct API)
# ============================================================================
"""
# Create users
alice = neurograph_db.create_node(
    "user",
    {"name": "Alice", "age": 30, "city": "San Francisco"}
)

bob = neurograph_db.create_node(
    "user",
    {"name": "Bob", "age": 25, "city": "New York"}
)

charlie = neurograph_db.create_node(
    "user",
    {"name": "Charlie", "age": 35, "city": "London"}
)

# Create project
project = neurograph_db.create_node(
    "project",
    {"name": "NeuroGraph", "status": "active"}
)

# Create relationships
neurograph_db.create_edge(
    alice.id,
    project.id,
    "works_on",
    {"role": "lead", "since": "2024-01-01"}
)

neurograph_db.create_edge(
    bob.id,
    project.id,
    "works_on",
    {"role": "contributor", "since": "2024-06-01"}
)

neurograph_db.create_edge(
    alice.id,
    bob.id,
    "knows",
    {"since": "2024-01-01"}
)

print("✅ Sample data created!")
"""


# ============================================================================
# CELL 4: Query All Nodes (Magic Command)
# ============================================================================
# %neurograph query "find all nodes"


# ============================================================================
# CELL 5: Filter by Type
# ============================================================================
# %neurograph query "find all nodes where type='user'"


# ============================================================================
# CELL 6: Filter by Property
# ============================================================================
# %neurograph query "find all nodes where age > 28"


# ============================================================================
# CELL 7: Check Status
# ============================================================================
# %neurograph status


# ============================================================================
# CELL 8: Visualize Graph
# ============================================================================
"""
from neurograph_jupyter.display import render_graph_visualization

result = neurograph_db.query("find all nodes")
render_graph_visualization(result, layout="spring")
"""


# ============================================================================
# CELL 9: Export to DataFrame
# ============================================================================
"""
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
df
"""


# ============================================================================
# CELL 10: Real-time Monitoring
# ============================================================================
"""
# Subscribe to metrics
%neurograph subscribe metrics

# Define handler
%%signal process_metrics
def handler(data):
    cpu = data.get("cpu", 0)
    memory = data.get("memory", 0)

    if cpu > 80:
        print(f"⚠️ High CPU: {cpu}%")
    if memory > 90:
        print(f"⚠️ High Memory: {memory}%")

    return {"status": "processed"}

# Emit test signal
%neurograph emit metrics "{'cpu': 85, 'memory': 70}"
"""


# ============================================================================
# CELL 11: Performance Monitoring
# ============================================================================
"""
import time

# Measure query performance
start = time.perf_counter()
result = neurograph_db.query("find all nodes")
duration = time.perf_counter() - start

print(f"Query returned {len(result.nodes)} nodes and {len(result.edges)} edges")
print(f"Duration: {duration*1000:.2f} ms")
print(f"Throughput: {len(result.nodes) / duration:.0f} nodes/sec")
"""


# ============================================================================
# CELL 12: Cleanup
# ============================================================================
"""
# Delete all test nodes
result = neurograph_db.query("find all nodes")
for node in result.nodes:
    neurograph_db.delete_node(node.id)

print("✅ Test data cleaned up")
"""


if __name__ == "__main__":
    print("=" * 70)
    print("NeuroGraph Jupyter Quick Start")
    print("=" * 70)
    print()
    print("This file contains code snippets for Jupyter notebooks.")
    print("Copy each CELL section into a separate Jupyter cell.")
    print()
    print("Quick setup:")
    print("1. pip install neurograph[jupyter]")
    print("2. jupyter notebook")
    print("3. Copy cells from this file")
    print()
    print("📚 Full tutorial: notebooks/jupyter_integration_tutorial.ipynb")
    print("📖 Documentation: docs/jupyter/JUPYTER_INTEGRATION.md")
    print("=" * 70)
