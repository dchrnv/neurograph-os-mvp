"""
Helper functions and extensions for NeuroGraph Jupyter integration.

Provides convenient methods for working with query results.
"""

from typing import Any, Dict, List, Optional
import json


class QueryResultExtensions:
    """
    Extension methods for QueryResult objects.

    These methods are monkey-patched onto QueryResult to provide
    convenient DataFrame export and visualization helpers.
    """

    @staticmethod
    def to_dataframe(result, include_properties: bool = True):
        """
        Convert QueryResult to pandas DataFrame.

        Args:
            result: QueryResult object
            include_properties: If True, expand properties as columns

        Returns:
            pandas.DataFrame with node/edge data

        Example:
            result = %neurograph query "find all nodes where type='user'"
            df = result.to_dataframe()
            df.head()
        """
        try:
            import pandas as pd
        except ImportError:
            raise ImportError("pandas is required for to_dataframe(). Install with: pip install pandas")

        data = []

        if hasattr(result, 'nodes') and result.nodes:
            for node in result.nodes:
                row = {
                    'id': node.id,
                    'type': getattr(node, 'type', None)
                }

                if include_properties and hasattr(node, 'properties') and node.properties:
                    # Expand properties as columns
                    row.update(node.properties)
                else:
                    # Keep properties as dict column
                    row['properties'] = getattr(node, 'properties', {})

                data.append(row)

        elif hasattr(result, 'edges') and result.edges:
            for edge in result.edges:
                row = {
                    'source_id': getattr(edge, 'source_id', None),
                    'target_id': getattr(edge, 'target_id', None),
                    'type': getattr(edge, 'type', None)
                }

                if include_properties and hasattr(edge, 'properties') and edge.properties:
                    row.update(edge.properties)
                else:
                    row['properties'] = getattr(edge, 'properties', {})

                data.append(row)

        return pd.DataFrame(data)

    @staticmethod
    def to_dict_list(result) -> List[Dict[str, Any]]:
        """
        Convert QueryResult to list of dictionaries.

        Args:
            result: QueryResult object

        Returns:
            List of dictionaries with node/edge data
        """
        data = []

        if hasattr(result, 'nodes') and result.nodes:
            for node in result.nodes:
                data.append({
                    'id': node.id,
                    'type': getattr(node, 'type', None),
                    'properties': getattr(node, 'properties', {})
                })

        elif hasattr(result, 'edges') and result.edges:
            for edge in result.edges:
                data.append({
                    'source_id': getattr(edge, 'source_id', None),
                    'target_id': getattr(edge, 'target_id', None),
                    'type': getattr(edge, 'type', None),
                    'properties': getattr(edge, 'properties', {})
                })

        return data

    @staticmethod
    def export_csv(result, filename: str, include_properties: bool = True):
        """
        Export QueryResult to CSV file.

        Args:
            result: QueryResult object
            filename: Output CSV filename
            include_properties: If True, expand properties as columns

        Example:
            result = %neurograph query "find all nodes"
            result.export_csv("nodes.csv")
        """
        df = QueryResultExtensions.to_dataframe(result, include_properties)
        df.to_csv(filename, index=False)
        print(f"✅ Exported {len(df)} rows to {filename}")

    @staticmethod
    def export_json(result, filename: str, indent: int = 2):
        """
        Export QueryResult to JSON file.

        Args:
            result: QueryResult object
            filename: Output JSON filename
            indent: JSON indentation (default: 2)

        Example:
            result = %neurograph query "find all nodes"
            result.export_json("nodes.json")
        """
        data = QueryResultExtensions.to_dict_list(result)

        with open(filename, 'w') as f:
            json.dump(data, f, indent=indent)

        print(f"✅ Exported {len(data)} items to {filename}")

    @staticmethod
    def plot_distribution(result, column: str, bins: int = 20, figsize: tuple = (10, 6)):
        """
        Plot histogram of a property distribution.

        Args:
            result: QueryResult object
            column: Property name to plot
            bins: Number of histogram bins
            figsize: Figure size (width, height)

        Example:
            result = %neurograph query "find all nodes where type='user'"
            result.plot_distribution("age")
        """
        try:
            import matplotlib.pyplot as plt
        except ImportError:
            raise ImportError("matplotlib is required for plot_distribution(). Install with: pip install matplotlib")

        df = QueryResultExtensions.to_dataframe(result, include_properties=True)

        if column not in df.columns:
            print(f"❌ Column '{column}' not found")
            print(f"Available columns: {', '.join(df.columns)}")
            return

        plt.figure(figsize=figsize)
        df[column].hist(bins=bins, edgecolor='black')
        plt.xlabel(column)
        plt.ylabel('Count')
        plt.title(f'Distribution of {column}')
        plt.grid(True, alpha=0.3)
        plt.show()

    @staticmethod
    def summary(result) -> Dict[str, Any]:
        """
        Get summary statistics for QueryResult.

        Args:
            result: QueryResult object

        Returns:
            Dictionary with summary statistics

        Example:
            result = %neurograph query "find all nodes"
            stats = result.summary()
            print(stats)
        """
        summary = {}

        if hasattr(result, 'nodes') and result.nodes:
            summary['node_count'] = len(result.nodes)

            # Count by type
            types = {}
            for node in result.nodes:
                node_type = getattr(node, 'type', 'unknown')
                types[node_type] = types.get(node_type, 0) + 1
            summary['node_types'] = types

            # Property statistics
            all_props = set()
            for node in result.nodes:
                if hasattr(node, 'properties') and node.properties:
                    all_props.update(node.properties.keys())
            summary['unique_properties'] = len(all_props)
            summary['property_names'] = sorted(list(all_props))

        if hasattr(result, 'edges') and result.edges:
            summary['edge_count'] = len(result.edges)

            # Count by type
            types = {}
            for edge in result.edges:
                edge_type = getattr(edge, 'type', 'unknown')
                types[edge_type] = types.get(edge_type, 0) + 1
            summary['edge_types'] = types

        return summary


def install_result_extensions():
    """
    Install extension methods on QueryResult class.

    This is called automatically when the extension loads.
    """
    try:
        from neurograph.query.result import QueryResult

        # Monkey-patch methods onto QueryResult
        QueryResult.to_dataframe = lambda self, **kwargs: QueryResultExtensions.to_dataframe(self, **kwargs)
        QueryResult.to_dict_list = lambda self: QueryResultExtensions.to_dict_list(self)
        QueryResult.export_csv = lambda self, *args, **kwargs: QueryResultExtensions.export_csv(self, *args, **kwargs)
        QueryResult.export_json = lambda self, *args, **kwargs: QueryResultExtensions.export_json(self, *args, **kwargs)
        QueryResult.plot_distribution = lambda self, *args, **kwargs: QueryResultExtensions.plot_distribution(self, *args, **kwargs)
        QueryResult.summary = lambda self: QueryResultExtensions.summary(self)

        return True
    except ImportError:
        # QueryResult not available, extension methods won't work
        return False
