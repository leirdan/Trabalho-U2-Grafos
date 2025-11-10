//! A Rust crate for representing and manipulating graphs using various data structures.
mod adjacency_matrix;
mod eulerian_cycle;
mod graph;
mod minimum_spanning_tree;
mod shortest_path;
mod traversal;

pub use graph::BfsEvent;
pub use graph::DfsEvent;
pub use graph::Edge;
pub use graph::Graph;
pub use graph::Node;
pub use graph::UndirectedGraph;

pub mod graphs {
    pub use crate::adjacency_matrix::AdjacencyMatrix;
    pub use crate::shortest_path::DijkstraIter;
}
