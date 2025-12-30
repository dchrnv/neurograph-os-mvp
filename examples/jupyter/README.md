# Jupyter Examples

Quick examples for NeuroGraph Jupyter Integration.

## Files

### [quick_start.py](quick_start.py)

Basic usage examples covering:
- Extension loading
- Database initialization
- Creating nodes and edges
- Querying data
- Visualization
- DataFrame export
- Real-time signals
- Performance monitoring

**Usage:**
1. Open Jupyter notebook
2. Copy cells from this file
3. Execute sequentially

### [real_time_dashboard.py](real_time_dashboard.py)

Real-time metrics monitoring dashboard:
- Live signal processing
- Metric collection and storage
- Threshold alerting
- Time-series visualization
- Statistical analysis
- Data export (CSV, reports)

**Usage:**
1. Copy cells into Jupyter notebook
2. Run simulation for 30 seconds
3. Analyze collected metrics
4. Export reports

## Quick Setup

```bash
# Install with Jupyter support
pip install neurograph[jupyter]

# Start Jupyter
jupyter notebook
```

## In Jupyter Notebook

```python
# Load extension
%load_ext neurograph_jupyter

# Initialize
%neurograph init --path ./my_graph.db

# Start using!
%neurograph query "find all nodes"
```

## Full Documentation

- **Tutorial:** [notebooks/jupyter_integration_tutorial.ipynb](../../notebooks/jupyter_integration_tutorial.ipynb)
- **User Guide:** [docs/jupyter/JUPYTER_INTEGRATION.md](../../docs/jupyter/JUPYTER_INTEGRATION.md)
- **API Reference:** Included in user guide

## Magic Commands

### Line Magic: `%neurograph`

```python
%neurograph init --path <db_path>      # Initialize
%neurograph status                      # Show status
%neurograph query "<query>"             # Execute query
%neurograph subscribe <channel>         # Subscribe
%neurograph emit <channel> <data>       # Emit signal
```

### Cell Magic: `%%signal`

```python
%%signal my_handler
def handler(data):
    # Process data
    return result
```

## Tips

1. **Use magic commands for quick operations**
   ```python
   %neurograph query "find all nodes where type='user'"
   ```

2. **Use direct API for complex logic**
   ```python
   for i in range(100):
       neurograph_db.create_node("user", {"index": i})
   ```

3. **Export to DataFrame for analysis**
   ```python
   import pandas as pd
   df = pd.DataFrame([...])
   df.describe()
   ```

4. **Visualize with NetworkX**
   ```python
   from neurograph_jupyter.display import render_graph_visualization
   render_graph_visualization(result, layout="spring")
   ```

## Examples by Use Case

### Data Exploration
- Load data
- Query with filters
- Visualize relationships
- Export to DataFrame

### Real-time Monitoring
- Subscribe to channels
- Define signal handlers
- Process events
- Generate alerts

### Performance Analysis
- Benchmark queries
- Monitor throughput
- Measure latency
- Track statistics

### Batch Processing
- Create multiple nodes
- Build relationships
- Transform data
- Export results

## Need Help?

- Check the [full tutorial](../../notebooks/jupyter_integration_tutorial.ipynb)
- Read the [user guide](../../docs/jupyter/JUPYTER_INTEGRATION.md)
- Review the [completion report](../../docs/completion/V0.61.0_COMPLETION.md)
