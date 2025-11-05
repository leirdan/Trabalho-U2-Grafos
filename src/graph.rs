use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Defines a generic interface for a graph data structure.
///
/// The [`Graph`] trait represents a **directed graph**, where each node can have
/// zero or more outgoing edges to other nodes.
/// It defines the essential operations for graph construction, traversal, and analysis.
///
/// Types implementing this trait can represent any kind of graph structure, such as
/// adjacency lists, adjacency matrices, ...
///
/// # Type Parameters
/// - `Node`: The type used to represent graph nodes.
///   Must implement [`Eq`], [`Hash`], and [`Copy`] to ensure efficient lookups.
pub trait Graph<Node: Eq + Hash + Copy>: Default {
    /// Returns the number of nodes (vertices) in the graph.
    fn order(&self) -> usize;

    /// Returns the number of edges in the graph.
    ///
    /// This includes all directed edges, so in a directed graph,
    /// an edge `(u, v)` counts as a single edge.
    fn size(&self) -> usize;

    /// Returns the **in-degree** and **out-degree** of a given node.
    ///
    /// # Arguments
    /// * `n` — The node whose degrees are being queried.
    ///
    /// # Returns
    /// A tuple `(usize, usize)` where:
    /// - The first element is the number of **incoming** edges.
    /// - The second element is the number of **outgoing** edges.
    fn node_degrees(&self, n: Node) -> (usize, usize);

    /// Returns an iterator over all nodes in the graph.
    ///
    /// The iterator yields each node exactly once.
    fn nodes(&self) -> impl Iterator<Item = Node>;

    /// Adds a new node to the graph.
    ///
    /// If the node already exists, this operation has no effect.
    fn add_node(&mut self, n: Node);

    /// Removes a node and all edges connected to it.
    ///
    /// If the node does not exist, this operation has no effect.
    fn remove_node(&mut self, n: Node);

    /// Adds a **directed edge** from node `n` to node `m`.
    ///
    /// If either node does not exist, this operation has no effect.
    fn add_edge(&mut self, n: Node, m: Node);

    /// Removes a **directed edge** from node `n` to node `m`, if it exists.
    ///
    /// If either node does not exist, this operation has no effect.
    fn remove_edge(&mut self, n: Node, m: Node);

    /// The associated iterator type for the neighbors of a given node.
    type Neighbors<'a>: Iterator<Item = Node>
    where
        Self: 'a,
        Node: 'a;

    /// Returns an iterator over all **neighbors** (adjacent nodes) of a given node.
    ///
    /// # Arguments
    /// * `n` — The node whose outgoing neighbors are to be listed.
    fn neighbors<'a>(&'a self, n: Node) -> Self::Neighbors<'a>;

    /// Checks whether the graph is **bipartite** and returns `true` or `false`
    fn bipartite(&self) -> bool;

    /// Returns the **underlying graph** of the current structure.
    ///
    /// This removes edge directionality, making each edge bidirectional.
    fn underlying_graph(&self) -> Self;

    /// Returns `true` if there is a directed edge from node `n` to node `m`.
    fn has_edge(&self, n: Node, m: Node) -> bool {
        self.neighbors(n).any(|neighbor| neighbor == m)
    }

    /// Returns an iterator that performs a **depth-first search (DFS)** starting from `start`.
    ///
    /// The iterator yields [`DfsEvent`] values that represent the traversal steps.
    fn dfs(&self, start: Node) -> DfsIter<'_, Node, Self>
    where
        Self: Sized,
    {
        DfsIter::new(self, start)
    }

    /// Returns an iterator that performs a **breadth-first search (BFS)** starting from `start`.
    ///
    /// The iterator yields [`BfsEvent`] values for each level of the search.
    fn bfs(&self, start: Node) -> BfsIter<'_, Node, Self>
    where
        Self: Sized,
    {
        BfsIter::new(self, start)
    }

    /// Returns an iterator that classifies all edges encountered during a DFS traversal.
    ///
    /// The classification follows standard DFS rules, producing edges of type ['Edge']
    fn classify_edges(&self, start: Node) -> DfsEdgesIter<'_, Node, Self>
    where
        Self: Sized,
    {
        DfsEdgesIter::new(self, start)
    }
}

/// Trait defining operations for **undirected graphs**.
///
/// Extends [`Graph`], treating each edge as a bidirectional connection `(n <-> m)`.
/// Provides utility methods for manipulation and analysis of undirected graphs,
/// including connectivity checks, biconnected components, and edge classification.
pub trait UndirectedGraph<Node: Copy + Eq + Hash>: Graph<Node> {
    /// Returns the total number of **undirected edges** in the graph.
    fn undirected_size(&self) -> usize;

    /// Checks whether the graph is **connected**.
    ///
    /// Returns `true` if there exists a path between every pair of nodes.
    fn connected(&self) -> bool;

    /// Returns an iterator over the **biconnected components** of the graph.
    ///
    /// The traversal starts from the given `start` node.
    fn biconnected_components(&self, start: Node) -> BiconnectedComponentsIter<'_, Node, Self>
    where
        Self: Sized,
    {
        BiconnectedComponentsIter::new(self, start)
    }

    /// Adds an **undirected edge** `(n <-> m)` to the graph.
    ///
    /// Internally, this adds both directed edges `(n -> m)` and `(m -> n)`.
    fn add_undirected_edge(&mut self, n: Node, m: Node) {
        self.add_edge(n, m);
        self.add_edge(m, n);
    }

    /// Removes an **undirected edge** `(n <-> m)` from the graph.
    ///
    /// Internally, this removes both directed edges `(n <-> m)` and `(m <-> n)`.
    fn remove_undirected_edge(&mut self, n: Node, m: Node) {
        self.remove_edge(n, m);
        self.remove_edge(m, n);
    }

    /// Returns the **degree** of the given node,
    /// considering all undirected connections.
    fn undirected_node_degree(&self, n: Node) -> usize {
        self.neighbors(n).count()
    }

    /// Returns an iterator classifying the **undirected edges** of the graph.
    ///
    /// Only edges of types [`Edge::Tree`] and [`Edge::Back`] are considered,
    /// as these represent meaningful relations in undirected graphs.
    fn classify_undirected_edges<'a>(&'a self, start: Node) -> impl Iterator<Item = Edge<Node>>
    where
        Self: Sized,
        Node: 'a,
    {
        DfsEdgesIter::new(self, start)
            .filter(|edge| matches!(edge, Edge::Tree(_, _) | Edge::Back(_, _)))
    }
}

/// Represents an event that occurs during a depth-first search (DFS) traversal.
///
/// This enum is used to describe the different types of events that can be
/// encountered while performing DFS on a graph. It is generic over the `Node`
/// type, which represents the nodes in the graph.
///
/// # Variants
/// - `Discover(Node, Option<Node>)`: Indicates that a node has been discovered for the first time.
///   The `Option<Node>` represents the parent node in the DFS tree (`None` for the start node).
/// - `Finish(Node)`: Indicates that all descendants of a node have been visited and the node is finished.
/// - `NonTreeEdge(Node, Node)`: Indicates that a non-tree edge (back, forward, or cross edge) is found
///   from the first node to the second node.
#[derive(Debug)]
pub enum DfsEvent<Node> {
    Discover(Node, Option<Node>),
    Finish(Node),
    NonTreeEdge(Node, Node),
}

/// Represents a iterator over a depth-first-search (DFS) traversal.
///
/// The iteration yields a `DfsEvent<Node>` over each instance of `next`.
pub struct DfsIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy,
    Self: 'a,
{
    graph: &'a G,
    stack: Vec<(Node, G::Neighbors<'a>)>,
    visited: HashSet<Node>,
    start_node: Option<Node>,
}

impl<'a, Node, G> DfsIter<'a, Node, G>
where
    Node: Eq + Hash + Copy,
    G: Graph<Node>,
{
    /// Creates a new DFS iterator starting from the given node.
    fn new(graph: &'a G, start: Node) -> Self {
        Self {
            graph,
            stack: vec![],
            visited: HashSet::with_capacity(graph.order()),
            start_node: Some(start),
        }
    }

    /// Sets the `start_node` field of a `DfsIter` manually.
    ///
    /// This enables starting another DFS while maintains the inner parts of the iterator
    /// initialized, like the `visited` dictionary.
    pub fn new_start(&mut self, start: Node) {
        self.start_node = Some(start)
    }
}

impl<'a, Node, G> Iterator for DfsIter<'a, Node, G>
where
    Node: Eq + Hash + Copy,
    G: Graph<Node>,
{
    type Item = DfsEvent<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(start_node) = self.start_node.take()
            && self.visited.insert(start_node)
        {
            self.stack
                .push((start_node, self.graph.neighbors(start_node)));
            return Some(DfsEvent::Discover(start_node, None));
        }

        if let Some((node, mut neighbors)) = self.stack.pop() {
            if let Some(neighbor) = neighbors.next() {
                self.stack.push((node, neighbors));

                if self.visited.insert(neighbor) {
                    self.stack.push((neighbor, self.graph.neighbors(neighbor)));
                    return Some(DfsEvent::Discover(neighbor, Some(node)));
                } else {
                    return Some(DfsEvent::NonTreeEdge(node, neighbor));
                }
            } else {
                return Some(DfsEvent::Finish(node));
            }
        }

        None
    }
}

/// Represents an event during a breadth-first search (BFS).
///
/// This enum is used to describe the different types of events that can be
/// encountered while performing DFS on a graph. It is generic over the `Node`
/// type, which represents the nodes in the graph.
///
/// # Variants
/// - `Discover(Node, Vec<Node>)`: Indicates that a node has been discovered for the first time.
///   The `Vec<Node>` represents the node's neighbors that will be explored on BFS tree.
/// - `CrossEdge(Node, Node)`: Indicates that a node has an edge to another and neither is an ancestor of the other.
#[derive(Debug)]
pub enum BfsEvent<Node> {
    Discover(Node, Vec<Node>),
    CrossEdge(Node, Node),
}

/// Represents a iterator over a breadth-first-search (BFS) traversal.
///
/// The iteration yields a `BfsEvent<Node>` over each instance of `next`.
pub struct BfsIter<'a, Node, G> {
    graph: &'a G,
    queue: VecDeque<Node>,
    visited: HashSet<Node>,
    parent: HashMap<Node, Option<Node>>,
}

impl<'a, Node, G> BfsIter<'a, Node, G>
where
    Node: Eq + Hash + Copy,
    G: Graph<Node>,
{
    /// Creates a new BFS iterator starting from the given node.
    fn new(graph: &'a G, start: Node) -> Self {
        let mut visited = HashSet::with_capacity(graph.order());
        visited.insert(start);

        let mut parent: HashMap<Node, Option<Node>> = HashMap::with_capacity(graph.order());
        parent.insert(start, None);

        Self {
            graph,
            queue: VecDeque::from(vec![start]),
            visited,
            parent,
        }
    }
}

impl<'a, Node, G> Iterator for BfsIter<'a, Node, G>
where
    Node: Eq + Hash + Copy + Display,
    G: Graph<Node>,
{
    type Item = Vec<BfsEvent<Node>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.queue.pop_front()?;
        let mut children: Vec<Node> = Vec::new();
        let mut events: Vec<BfsEvent<Node>> = Vec::new();

        for neighbor in self.graph.neighbors(node) {
            if self.visited.insert(neighbor) {
                self.queue.push_back(neighbor);
                self.parent.insert(neighbor, Some(node));
                children.push(neighbor);
            } else if Some(node) != self.parent.get(&neighbor).copied().flatten() {
                events.push(BfsEvent::CrossEdge(node, neighbor));
            }
        }

        events.push(BfsEvent::Discover(node, children));
        Some(events)
    }
}

/// Represents the classification of an edge in a graph during a depth-first search (DFS).
///
/// This enum is used to categorize edges based on the DFS traversal. It is generic
/// over the `Node` type, which represents the nodes in the graph. The classification
/// is based on the relationship between the two nodes connected by the edge in the DFS tree.
///
/// # Variants
/// - `Tree(u, v)`: An edge from a parent `u` to a child `v` in the DFS tree.
/// - `Back(u, v)`: An edge from a node `u` to its ancestor `v` in the DFS tree. This indicates a cycle.
/// - `ParentBack(u, v)`: A special case of a back edge where `v` is the direct parent of `u`.
///   This is common in undirected graphs.
/// - `Forward(u, v)`: An edge from a node `u` to its descendant `v` that is not a tree edge.
/// - `Cross(u, v)`: An edge between two nodes `u` and `v` such that neither is an ancestor of the other.
#[derive(Debug)]
pub enum Edge<Node> {
    Tree(Node, Node),
    Back(Node, Node),
    ParentBack(Node, Node),
    Forward(Node, Node),
    Cross(Node, Node),
}

/// An iterator that performs a depth-first search (DFS) and classifies the edges of the graph.
///
/// This iterator wraps a `DfsIter` and uses its events to classify each edge of the
/// graph into one of the categories defined by the `Edge` enum. It yields an `Edge<Node>`
/// for each edge encountered during the traversal.
pub struct DfsEdgesIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy,
    Self: 'a,
{
    iter: DfsIter<'a, Node, G>,
    time: usize,
    discovery: HashMap<Node, usize>,
    finish: HashMap<Node, usize>,
    parent: HashMap<Node, Node>,
    stack_hash: HashSet<Node>,
}

impl<'a, Node, G> DfsEdgesIter<'a, Node, G>
where
    Node: Eq + Hash + Copy,
    G: Graph<Node>,
{
    /// Creates a new DFS-with-edges iterator starting from the given node.
    fn new(graph: &'a G, start: Node) -> Self {
        Self {
            iter: DfsIter::new(graph, start),
            time: 0,
            discovery: HashMap::with_capacity(graph.order()),
            finish: HashMap::with_capacity(graph.order()),
            parent: HashMap::with_capacity(graph.order()),
            stack_hash: HashSet::with_capacity(graph.order()),
        }
    }

    /// Sets the `start_node` field of the inner `DfsIter` manually.
    ///
    /// This enables classifying edges from another components of a graph.
    pub fn new_start(&mut self, start: Node) {
        self.iter.new_start(start);
    }
}

impl<'a, Node, G> Iterator for DfsEdgesIter<'a, Node, G>
where
    Node: Eq + Hash + Copy,
    G: Graph<Node>,
{
    type Item = Edge<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.iter.next() {
            match event {
                DfsEvent::Discover(node, maybe_parent) => {
                    self.stack_hash.insert(node);
                    self.discovery.insert(node, self.time);
                    self.time += 1;
                    if let Some(parent) = maybe_parent {
                        self.parent.insert(node, parent);
                        return Some(Edge::Tree(parent, node));
                    }
                }
                DfsEvent::Finish(node) => {
                    self.stack_hash.remove(&node);
                    self.finish.insert(node, self.time);
                    self.time += 1;
                }
                DfsEvent::NonTreeEdge(node, neighbor) => {
                    if self.stack_hash.contains(&neighbor) {
                        if self
                            .parent
                            .get(&node)
                            .is_some_and(|&parent| parent == neighbor)
                        {
                            return Some(Edge::ParentBack(node, neighbor));
                        } else {
                            return Some(Edge::Back(node, neighbor));
                        }
                    } else if self
                        .discovery
                        .get(&node)
                        .is_some_and(|t1| self.discovery.get(&neighbor).is_some_and(|t2| t1 < t2))
                    {
                        return Some(Edge::Forward(node, neighbor));
                    } else {
                        return Some(Edge::Cross(node, neighbor));
                    }
                }
            }
        }
        None
    }
}

/// An iterator that yields the biconnected components of a undirected graph (`UndirectedGraph`).
///
/// The iterator identifies the biconnected components during a depth-first-search (DFS) that's
/// made by a inner iterator, a `DfsIter`.
pub struct BiconnectedComponentsIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy,
    Self: 'a,
{
    iter: DfsIter<'a, Node, G>,
    time: usize,
    discovery: HashMap<Node, usize>,
    lowpt: HashMap<Node, usize>,
    parents: HashMap<Node, Node>,
    edge_stack: Vec<(Node, Node)>,
}

impl<'a, Node, G> BiconnectedComponentsIter<'a, Node, G>
where
    G: Graph<Node> + 'a,
    Node: Eq + Hash + Copy + 'a,
{
    /// Creates a new iterator over the biconnected components of an undirected graph
    fn new(graph: &'a G, start: Node) -> Self {
        Self {
            iter: graph.dfs(start),
            time: 0,
            discovery: HashMap::with_capacity(graph.order()),
            lowpt: HashMap::with_capacity(graph.order()),
            parents: HashMap::with_capacity(graph.order()),
            edge_stack: Vec::with_capacity(graph.order()),
        }
    }

    /// Extracts a biconnected component from the edge stack.
    fn extract_component(&mut self, u: Node, v: Node) -> Option<Vec<(Node, Node)>> {
        let mut component = Vec::new();
        while let Some(edge) = self.edge_stack.pop() {
            component.push(edge);
            if edge == (u, v) || edge == (v, u) {
                break;
            }
        }
        Some(component)
    }
}

impl<'a, Node, G> Iterator for BiconnectedComponentsIter<'a, Node, G>
where
    G: Graph<Node>,
    Node: Eq + Hash + Copy + Display,
{
    type Item = Vec<(Node, Node)>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(event) = self.iter.next() {
            match event {
                DfsEvent::Discover(node, maybe_parent) => {
                    self.discovery.insert(node, self.time);
                    self.lowpt.insert(node, self.time);
                    self.time += 1;
                    if let Some(parent) = maybe_parent {
                        self.edge_stack.push((parent, node));
                        self.parents.insert(node, parent);
                    }
                }
                DfsEvent::Finish(node) => {
                    if let Some(&parent) = self.parents.get(&node) {
                        let &node_low = self.lowpt.get(&node).unwrap();
                        let parent_low = self.lowpt.get_mut(&parent).unwrap();

                        *parent_low = (*parent_low).min(node_low);

                        if self.discovery[&parent] <= self.lowpt[&node] {
                            return self.extract_component(parent, node);
                        }
                    } else if !self.edge_stack.is_empty() {
                        return Some(std::mem::take(&mut self.edge_stack));
                    }
                }
                DfsEvent::NonTreeEdge(u, v) => {
                    if Some(&v) != self.parents.get(&u) && self.discovery[&v] < self.discovery[&u] {
                        self.edge_stack.push((u, v));
                        self.lowpt
                            .entry(u)
                            .and_modify(|u_low| *u_low = (*u_low).min(self.discovery[&v]));
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{DfsEvent, Graph, UndirectedGraph, graphs::AdjacencyList};

    #[test]
    fn dfs_with_cycle() {
        let mut g = AdjacencyList::default();
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_edge(0, 1);
        g.add_edge(1, 2);
        g.add_edge(2, 0); // Cycle

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(1, Some(0)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(2, Some(1)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::NonTreeEdge(2, 0))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(2))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(1))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }

    #[test]
    fn dfs_simple_path() {
        let mut g = AdjacencyList::default();
        g.add_node(0);
        g.add_node(1);
        g.add_node(2);
        g.add_edge(0, 1);
        g.add_edge(1, 2);

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(1, Some(0)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(2, Some(1)))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(2))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(1))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }

    #[test]
    fn single_node_dfs() {
        let mut g = AdjacencyList::default();
        g.add_node(0);

        let mut dfs = g.dfs(0);

        assert!(matches!(dfs.next(), Some(DfsEvent::Discover(0, None))));
        assert!(matches!(dfs.next(), Some(DfsEvent::Finish(0))));
        assert!(dfs.next().is_none());
    }

    #[test]
    fn test_biconnected_components() {
        // 0 -- 1 -- 4
        //    /  \
        //   3 -- 2
        let mut graph = AdjacencyList::default();
        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);
        graph.add_undirected_edge(1, 4);
        graph.add_undirected_edge(0, 1);
        graph.add_undirected_edge(1, 2);
        graph.add_undirected_edge(1, 3);
        graph.add_undirected_edge(2, 3);

        let components: Vec<Vec<(usize, usize)>> = graph.biconnected_components(0).collect();

        assert_eq!(components.len(), 3);
        assert!(components.contains(&vec![(1, 4)]));
        assert!(components.contains(&vec![(3, 1), (2, 3), (1, 2)]));
        assert!(components.contains(&vec![(0, 1)]));
    }
}
