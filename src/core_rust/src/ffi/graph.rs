//! Python bindings for Graph V2.0 structure

use crate::graph::{Direction, EdgeId, Graph, GraphConfig, NodeId, Path, Subgraph};
use pyo3::prelude::*;
use std::collections::HashSet;

/// Python wrapper for GraphConfig
#[pyclass(name = "GraphConfig")]
#[derive(Clone)]
pub struct PyGraphConfig {
    pub(crate) inner: GraphConfig,
}

#[pymethods]
impl PyGraphConfig {
    #[new]
    #[pyo3(signature = (deduplicate_edges=false, initial_capacity=1000))]
    fn new(deduplicate_edges: bool, initial_capacity: usize) -> Self {
        PyGraphConfig {
            inner: GraphConfig {
                deduplicate_edges,
                initial_capacity,
            },
        }
    }

    #[getter]
    fn deduplicate_edges(&self) -> bool {
        self.inner.deduplicate_edges
    }

    #[setter]
    fn set_deduplicate_edges(&mut self, value: bool) {
        self.inner.deduplicate_edges = value;
    }

    #[getter]
    fn initial_capacity(&self) -> usize {
        self.inner.initial_capacity
    }

    #[setter]
    fn set_initial_capacity(&mut self, value: usize) {
        self.inner.initial_capacity = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "GraphConfig(deduplicate_edges={}, initial_capacity={})",
            self.inner.deduplicate_edges, self.inner.initial_capacity
        )
    }
}

/// Python wrapper for Path
#[pyclass(name = "Path")]
#[derive(Clone)]
pub struct PyPath {
    pub(crate) inner: Path,
}

#[pymethods]
impl PyPath {
    #[getter]
    fn nodes(&self) -> Vec<NodeId> {
        self.inner.nodes.clone()
    }

    #[getter]
    fn edges(&self) -> Vec<EdgeId> {
        self.inner.edges.clone()
    }

    #[getter]
    fn total_cost(&self) -> f32 {
        self.inner.total_cost
    }

    #[getter]
    fn length(&self) -> usize {
        self.inner.length
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    fn contains_node(&self, node_id: NodeId) -> bool {
        self.inner.contains_node(node_id)
    }

    fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.inner.contains_edge(edge_id)
    }

    fn __repr__(&self) -> String {
        format!(
            "Path(nodes={}, length={}, cost={:.2})",
            self.inner.nodes.len(),
            self.inner.length,
            self.inner.total_cost
        )
    }

    fn __len__(&self) -> usize {
        self.inner.length
    }
}

/// Python wrapper for Subgraph
#[pyclass(name = "Subgraph")]
#[derive(Clone)]
pub struct PySubgraph {
    pub(crate) inner: Subgraph,
}

#[pymethods]
impl PySubgraph {
    #[new]
    fn new() -> Self {
        PySubgraph {
            inner: Subgraph::new(),
        }
    }

    #[getter]
    fn nodes(&self) -> Vec<NodeId> {
        self.inner.nodes.iter().copied().collect()
    }

    #[getter]
    fn edges(&self) -> Vec<EdgeId> {
        self.inner.edges.iter().copied().collect()
    }

    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    fn contains_node(&self, node_id: NodeId) -> bool {
        self.inner.contains_node(node_id)
    }

    fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.inner.contains_edge(edge_id)
    }

    fn __repr__(&self) -> String {
        format!(
            "Subgraph(nodes={}, edges={})",
            self.inner.node_count(),
            self.inner.edge_count()
        )
    }

    fn __len__(&self) -> usize {
        self.inner.node_count()
    }
}

/// Python wrapper for Graph V2.0
#[pyclass(name = "Graph")]
pub struct PyGraph {
    inner: Graph,
}

#[pymethods]
impl PyGraph {
    /// Create a new Graph with optional configuration
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyGraphConfig>) -> Self {
        PyGraph {
            inner: match config {
                Some(cfg) => Graph::with_config(cfg.inner),
                None => Graph::new(),
            },
        }
    }

    // ==================== NODE OPERATIONS ====================

    /// Add a node to the graph
    fn add_node(&mut self, node_id: NodeId) -> bool {
        self.inner.add_node(node_id)
    }

    /// Remove a node from the graph
    fn remove_node(&mut self, node_id: NodeId) -> bool {
        self.inner.remove_node(node_id)
    }

    /// Check if node exists
    fn contains_node(&self, node_id: NodeId) -> bool {
        self.inner.contains_node(node_id)
    }

    /// Get number of nodes
    fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// Get all node IDs
    fn get_nodes(&self) -> Vec<NodeId> {
        self.inner.get_nodes()
    }

    // ==================== EDGE OPERATIONS ====================

    /// Compute edge ID from connection parameters
    #[staticmethod]
    fn compute_edge_id(from_id: NodeId, to_id: NodeId, edge_type: u8) -> EdgeId {
        Graph::compute_edge_id(from_id, to_id, edge_type)
    }

    /// Add an edge to the graph
    fn add_edge(
        &mut self,
        edge_id: EdgeId,
        from_id: NodeId,
        to_id: NodeId,
        edge_type: u8,
        weight: f32,
        bidirectional: bool,
    ) -> PyResult<bool> {
        self.inner
            .add_edge(edge_id, from_id, to_id, edge_type, weight, bidirectional)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Remove an edge from the graph
    fn remove_edge(&mut self, edge_id: EdgeId) -> bool {
        self.inner.remove_edge(edge_id)
    }

    /// Check if edge exists
    fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.inner.contains_edge(edge_id)
    }

    /// Get number of edges
    fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    /// Get neighbors of a node
    /// direction: 0 = Outgoing, 1 = Incoming, 2 = Both
    fn get_neighbors(&self, node_id: NodeId, direction: u8) -> Vec<(NodeId, EdgeId)> {
        let dir = match direction {
            0 => Direction::Outgoing,
            1 => Direction::Incoming,
            _ => Direction::Both,
        };
        self.inner.get_neighbors(node_id, dir)
    }

    /// Get degree of a node
    /// direction: 0 = Outgoing, 1 = Incoming, 2 = Both
    fn get_degree(&self, node_id: NodeId, direction: u8) -> usize {
        let dir = match direction {
            0 => Direction::Outgoing,
            1 => Direction::Incoming,
            _ => Direction::Both,
        };
        self.inner.get_degree(node_id, dir)
    }

    // ==================== TRAVERSAL ====================

    /// BFS traversal
    /// Returns list of (node_id, depth) tuples
    fn bfs(&self, start_id: NodeId, max_depth: Option<usize>) -> Vec<(NodeId, usize)> {
        let mut result = Vec::new();
        self.inner.bfs(start_id, max_depth, |node_id, depth| {
            result.push((node_id, depth));
        });
        result
    }

    /// DFS traversal
    /// Returns list of (node_id, depth) tuples
    fn dfs(&self, start_id: NodeId, max_depth: Option<usize>) -> Vec<(NodeId, usize)> {
        let mut result = Vec::new();
        self.inner.dfs(start_id, max_depth, |node_id, depth| {
            result.push((node_id, depth));
        });
        result
    }

    // ==================== PATHFINDING ====================

    /// Find shortest path (unweighted BFS)
    fn find_path(&self, from_id: NodeId, to_id: NodeId) -> Option<PyPath> {
        self.inner
            .find_path(from_id, to_id)
            .map(|path| PyPath { inner: path })
    }

    /// Find shortest weighted path (Dijkstra)
    fn dijkstra(&self, from_id: NodeId, to_id: NodeId) -> Option<PyPath> {
        self.inner
            .dijkstra(from_id, to_id)
            .map(|path| PyPath { inner: path })
    }

    // ==================== SUBGRAPHS ====================

    /// Extract subgraph from node set
    fn extract_subgraph(&self, node_ids: Vec<NodeId>) -> PySubgraph {
        let nodes: HashSet<NodeId> = node_ids.into_iter().collect();
        PySubgraph {
            inner: self.inner.extract_subgraph(&nodes),
        }
    }

    /// Extract ego-network (neighborhood)
    fn extract_neighborhood(&self, center_id: NodeId, radius: usize) -> PySubgraph {
        PySubgraph {
            inner: self.inner.extract_neighborhood(center_id, radius),
        }
    }

    // ==================== UTILITY ====================

    /// Clear all nodes and edges
    fn clear(&mut self) {
        self.inner.clear();
    }

    fn __len__(&self) -> usize {
        self.inner.node_count()
    }

    fn __repr__(&self) -> String {
        format!(
            "Graph(nodes={}, edges={})",
            self.inner.node_count(),
            self.inner.edge_count()
        )
    }
}

/// Helper function to create a simple graph for testing
#[pyfunction]
pub fn create_simple_graph(num_nodes: usize, num_edges: usize) -> PyGraph {
    let mut graph = Graph::new();

    // Add nodes
    for i in 1..=(num_nodes as u32) {
        graph.add_node(i);
    }

    // Add edges (simple chain for now)
    for i in 1..=(num_edges.min(num_nodes - 1) as u32) {
        let edge_id = Graph::compute_edge_id(i, i + 1, 0);
        let _ = graph.add_edge(edge_id, i, i + 1, 0, 1.0, false);
    }

    PyGraph { inner: graph }
}
