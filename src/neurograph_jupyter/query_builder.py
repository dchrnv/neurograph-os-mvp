"""
Fluent query builder for NeuroGraph queries.

Provides programmatic interface for building queries.
"""

from typing import Any, List, Optional, Union


class QueryBuilder:
    """
    Fluent interface for building NeuroGraph queries.

    Example:
        q = QueryBuilder()
        q.find("nodes").where("type", "=", "user").where("age", ">", 30)
        print(q.build())  # "find all nodes where type='user' and age > 30"
        result = q.execute()
    """

    def __init__(self, graph_ops=None):
        """
        Initialize query builder.

        Args:
            graph_ops: GraphOperations instance (optional, can be set later)
        """
        self.graph_ops = graph_ops
        self._target = None
        self._conditions = []
        self._limit = None
        self._offset = None

    def find(self, target: str = "nodes"):
        """
        Set query target.

        Args:
            target: "nodes" or "edges"

        Returns:
            Self for chaining
        """
        self._target = target
        return self

    def where(self, property_name: str, operator: str, value: Any):
        """
        Add WHERE condition.

        Args:
            property_name: Property to filter on
            operator: Comparison operator (=, !=, >, >=, <, <=, in)
            value: Value to compare against

        Returns:
            Self for chaining

        Example:
            q.where("type", "=", "user")
            q.where("age", ">", 30)
            q.where("status", "in", ["active", "pending"])
        """
        self._conditions.append((property_name, operator, value))
        return self

    def limit(self, n: int):
        """
        Limit number of results.

        Args:
            n: Maximum number of results

        Returns:
            Self for chaining
        """
        self._limit = n
        return self

    def offset(self, n: int):
        """
        Skip first N results.

        Args:
            n: Number of results to skip

        Returns:
            Self for chaining
        """
        self._offset = n
        return self

    def build(self) -> str:
        """
        Build query string.

        Returns:
            Query string

        Example:
            q = QueryBuilder()
            q.find("nodes").where("type", "=", "user")
            query = q.build()  # "find all nodes where type='user'"
        """
        if not self._target:
            raise ValueError("Must call find() first")

        query = f"find all {self._target}"

        if self._conditions:
            where_clauses = []
            for prop, op, value in self._conditions:
                if isinstance(value, str):
                    where_clauses.append(f"{prop} {op} '{value}'")
                elif isinstance(value, (list, tuple)):
                    values_str = ", ".join([f"'{v}'" if isinstance(v, str) else str(v) for v in value])
                    where_clauses.append(f"{prop} {op} ({values_str})")
                else:
                    where_clauses.append(f"{prop} {op} {value}")

            query += " where " + " and ".join(where_clauses)

        if self._limit:
            query += f" limit {self._limit}"

        if self._offset:
            query += f" offset {self._offset}"

        return query

    def execute(self):
        """
        Execute the query.

        Returns:
            QueryResult

        Raises:
            ValueError: If graph_ops not set
        """
        if not self.graph_ops:
            raise ValueError("graph_ops not set. Pass to constructor or set manually.")

        query = self.build()
        return self.graph_ops.query(query)

    def __str__(self) -> str:
        """String representation shows built query."""
        return self.build()

    def __repr__(self) -> str:
        """Repr shows class and query."""
        return f"QueryBuilder({self.build()!r})"


# Convenience function
def query(target: str = "nodes"):
    """
    Start building a query.

    Args:
        target: "nodes" or "edges"

    Returns:
        QueryBuilder instance

    Example:
        from neurograph_jupyter.query_builder import query

        q = query("nodes").where("type", "=", "user").where("age", ">", 30)
        print(q)  # Shows query string
    """
    return QueryBuilder().find(target)
