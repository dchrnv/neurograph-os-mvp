"""
Real-time Metrics Dashboard Example

Demonstrates real-time signal processing and visualization in Jupyter.
"""

# ============================================================================
# CELL 1: Setup
# ============================================================================
"""
%load_ext neurograph_jupyter
%neurograph init --path ./metrics.db
%neurograph subscribe metrics
"""


# ============================================================================
# CELL 2: Import Libraries
# ============================================================================
"""
import time
import random
from datetime import datetime
import matplotlib.pyplot as plt
from IPython.display import clear_output
"""


# ============================================================================
# CELL 3: Define Metrics Processor
# ============================================================================
"""
%%signal process_metrics
def handler(data):
    # Extract metrics
    timestamp = data.get("timestamp", time.time())
    cpu = data.get("cpu", 0)
    memory = data.get("memory", 0)
    network = data.get("network", 0)

    # Store in database as nodes
    metric_node = neurograph_db.create_node(
        "metric",
        {
            "timestamp": timestamp,
            "cpu": cpu,
            "memory": memory,
            "network": network,
            "datetime": datetime.fromtimestamp(timestamp).isoformat()
        }
    )

    # Alert on thresholds
    alerts = []
    if cpu > 80:
        alerts.append(f"🔴 HIGH CPU: {cpu}%")
    if memory > 90:
        alerts.append(f"🔴 HIGH MEMORY: {memory}%")
    if network > 1000:  # MB/s
        alerts.append(f"🔴 HIGH NETWORK: {network} MB/s")

    if alerts:
        print("\\n".join(alerts))
    else:
        print(f"✅ Metrics OK - CPU: {cpu}%, MEM: {memory}%, NET: {network} MB/s")

    return {"stored": metric_node.id, "alerts": len(alerts)}
"""


# ============================================================================
# CELL 4: Simulate Live Metrics
# ============================================================================
"""
# Simulate metrics for 30 seconds
print("Simulating live metrics for 30 seconds...")

for i in range(30):
    # Generate random metrics
    metrics = {
        "timestamp": time.time(),
        "cpu": random.randint(20, 95),
        "memory": random.randint(40, 95),
        "network": random.randint(10, 1200)
    }

    # Emit signal
    %neurograph emit metrics str(metrics)

    time.sleep(1)

print("\\n✅ Simulation complete!")
"""


# ============================================================================
# CELL 5: Query Recent Metrics
# ============================================================================
"""
# Get last 30 metrics
result = neurograph_db.query("find all nodes where type='metric'")

print(f"Total metrics collected: {len(result.nodes)}")

# Show last 5
import pandas as pd

data = []
for node in result.nodes[-5:]:
    data.append({
        "time": node.properties["datetime"],
        "cpu": node.properties["cpu"],
        "memory": node.properties["memory"],
        "network": node.properties["network"]
    })

df = pd.DataFrame(data)
df
"""


# ============================================================================
# CELL 6: Visualize Metrics
# ============================================================================
"""
import matplotlib.pyplot as plt

# Extract data
result = neurograph_db.query("find all nodes where type='metric'")

timestamps = [n.properties["timestamp"] for n in result.nodes]
cpu_values = [n.properties["cpu"] for n in result.nodes]
memory_values = [n.properties["memory"] for n in result.nodes]
network_values = [n.properties["network"] for n in result.nodes]

# Create subplots
fig, axes = plt.subplots(3, 1, figsize=(12, 8))

# CPU
axes[0].plot(timestamps, cpu_values, 'b-', linewidth=2)
axes[0].axhline(y=80, color='r', linestyle='--', label='Threshold')
axes[0].set_ylabel('CPU %')
axes[0].set_title('CPU Usage Over Time')
axes[0].legend()
axes[0].grid(True, alpha=0.3)

# Memory
axes[1].plot(timestamps, memory_values, 'g-', linewidth=2)
axes[1].axhline(y=90, color='r', linestyle='--', label='Threshold')
axes[1].set_ylabel('Memory %')
axes[1].set_title('Memory Usage Over Time')
axes[1].legend()
axes[1].grid(True, alpha=0.3)

# Network
axes[2].plot(timestamps, network_values, 'orange', linewidth=2)
axes[2].axhline(y=1000, color='r', linestyle='--', label='Threshold')
axes[2].set_ylabel('Network (MB/s)')
axes[2].set_xlabel('Timestamp')
axes[2].set_title('Network Usage Over Time')
axes[2].legend()
axes[2].grid(True, alpha=0.3)

plt.tight_layout()
plt.show()
"""


# ============================================================================
# CELL 7: Statistics
# ============================================================================
"""
import pandas as pd
import numpy as np

# Get all metrics
result = neurograph_db.query("find all nodes where type='metric'")

data = []
for node in result.nodes:
    data.append({
        "cpu": node.properties["cpu"],
        "memory": node.properties["memory"],
        "network": node.properties["network"]
    })

df = pd.DataFrame(data)

print("📊 Metrics Statistics\\n")
print("CPU:")
print(f"  Average: {df['cpu'].mean():.1f}%")
print(f"  Max: {df['cpu'].max():.1f}%")
print(f"  Min: {df['cpu'].min():.1f}%")
print(f"  P95: {df['cpu'].quantile(0.95):.1f}%")

print("\\nMemory:")
print(f"  Average: {df['memory'].mean():.1f}%")
print(f"  Max: {df['memory'].max():.1f}%")
print(f"  Min: {df['memory'].min():.1f}%")
print(f"  P95: {df['memory'].quantile(0.95):.1f}%")

print("\\nNetwork:")
print(f"  Average: {df['network'].mean():.1f} MB/s")
print(f"  Max: {df['network'].max():.1f} MB/s")
print(f"  Min: {df['network'].min():.1f} MB/s")
print(f"  P95: {df['network'].quantile(0.95):.1f} MB/s")

# Count alerts
cpu_alerts = len(df[df['cpu'] > 80])
memory_alerts = len(df[df['memory'] > 90])
network_alerts = len(df[df['network'] > 1000])

print(f"\\n🚨 Alert Summary:")
print(f"  CPU alerts: {cpu_alerts}")
print(f"  Memory alerts: {memory_alerts}")
print(f"  Network alerts: {network_alerts}")
print(f"  Total: {cpu_alerts + memory_alerts + network_alerts}")
"""


# ============================================================================
# CELL 8: Export Report
# ============================================================================
"""
# Export to CSV
df.to_csv("metrics_report.csv", index=False)
print("✅ Report exported to metrics_report.csv")

# Export statistics
with open("metrics_stats.txt", "w") as f:
    f.write("NeuroGraph Metrics Report\\n")
    f.write("=" * 50 + "\\n\\n")
    f.write(f"Total samples: {len(df)}\\n\\n")
    f.write("CPU Statistics:\\n")
    f.write(f"  Average: {df['cpu'].mean():.1f}%\\n")
    f.write(f"  Max: {df['cpu'].max():.1f}%\\n")
    f.write(f"  P95: {df['cpu'].quantile(0.95):.1f}%\\n\\n")
    f.write(f"CPU Alerts: {cpu_alerts}\\n")
    f.write(f"Memory Alerts: {memory_alerts}\\n")
    f.write(f"Network Alerts: {network_alerts}\\n")

print("✅ Statistics exported to metrics_stats.txt")
"""


# ============================================================================
# CELL 9: Cleanup
# ============================================================================
"""
# Delete all metrics
result = neurograph_db.query("find all nodes where type='metric'")
for node in result.nodes:
    neurograph_db.delete_node(node.id)

print(f"✅ Cleaned up {len(result.nodes)} metric nodes")
"""


if __name__ == "__main__":
    print("=" * 70)
    print("Real-time Metrics Dashboard Example")
    print("=" * 70)
    print()
    print("This example demonstrates:")
    print("  • Real-time signal processing")
    print("  • Metric collection and storage")
    print("  • Threshold alerting")
    print("  • Time-series visualization")
    print("  • Statistical analysis")
    print("  • Data export")
    print()
    print("Copy each CELL section into a separate Jupyter cell.")
    print()
    print("📚 Full tutorial: notebooks/jupyter_integration_tutorial.ipynb")
    print("=" * 70)
