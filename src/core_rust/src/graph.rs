use crate::Connection;
use std::cmp::Ordering;
/// Graph V2.0 - Topological navigation for NeuroGraph OS
///
/// Graph provides topological indexing and navigation over Token connections.
/// Unlike Grid (spatial indexing), Graph handles topology: paths, neighbors, subgraphs.
///
/// # Architecture
///
/// - NodeId: u32 (references Token.id)
/// - EdgeId: u64 (hash of connection identifier)
/// - Adjacency lists for O(1) neighbor access
/// - Directed graph support (in/out edges)
/// - Integration with Grid and Connection
///
/// # Key Operations
///
/// - Topology: add_node, add_edge, get_neighbors
/// - Traversal: BFS, DFS
/// - Pathfinding: shortest_path (BFS), dijkstra
/// - Subgraphs: extract_subgraph, extract_neighborhood
///
/// # Memory Layout
///
/// - adjacency_out: HashMap<NodeId, Vec<EdgeId>> - outgoing edges
/// - adjacency_in: HashMap<NodeId, Vec<EdgeId>> - incoming edges
/// - edge_map: HashMap<EdgeId, EdgeInfo> - edge metadata
///
/// Total memory: ~50 bytes per node + ~40 bytes per edge
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// Node identifier (Token.id)
pub type NodeId = u32;

/// Edge identifier (hash of connection)
pub type EdgeId = u64;

/// Direction for neighbor queries
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Outgoing, // Only outgoing edges
    Incoming, // Only incoming edges
    Both,     // Both directions
}

/// Edge metadata stored in graph
#[derive(Debug, Clone)]
pub struct EdgeInfo {
    pub from_id: NodeId,
    pub to_id: NodeId,
    pub edge_type: u8,       // Connection type
    pub weight: f32,         // Connection weight (for pathfinding)
    pub bidirectional: bool, // Whether edge can be traversed both ways
}

/// Path through the graph
#[derive(Debug, Clone)]
pub struct Path {
    pub nodes: Vec<NodeId>,
    pub edges: Vec<EdgeId>,
    pub total_cost: f32,
    pub length: usize, // Number of hops
}

impl Path {
    /// Create empty path
    pub fn empty() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            total_cost: 0.0,
            length: 0,
        }
    }

    /// Check if path is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Check if path is valid (edges.len() == nodes.len() - 1)
    pub fn is_valid(&self) -> bool {
        if self.nodes.is_empty() {
            return self.edges.is_empty();
        }
        self.edges.len() == self.nodes.len() - 1
    }

    /// Check if path contains node
    pub fn contains_node(&self, node_id: NodeId) -> bool {
        self.nodes.contains(&node_id)
    }

    /// Check if path contains edge
    pub fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.edges.contains(&edge_id)
    }
}

/// Subgraph (induced subgraph from node set)
#[derive(Debug, Clone)]
pub struct Subgraph {
    pub nodes: HashSet<NodeId>,
    pub edges: HashSet<EdgeId>,
}

impl Subgraph {
    /// Create empty subgraph
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Check if contains node
    pub fn contains_node(&self, node_id: NodeId) -> bool {
        self.nodes.contains(&node_id)
    }

    /// Check if contains edge
    pub fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.edges.contains(&edge_id)
    }
}

/// Graph configuration
#[derive(Debug, Clone)]
pub struct GraphConfig {
    /// Enable edge deduplication (slower insert, less memory)
    pub deduplicate_edges: bool,
    /// Pre-allocate capacity for nodes
    pub initial_capacity: usize,
}

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            deduplicate_edges: false,
            initial_capacity: 1000,
        }
    }
}

/// State for Dijkstra's algorithm priority queue
#[derive(Debug, Clone)]
struct DijkstraState {
    cost: f32,
    node: NodeId,
}

// Implement Ord for min-heap (reverse order for BinaryHeap)
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order: lower cost = higher priority
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node == other.node
    }
}

impl Eq for DijkstraState {}

/// Graph V2.0 - Topological indexing and navigation
///
/// # Example
///
/// ```rust
/// use neurograph_core::{Graph, GraphConfig};
///
/// let mut graph = Graph::new();
///
/// // Add nodes
/// graph.add_node(1);
/// graph.add_node(2);
///
/// // Add edge
/// let edge_id = Graph::compute_edge_id(1, 2, 0);
/// graph.add_edge(edge_id, 1, 2, 0, 1.0, false);
///
/// // Find neighbors
/// let neighbors = graph.get_neighbors(1, Direction::Outgoing);
/// assert_eq!(neighbors.len(), 1);
/// ```
pub struct Graph {
    config: GraphConfig,
    /// Outgoing edges for each node
    adjacency_out: HashMap<NodeId, Vec<EdgeId>>,
    /// Incoming edges for each node
    adjacency_in: HashMap<NodeId, Vec<EdgeId>>,
    /// Edge metadata
    edge_map: HashMap<EdgeId, EdgeInfo>,
}

impl Graph {
    /// Create new empty graph
    pub fn new() -> Self {
        Self::with_config(GraphConfig::default())
    }

    /// Create graph with custom configuration
    pub fn with_config(config: GraphConfig) -> Self {
        let capacity = config.initial_capacity;
        Self {
            config,
            adjacency_out: HashMap::with_capacity(capacity),
            adjacency_in: HashMap::with_capacity(capacity),
            edge_map: HashMap::new(),
        }
    }

    /// Compute edge ID from connection parameters
    /// Uses FNV-1a hash for speed
    pub fn compute_edge_id(from_id: NodeId, to_id: NodeId, edge_type: u8) -> EdgeId {
        // FNV-1a hash
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;

        // Hash from_id
        hash ^= (from_id & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((from_id >> 8) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((from_id >> 16) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((from_id >> 24) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash to_id
        hash ^= (to_id & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((to_id >> 8) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((to_id >> 16) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        hash ^= ((to_id >> 24) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        // Hash edge_type
        hash ^= edge_type as u64;
        hash = hash.wrapping_mul(FNV_PRIME);

        hash
    }

    /// Add node to graph
    /// Returns true if node was added, false if already exists
    pub fn add_node(&mut self, node_id: NodeId) -> bool {
        if self.adjacency_out.contains_key(&node_id) {
            return false;
        }

        self.adjacency_out.insert(node_id, Vec::new());
        self.adjacency_in.insert(node_id, Vec::new());
        true
    }

    /// Remove node from graph
    /// Also removes all edges connected to this node
    /// Returns true if node was removed
    pub fn remove_node(&mut self, node_id: NodeId) -> bool {
        if !self.adjacency_out.contains_key(&node_id) {
            return false;
        }

        // Collect all edges to remove
        let mut edges_to_remove = Vec::new();

        if let Some(out_edges) = self.adjacency_out.get(&node_id) {
            edges_to_remove.extend(out_edges.iter().copied());
        }

        if let Some(in_edges) = self.adjacency_in.get(&node_id) {
            edges_to_remove.extend(in_edges.iter().copied());
        }

        // Remove all edges
        for edge_id in edges_to_remove {
            self.remove_edge(edge_id);
        }

        // Remove node
        self.adjacency_out.remove(&node_id);
        self.adjacency_in.remove(&node_id);

        true
    }

    /// Check if node exists in graph
    pub fn contains_node(&self, node_id: NodeId) -> bool {
        self.adjacency_out.contains_key(&node_id)
    }

    /// Get number of nodes
    pub fn node_count(&self) -> usize {
        self.adjacency_out.len()
    }

    /// Add edge to graph
    /// Both nodes must already exist
    /// Returns true if edge was added
    pub fn add_edge(
        &mut self,
        edge_id: EdgeId,
        from_id: NodeId,
        to_id: NodeId,
        edge_type: u8,
        weight: f32,
        bidirectional: bool,
    ) -> Result<bool, String> {
        // Check nodes exist
        if !self.contains_node(from_id) {
            return Err(format!("Node {} does not exist", from_id));
        }
        if !self.contains_node(to_id) {
            return Err(format!("Node {} does not exist", to_id));
        }

        // Check if edge already exists
        if self.edge_map.contains_key(&edge_id) {
            return Ok(false);
        }

        // Add edge metadata
        let edge_info = EdgeInfo {
            from_id,
            to_id,
            edge_type,
            weight,
            bidirectional,
        };
        self.edge_map.insert(edge_id, edge_info);

        // Add to adjacency lists
        self.adjacency_out.get_mut(&from_id).unwrap().push(edge_id);

        self.adjacency_in.get_mut(&to_id).unwrap().push(edge_id);

        Ok(true)
    }

    /// Remove edge from graph
    /// Returns true if edge was removed
    pub fn remove_edge(&mut self, edge_id: EdgeId) -> bool {
        if let Some(edge_info) = self.edge_map.remove(&edge_id) {
            // Remove from adjacency lists
            if let Some(out_edges) = self.adjacency_out.get_mut(&edge_info.from_id) {
                out_edges.retain(|&e| e != edge_id);
            }

            if let Some(in_edges) = self.adjacency_in.get_mut(&edge_info.to_id) {
                in_edges.retain(|&e| e != edge_id);
            }

            true
        } else {
            false
        }
    }

    /// Check if edge exists
    pub fn contains_edge(&self, edge_id: EdgeId) -> bool {
        self.edge_map.contains_key(&edge_id)
    }

    /// Get edge metadata
    pub fn get_edge(&self, edge_id: EdgeId) -> Option<&EdgeInfo> {
        self.edge_map.get(&edge_id)
    }

    /// Get number of edges
    pub fn edge_count(&self) -> usize {
        self.edge_map.len()
    }

    /// Get neighbors of a node
    /// Returns list of (neighbor_id, edge_id) tuples
    pub fn get_neighbors(&self, node_id: NodeId, direction: Direction) -> Vec<(NodeId, EdgeId)> {
        let mut neighbors = Vec::new();

        match direction {
            Direction::Outgoing => {
                if let Some(edges) = self.adjacency_out.get(&node_id) {
                    for &edge_id in edges {
                        if let Some(edge_info) = self.edge_map.get(&edge_id) {
                            neighbors.push((edge_info.to_id, edge_id));
                        }
                    }
                }
            }
            Direction::Incoming => {
                if let Some(edges) = self.adjacency_in.get(&node_id) {
                    for &edge_id in edges {
                        if let Some(edge_info) = self.edge_map.get(&edge_id) {
                            neighbors.push((edge_info.from_id, edge_id));
                        }
                    }
                }
            }
            Direction::Both => {
                // Outgoing
                if let Some(edges) = self.adjacency_out.get(&node_id) {
                    for &edge_id in edges {
                        if let Some(edge_info) = self.edge_map.get(&edge_id) {
                            neighbors.push((edge_info.to_id, edge_id));
                        }
                    }
                }
                // Incoming (for bidirectional edges)
                if let Some(edges) = self.adjacency_in.get(&node_id) {
                    for &edge_id in edges {
                        if let Some(edge_info) = self.edge_map.get(&edge_id) {
                            if edge_info.bidirectional {
                                neighbors.push((edge_info.from_id, edge_id));
                            }
                        }
                    }
                }
            }
        }

        neighbors
    }

    /// Get degree of a node (number of edges)
    pub fn get_degree(&self, node_id: NodeId, direction: Direction) -> usize {
        match direction {
            Direction::Outgoing => self.adjacency_out.get(&node_id).map_or(0, |e| e.len()),
            Direction::Incoming => self.adjacency_in.get(&node_id).map_or(0, |e| e.len()),
            Direction::Both => {
                let out = self.adjacency_out.get(&node_id).map_or(0, |e| e.len());
                let in_count = self.adjacency_in.get(&node_id).map_or(0, |edges| {
                    edges
                        .iter()
                        .filter(|&&e| {
                            self.edge_map
                                .get(&e)
                                .map_or(false, |info| info.bidirectional)
                        })
                        .count()
                });
                out + in_count
            }
        }
    }

    /// Get all node IDs
    pub fn get_nodes(&self) -> Vec<NodeId> {
        self.adjacency_out.keys().copied().collect()
    }

    /// Clear all nodes and edges
    pub fn clear(&mut self) {
        self.adjacency_out.clear();
        self.adjacency_in.clear();
        self.edge_map.clear();
    }

    // ==================== TRAVERSAL ALGORITHMS ====================

    /// Breadth-First Search (BFS) traversal
    /// Visits nodes level by level starting from start_id
    /// Calls visitor function for each visited node with (node_id, depth)
    ///
    /// # Arguments
    ///
    /// * `start_id` - Starting node
    /// * `max_depth` - Maximum depth to traverse (None = unlimited)
    /// * `visitor` - Function called for each visited node
    ///
    /// # Example
    ///
    /// ```
    /// graph.bfs(1, Some(3), |node_id, depth| {
    ///     println!("Visited node {} at depth {}", node_id, depth);
    /// });
    /// ```
    pub fn bfs<F>(&self, start_id: NodeId, max_depth: Option<usize>, mut visitor: F)
    where
        F: FnMut(NodeId, usize),
    {
        if !self.contains_node(start_id) {
            return;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((start_id, 0));
        visited.insert(start_id);

        while let Some((current_id, depth)) = queue.pop_front() {
            visitor(current_id, depth);

            // Check max depth
            if let Some(max_d) = max_depth {
                if depth >= max_d {
                    continue;
                }
            }

            // Visit neighbors
            let neighbors = self.get_neighbors(current_id, Direction::Both);
            for (neighbor_id, _edge_id) in neighbors {
                if !visited.contains(&neighbor_id) {
                    visited.insert(neighbor_id);
                    queue.push_back((neighbor_id, depth + 1));
                }
            }
        }
    }

    /// BFS iterator for lazy traversal
    /// Returns iterator over (node_id, depth) pairs
    pub fn bfs_iter(&self, start_id: NodeId, max_depth: Option<usize>) -> BFSIterator {
        BFSIterator::new(self, start_id, max_depth)
    }

    /// Depth-First Search (DFS) traversal
    /// Visits nodes depth-first starting from start_id
    /// Calls visitor function for each visited node with (node_id, depth)
    ///
    /// # Arguments
    ///
    /// * `start_id` - Starting node
    /// * `max_depth` - Maximum depth to traverse (None = unlimited)
    /// * `visitor` - Function called for each visited node
    ///
    /// # Example
    ///
    /// ```
    /// graph.dfs(1, Some(5), |node_id, depth| {
    ///     println!("Visited node {} at depth {}", node_id, depth);
    /// });
    /// ```
    pub fn dfs<F>(&self, start_id: NodeId, max_depth: Option<usize>, mut visitor: F)
    where
        F: FnMut(NodeId, usize),
    {
        if !self.contains_node(start_id) {
            return;
        }

        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        stack.push((start_id, 0));

        while let Some((current_id, depth)) = stack.pop() {
            if visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id);
            visitor(current_id, depth);

            // Check max depth
            if let Some(max_d) = max_depth {
                if depth >= max_d {
                    continue;
                }
            }

            // Visit neighbors (in reverse order to maintain left-to-right traversal)
            let neighbors = self.get_neighbors(current_id, Direction::Both);
            for (neighbor_id, _edge_id) in neighbors.into_iter().rev() {
                if !visited.contains(&neighbor_id) {
                    stack.push((neighbor_id, depth + 1));
                }
            }
        }
    }

    /// DFS iterator for lazy traversal
    /// Returns iterator over (node_id, depth) pairs
    pub fn dfs_iter(&self, start_id: NodeId, max_depth: Option<usize>) -> DFSIterator {
        DFSIterator::new(self, start_id, max_depth)
    }

    // ==================== PATHFINDING ====================

    /// Find shortest path between two nodes (unweighted BFS)
    /// Returns None if no path exists
    ///
    /// # Arguments
    ///
    /// * `from_id` - Starting node
    /// * `to_id` - Target node
    ///
    /// # Example
    ///
    /// ```
    /// if let Some(path) = graph.find_path(1, 5) {
    ///     println!("Path length: {}", path.length);
    ///     println!("Nodes: {:?}", path.nodes);
    /// }
    /// ```
    pub fn find_path(&self, from_id: NodeId, to_id: NodeId) -> Option<Path> {
        // Same node
        if from_id == to_id {
            return Some(Path {
                nodes: vec![from_id],
                edges: Vec::new(),
                total_cost: 0.0,
                length: 0,
            });
        }

        // Check nodes exist
        if !self.contains_node(from_id) || !self.contains_node(to_id) {
            return None;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut predecessors: HashMap<NodeId, (NodeId, EdgeId)> = HashMap::new();

        queue.push_back(from_id);
        visited.insert(from_id);

        // BFS
        while let Some(current_id) = queue.pop_front() {
            if current_id == to_id {
                // Found! Reconstruct path
                return Some(self.reconstruct_path(from_id, to_id, &predecessors));
            }

            let neighbors = self.get_neighbors(current_id, Direction::Both);
            for (neighbor_id, edge_id) in neighbors {
                if !visited.contains(&neighbor_id) {
                    visited.insert(neighbor_id);
                    predecessors.insert(neighbor_id, (current_id, edge_id));
                    queue.push_back(neighbor_id);
                }
            }
        }

        None // No path found
    }

    /// Find shortest weighted path using Dijkstra's algorithm
    /// Uses edge weights from EdgeInfo
    /// Returns None if no path exists
    ///
    /// # Arguments
    ///
    /// * `from_id` - Starting node
    /// * `to_id` - Target node
    ///
    /// # Example
    ///
    /// ```
    /// if let Some(path) = graph.dijkstra(1, 5) {
    ///     println!("Total cost: {}", path.total_cost);
    ///     println!("Path length: {}", path.length);
    /// }
    /// ```
    pub fn dijkstra(&self, from_id: NodeId, to_id: NodeId) -> Option<Path> {
        // Same node
        if from_id == to_id {
            return Some(Path {
                nodes: vec![from_id],
                edges: Vec::new(),
                total_cost: 0.0,
                length: 0,
            });
        }

        // Check nodes exist
        if !self.contains_node(from_id) || !self.contains_node(to_id) {
            return None;
        }

        let mut distances: HashMap<NodeId, f32> = HashMap::new();
        let mut predecessors: HashMap<NodeId, (NodeId, EdgeId)> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(from_id, 0.0);
        heap.push(DijkstraState {
            cost: 0.0,
            node: from_id,
        });

        while let Some(DijkstraState { cost, node }) = heap.pop() {
            // Found target
            if node == to_id {
                return Some(self.reconstruct_path(from_id, to_id, &predecessors));
            }

            // Skip if we already found better path
            if cost > *distances.get(&node).unwrap_or(&f32::INFINITY) {
                continue;
            }

            // Visit neighbors
            let neighbors = self.get_neighbors(node, Direction::Both);
            for (neighbor_id, edge_id) in neighbors {
                if let Some(edge_info) = self.edge_map.get(&edge_id) {
                    // Edge cost (inverse of weight, or 1.0 if weight is 0)
                    let edge_cost = if edge_info.weight > 0.0 {
                        1.0 / edge_info.weight
                    } else {
                        1.0
                    };

                    let new_cost = cost + edge_cost;
                    let current_best = *distances.get(&neighbor_id).unwrap_or(&f32::INFINITY);

                    if new_cost < current_best {
                        distances.insert(neighbor_id, new_cost);
                        predecessors.insert(neighbor_id, (node, edge_id));
                        heap.push(DijkstraState {
                            cost: new_cost,
                            node: neighbor_id,
                        });
                    }
                }
            }
        }

        None // No path found
    }

    /// Reconstruct path from predecessors map
    fn reconstruct_path(
        &self,
        from_id: NodeId,
        to_id: NodeId,
        predecessors: &HashMap<NodeId, (NodeId, EdgeId)>,
    ) -> Path {
        let mut path_nodes = Vec::new();
        let mut path_edges = Vec::new();
        let mut current = to_id;

        // Trace back from target to source
        while current != from_id {
            path_nodes.push(current);

            if let Some(&(prev_node, edge_id)) = predecessors.get(&current) {
                path_edges.push(edge_id);
                current = prev_node;
            } else {
                // Should not happen if path exists
                break;
            }
        }

        path_nodes.push(from_id);

        // Reverse to get from->to order
        path_nodes.reverse();
        path_edges.reverse();

        // Calculate total cost
        let total_cost = path_edges
            .iter()
            .filter_map(|&edge_id| self.edge_map.get(&edge_id))
            .map(|edge_info| {
                if edge_info.weight > 0.0 {
                    1.0 / edge_info.weight
                } else {
                    1.0
                }
            })
            .sum();

        Path {
            nodes: path_nodes.clone(),
            edges: path_edges.clone(),
            total_cost,
            length: path_edges.len(),
        }
    }

    // ==================== SUBGRAPHS ====================

    /// Extract induced subgraph from node set
    /// Subgraph contains all nodes and all edges between them
    ///
    /// # Arguments
    ///
    /// * `node_ids` - Set of node IDs to include
    ///
    /// # Example
    ///
    /// ```
    /// let mut nodes = HashSet::new();
    /// nodes.insert(1);
    /// nodes.insert(2);
    /// nodes.insert(3);
    ///
    /// let subgraph = graph.extract_subgraph(&nodes);
    /// println!("Subgraph has {} nodes and {} edges",
    ///          subgraph.node_count(), subgraph.edge_count());
    /// ```
    pub fn extract_subgraph(&self, node_ids: &HashSet<NodeId>) -> Subgraph {
        let mut subgraph = Subgraph::new();
        subgraph.nodes = node_ids.clone();

        // Find all edges between nodes in the set
        for &node_id in node_ids {
            if let Some(outgoing) = self.adjacency_out.get(&node_id) {
                for &edge_id in outgoing {
                    if let Some(edge_info) = self.edge_map.get(&edge_id) {
                        if node_ids.contains(&edge_info.to_id) {
                            subgraph.edges.insert(edge_id);
                        }
                    }
                }
            }
        }

        subgraph
    }

    /// Extract ego-network (neighborhood) around a node
    /// Returns subgraph containing center node and all nodes within radius hops
    ///
    /// # Arguments
    ///
    /// * `center_id` - Center node
    /// * `radius` - Maximum number of hops
    ///
    /// # Example
    ///
    /// ```
    /// // Extract 2-hop neighborhood around node 10
    /// let neighborhood = graph.extract_neighborhood(10, 2);
    /// ```
    pub fn extract_neighborhood(&self, center_id: NodeId, radius: usize) -> Subgraph {
        let mut nodes_within = HashSet::new();
        nodes_within.insert(center_id);

        // BFS to find all nodes within radius
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((center_id, 0));
        visited.insert(center_id);

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth < radius {
                let neighbors = self.get_neighbors(current_id, Direction::Both);
                for (neighbor_id, _edge_id) in neighbors {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id);
                        nodes_within.insert(neighbor_id);
                        queue.push_back((neighbor_id, depth + 1));
                    }
                }
            }
        }

        self.extract_subgraph(&nodes_within)
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== ITERATORS ====================

/// BFS Iterator
pub struct BFSIterator<'a> {
    graph: &'a Graph,
    visited: HashSet<NodeId>,
    queue: VecDeque<(NodeId, usize)>,
    max_depth: Option<usize>,
}

impl<'a> BFSIterator<'a> {
    fn new(graph: &'a Graph, start_id: NodeId, max_depth: Option<usize>) -> Self {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        if graph.contains_node(start_id) {
            queue.push_back((start_id, 0));
            visited.insert(start_id);
        }

        Self {
            graph,
            visited,
            queue,
            max_depth,
        }
    }
}

impl<'a> Iterator for BFSIterator<'a> {
    type Item = (NodeId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((current_id, depth)) = self.queue.pop_front() {
            // Check max depth before adding neighbors
            if self.max_depth.is_none() || depth < self.max_depth.unwrap() {
                let neighbors = self.graph.get_neighbors(current_id, Direction::Both);
                for (neighbor_id, _edge_id) in neighbors {
                    if !self.visited.contains(&neighbor_id) {
                        self.visited.insert(neighbor_id);
                        self.queue.push_back((neighbor_id, depth + 1));
                    }
                }
            }

            Some((current_id, depth))
        } else {
            None
        }
    }
}

/// DFS Iterator
pub struct DFSIterator<'a> {
    graph: &'a Graph,
    visited: HashSet<NodeId>,
    stack: Vec<(NodeId, usize)>,
    max_depth: Option<usize>,
}

impl<'a> DFSIterator<'a> {
    fn new(graph: &'a Graph, start_id: NodeId, max_depth: Option<usize>) -> Self {
        let mut stack = Vec::new();

        if graph.contains_node(start_id) {
            stack.push((start_id, 0));
        }

        Self {
            graph,
            visited: HashSet::new(),
            stack,
            max_depth,
        }
    }
}

impl<'a> Iterator for DFSIterator<'a> {
    type Item = (NodeId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current_id, depth)) = self.stack.pop() {
            if self.visited.contains(&current_id) {
                continue;
            }

            self.visited.insert(current_id);

            // Add neighbors to stack if not at max depth
            if self.max_depth.is_none() || depth < self.max_depth.unwrap() {
                let neighbors = self.graph.get_neighbors(current_id, Direction::Both);
                for (neighbor_id, _edge_id) in neighbors.into_iter().rev() {
                    if !self.visited.contains(&neighbor_id) {
                        self.stack.push((neighbor_id, depth + 1));
                    }
                }
            }

            return Some((current_id, depth));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_remove_node() {
        let mut graph = Graph::new();

        // Add node
        assert!(graph.add_node(1));
        assert_eq!(graph.node_count(), 1);
        assert!(graph.contains_node(1));

        // Add duplicate
        assert!(!graph.add_node(1));
        assert_eq!(graph.node_count(), 1);

        // Remove node
        assert!(graph.remove_node(1));
        assert_eq!(graph.node_count(), 0);
        assert!(!graph.contains_node(1));

        // Remove non-existent
        assert!(!graph.remove_node(1));
    }

    #[test]
    fn test_add_remove_edge() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);

        let edge_id = Graph::compute_edge_id(1, 2, 0);

        // Add edge
        assert!(graph.add_edge(edge_id, 1, 2, 0, 1.0, false).unwrap());
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.contains_edge(edge_id));

        // Add duplicate
        assert!(!graph.add_edge(edge_id, 1, 2, 0, 1.0, false).unwrap());
        assert_eq!(graph.edge_count(), 1);

        // Remove edge
        assert!(graph.remove_edge(edge_id));
        assert_eq!(graph.edge_count(), 0);
        assert!(!graph.contains_edge(edge_id));
    }

    #[test]
    fn test_get_neighbors() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        let edge1 = Graph::compute_edge_id(1, 2, 0);
        let edge2 = Graph::compute_edge_id(1, 3, 0);

        graph.add_edge(edge1, 1, 2, 0, 1.0, false).unwrap();
        graph.add_edge(edge2, 1, 3, 0, 1.0, false).unwrap();

        // Outgoing neighbors
        let neighbors = graph.get_neighbors(1, Direction::Outgoing);
        assert_eq!(neighbors.len(), 2);

        // Incoming neighbors
        let neighbors = graph.get_neighbors(2, Direction::Incoming);
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].0, 1);
    }

    #[test]
    fn test_get_degree() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);

        let edge_id = Graph::compute_edge_id(1, 2, 0);
        graph.add_edge(edge_id, 1, 2, 0, 1.0, false).unwrap();

        assert_eq!(graph.get_degree(1, Direction::Outgoing), 1);
        assert_eq!(graph.get_degree(1, Direction::Incoming), 0);
        assert_eq!(graph.get_degree(2, Direction::Outgoing), 0);
        assert_eq!(graph.get_degree(2, Direction::Incoming), 1);
    }

    #[test]
    fn test_path() {
        let path = Path::empty();
        assert!(path.is_empty());
        assert!(path.is_valid());

        let path = Path {
            nodes: vec![1, 2, 3],
            edges: vec![100, 101],
            total_cost: 2.0,
            length: 2,
        };
        assert!(!path.is_empty());
        assert!(path.is_valid());
        assert!(path.contains_node(2));
        assert!(path.contains_edge(100));
    }

    #[test]
    fn test_remove_node_removes_edges() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        let edge1 = Graph::compute_edge_id(1, 2, 0);
        let edge2 = Graph::compute_edge_id(2, 3, 0);

        graph.add_edge(edge1, 1, 2, 0, 1.0, false).unwrap();
        graph.add_edge(edge2, 2, 3, 0, 1.0, false).unwrap();

        assert_eq!(graph.edge_count(), 2);

        // Remove node 2 should remove both edges
        graph.remove_node(2);
        assert_eq!(graph.edge_count(), 0);
        assert!(!graph.contains_edge(edge1));
        assert!(!graph.contains_edge(edge2));
    }

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        // Create simple chain: 1 -> 2 -> 3 -> 4 -> 5
        let edge1 = Graph::compute_edge_id(1, 2, 0);
        let edge2 = Graph::compute_edge_id(2, 3, 0);
        let edge3 = Graph::compute_edge_id(3, 4, 0);
        let edge4 = Graph::compute_edge_id(4, 5, 0);

        graph.add_edge(edge1, 1, 2, 0, 1.0, false).unwrap();
        graph.add_edge(edge2, 2, 3, 0, 1.0, false).unwrap();
        graph.add_edge(edge3, 3, 4, 0, 1.0, false).unwrap();
        graph.add_edge(edge4, 4, 5, 0, 1.0, false).unwrap();

        // BFS from node 1
        let mut visited = Vec::new();
        graph.bfs(1, None, |node_id, depth| {
            visited.push((node_id, depth));
        });

        assert_eq!(visited.len(), 5);
        assert_eq!(visited[0], (1, 0)); // Start node
        assert_eq!(visited[1], (2, 1)); // Direct neighbor
    }

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        // Create tree: 1 -> 2, 1 -> 3, 2 -> 4, 2 -> 5
        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(1, 3, 0), 1, 3, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 4, 0), 2, 4, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 5, 0), 2, 5, 0, 1.0, false)
            .unwrap();

        // DFS from node 1
        let mut visited = Vec::new();
        graph.dfs(1, None, |node_id, _depth| {
            visited.push(node_id);
        });

        assert_eq!(visited.len(), 5);
        assert_eq!(visited[0], 1); // Start node
    }

    #[test]
    fn test_find_path() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        // Chain: 1 -> 2 -> 3 -> 4 -> 5
        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 3, 0), 2, 3, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(3, 4, 0), 3, 4, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(4, 5, 0), 4, 5, 0, 1.0, false)
            .unwrap();

        // Find path 1 -> 5
        let path = graph.find_path(1, 5).unwrap();
        assert_eq!(path.length, 4);
        assert_eq!(path.nodes, vec![1, 2, 3, 4, 5]);
        assert!(path.is_valid());

        // No path to isolated node
        graph.add_node(100);
        assert!(graph.find_path(1, 100).is_none());

        // Same node
        let path = graph.find_path(1, 1).unwrap();
        assert_eq!(path.length, 0);
        assert_eq!(path.nodes, vec![1]);
    }

    #[test]
    fn test_dijkstra() {
        let mut graph = Graph::new();
        for i in 1..=4 {
            graph.add_node(i);
        }

        // Diamond shape with different weights
        // 1 -> 2 (weight 1.0)
        // 1 -> 3 (weight 0.5)
        // 2 -> 4 (weight 1.0)
        // 3 -> 4 (weight 1.0)
        // Best path: 1 -> 3 -> 4 (cost = 1.0 + 1.0 = 2.0)
        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(1, 3, 0), 1, 3, 0, 2.0, false)
            .unwrap(); // Higher weight = lower cost
        graph
            .add_edge(Graph::compute_edge_id(2, 4, 0), 2, 4, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(3, 4, 0), 3, 4, 0, 1.0, false)
            .unwrap();

        let path = graph.dijkstra(1, 4).unwrap();
        assert!(path.is_valid());
        assert_eq!(path.nodes[0], 1);
        assert_eq!(path.nodes[path.nodes.len() - 1], 4);
    }

    #[test]
    fn test_extract_subgraph() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        // Chain: 1 -> 2 -> 3 -> 4 -> 5
        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 3, 0), 2, 3, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(3, 4, 0), 3, 4, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(4, 5, 0), 4, 5, 0, 1.0, false)
            .unwrap();

        // Extract subgraph with nodes 2, 3, 4
        let mut nodes = HashSet::new();
        nodes.insert(2);
        nodes.insert(3);
        nodes.insert(4);

        let subgraph = graph.extract_subgraph(&nodes);
        assert_eq!(subgraph.node_count(), 3);
        assert_eq!(subgraph.edge_count(), 2); // Edges 2->3 and 3->4
    }

    #[test]
    fn test_extract_neighborhood() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        // Chain: 1 -> 2 -> 3 -> 4 -> 5
        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, true)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 3, 0), 2, 3, 0, 1.0, true)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(3, 4, 0), 3, 4, 0, 1.0, true)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(4, 5, 0), 4, 5, 0, 1.0, true)
            .unwrap();

        // 2-hop neighborhood around node 3 (bidirectional edges)
        let neighborhood = graph.extract_neighborhood(3, 2);
        assert_eq!(neighborhood.node_count(), 5); // All nodes within 2 hops: 1,2,3,4,5
    }

    #[test]
    fn test_bfs_iterator() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 3, 0), 2, 3, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(3, 4, 0), 3, 4, 0, 1.0, false)
            .unwrap();

        let visited: Vec<_> = graph.bfs_iter(1, Some(2)).collect();
        assert!(visited.len() >= 1);
        assert_eq!(visited[0].0, 1); // First node is start
    }

    #[test]
    fn test_dfs_iterator() {
        let mut graph = Graph::new();
        for i in 1..=5 {
            graph.add_node(i);
        }

        graph
            .add_edge(Graph::compute_edge_id(1, 2, 0), 1, 2, 0, 1.0, false)
            .unwrap();
        graph
            .add_edge(Graph::compute_edge_id(2, 3, 0), 2, 3, 0, 1.0, false)
            .unwrap();

        let visited: Vec<_> = graph.dfs_iter(1, None).collect();
        assert!(visited.len() >= 1);
        assert_eq!(visited[0].0, 1); // First node is start
    }
}
