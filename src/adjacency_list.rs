use crate::Graph;
use crate::graph::{DfsEvent, UndirectedGraph};
use crate::graphs::AdjacencyMatrix;

/// Represents a graph using an adjacency list.
/// Each index in the inner vector corresponds to a node, and the vector at that index
/// contains the neighbors of that node.
#[derive(Debug, Clone, Default)]
pub struct AdjacencyList(pub Vec<Vec<usize>>);

impl AdjacencyList {
    /// Constructs an adjacency list from an adjacency matrix.
    ///
    /// # Arguments
    /// * `matrix` - Reference to the adjacency matrix.
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let mut adjacency_list = vec![Vec::new(); matrix.0.len()];

        for (i, row) in matrix.0.iter().enumerate() {
            adjacency_list[i].extend(
                row.iter()
                    .enumerate()
                    .filter_map(|(j, &val)| (val != 0).then_some(j)),
            );
        }
        AdjacencyList(adjacency_list)
    }
}

impl Graph<usize> for AdjacencyList {
    fn order(&self) -> usize {
        self.0.len()
    }

    fn size(&self) -> usize {
        self.0.iter().map(|neighbors| neighbors.len()).sum()
    }

    fn node_degrees(&self, n: usize) -> (usize, usize) {
        let out_deg = self.0.get(n).map_or(0, |neighbors| neighbors.len());
        let in_deg = self
            .0
            .iter()
            .filter(|neighbors| neighbors.contains(&n))
            .count();
        (in_deg, out_deg)
    }

    fn nodes(&self) -> impl Iterator<Item = usize> {
        0..self.order()
    }

    fn add_node(&mut self, _n: usize) {
        self.0.push(Vec::new());
    }

    fn remove_node(&mut self, n: usize) {
        if n < self.0.len() {
            self.0.remove(n);
            for neighbors in self.0.iter_mut() {
                neighbors.retain(|&x| x != n);
                for x in neighbors.iter_mut() {
                    if *x > n {
                        *x -= 1;
                    }
                }
            }
        }
    }

    fn add_edge(&mut self, n: usize, m: usize) {
        if self.0.get(m).is_some()
            && let Some(n_edges) = self.0.get_mut(n)
            && !n_edges.contains(&m)
        {
            n_edges.push(m);
        }
    }

    fn remove_edge(&mut self, n: usize, m: usize) {
        if let Some(edges) = self.0.get_mut(n) {
            edges.retain(|&x| x != m);
        }
    }

    type Neighbors<'a> = std::iter::Copied<std::slice::Iter<'a, usize>>;

    fn neighbors<'a>(&'a self, n: usize) -> Self::Neighbors<'a> {
        match self.0.get(n) {
            Some(edges) => edges.iter().copied(),
            None => [].iter().copied(),
        }
    }

    fn bipartite(&self) -> bool {
        let n = self.order();
        if n == 0 {
            return true;
        }

        let mut side = vec![None; n];

        for start in 0..n {
            if side[start].is_some() {
                continue;
            }
            side[start] = Some(0);
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(start);

            while let Some(u) = queue.pop_front() {
                let u_side = side[u].unwrap();
                for v in self.neighbors(u) {
                    if side[v].is_none() {
                        side[v] = Some(1 - u_side);
                        queue.push_back(v);
                    } else if side[v] == Some(u_side) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn underlying_graph(&self) -> Self {
        let mut list = AdjacencyList(vec![Vec::new(); self.0.len()]);

        for (idx_r, row) in self.0.iter().enumerate() {
            for &col in row.iter() {
                if !list.has_edge(idx_r, col) {
                    list.add_undirected_edge(idx_r, col);
                }
            }
        }
        list
    }
}

impl UndirectedGraph<usize> for AdjacencyList {
    fn undirected_size(&self) -> usize {
        let mut self_loops = 0;
        let regular_edges: usize = self
            .0
            .iter()
            .enumerate()
            .map(|(i, _)| {
                self.neighbors(i)
                    .filter(|&n| {
                        let is_self_loop = n == i;
                        self_loops += is_self_loop as usize;
                        !is_self_loop
                    })
                    .count()
            })
            .sum();
        regular_edges / 2 + self_loops
    }

    fn connected(&self) -> bool {
        for i in 0..self.order() {
            if self
                .dfs(i)
                .filter(|event| matches!(event, DfsEvent::Discover(_, _)))
                .count()
                != self.order()
            {
                return false;
            }
        }
        true
    }

    fn undirected_node_degree(&self, node: usize) -> usize {
        self.0
            .get(node)
            .map(|neighbors| neighbors.len())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn connected_undirected_graph() {
        // Graph: 2 ── 0 ── 1
        // should be connected.
        assert!(AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]).connected())
    }

    #[test]
    fn unconnected_undirected_graph() {
        // Graph: 2    0 ── 1
        // should be not connected.
        assert!(!AdjacencyList(vec![vec![1], vec![0], vec![]]).connected())
    }

    #[test]
    fn underlying_graph_conversion() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let original_list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);

        let underlying_list = original_list.underlying_graph();

        // Current graph:
        //     0        - 1
        //       \    /    \
        //        - 3       - 2
        //       /
        //      4
        assert_eq!(original_list.order(), underlying_list.order());
        assert_eq!(original_list.size(), underlying_list.undirected_size());
        assert!(underlying_list.connected());
    }

    #[test]
    fn underlying_graph_conversion_and_node_delete_after() {
        // Graph:
        // 0 -> 1 -> 2 <- 3
        //      \    ^
        //       \   |
        //       ->  4
        let original_list = AdjacencyList(vec![vec![1], vec![2, 4], vec![], vec![2], vec![2]]);

        let mut underlying_list = original_list.underlying_graph();

        // Current graph:
        // 0 -- 1 -- 2 -- 3
        //      \    |
        //       \   |
        //        -  4
        assert_eq!(original_list.order(), underlying_list.order());
        assert_eq!(original_list.size(), underlying_list.undirected_size());
        assert!(underlying_list.connected());

        underlying_list.remove_node(2);
        // Current graph:
        // 0 -- 1         2
        //      \
        //       \
        //        -  3
        assert_ne!(original_list.order(), underlying_list.order());
        assert!(!underlying_list.connected());
    }

    #[test]
    fn graph_add_new_node() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4    5
        assert!(list.order() == 6);
        assert!(list.size() == 4);
        assert!(!list.connected());
    }

    #[test]
    fn graph_add_new_node_and_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.order() == 6);
        assert!(list.size() == 5);
        assert!(list.has_edge(3, 5));
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_add_new_node_and_loop_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        list.add_node(5);
        list.add_edge(3, 5);
        list.add_edge(2, 2);

        // Current graph:
        //     0      -> 1      ---
        //       \    /   \    /   \
        //        -> 3     -> 2    |
        //       /    \        ^   /
        //      4      -> 5    \--
        assert!(list.order() == 6);
        assert!(list.size() == 6);
        assert!(list.has_edge(3, 5));
        assert!(list.has_edge(2, 2));
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_edge() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]]);

        list.remove_edge(0, 3);
        // Current graph:
        //     0       -> 1
        //           /   \
        //          3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.size() == 4);
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(4, 3));
        assert!(!list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_edge_loop() {
        // Graph:
        //     0      -> 1      ---
        //       \    /   \    /   \
        //        -> 3     -> 2    |
        //       /    \        ^   /
        //      4      -> 5    \--
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![2], vec![1, 5], vec![3], vec![]]);
        list.remove_edge(2, 2);

        // Current graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        assert!(list.size() == 5);
        assert!(!list.has_edge(2, 2));
        assert!(list.has_edge(1, 2));
        assert!(list.underlying_graph().connected());
    }

    #[test]
    fn graph_remove_node() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /    \
        //      4      -> 5
        let mut list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1, 5], vec![3], vec![]]);
        list.remove_node(3);

        // Current graph:
        //     0      1
        //              \
        //               -> 2
        //
        //      3       4
        assert!(list.order() == 5);
        assert!(list.size() == 1);
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(1, 2));
        assert!(!list.underlying_graph().connected());
    }

    #[test]
    fn undirected_graph_add_new_node() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 4);
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_add_new_node_and_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_undirected_edge(3, 5);

        // Current graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 5);
        assert!(list.has_edge(3, 5));
        assert!(list.connected());
    }

    #[test]
    fn undirected_graph_add_new_node_and_loop_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /
        //      4
        let mut list = AdjacencyList(vec![vec![3], vec![2, 3], vec![1], vec![0, 1, 4], vec![3]]);
        list.add_node(5);
        list.add_undirected_edge(3, 5);
        list.add_undirected_edge(2, 2);

        // Current graph:
        //     0       1      -
        //       \   /   \  /   \
        //         3      2      |
        //       /   \      \    /
        //      4     5       -
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 6);
        assert!(list.connected());
        assert!(list.has_edge(3, 5));
        assert!(list.has_edge(2, 2));
    }

    #[test]
    fn undirected_graph_remove_edge() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_undirected_edge(1, 3);

        // Current graph:
        //     0       1
        //       \       \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 4);
        assert!(!list.has_edge(3, 1));
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_remove_edge_loop() {
        // Graph:
        //     0       1      -
        //       \   /   \  /   \
        //         3      2      |
        //       /   \      \    /
        //      4     5       -
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1, 2],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_undirected_edge(2, 2);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //       /   \
        //      4     5
        assert!(list.order() == 6);
        assert!(list.undirected_size() == 5);
        assert!(!list.has_edge(2, 2));
        assert!(list.has_edge(2, 1));
        assert!(list.connected());
    }

    #[test]
    fn undirected_graph_remove_node_3() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_node(3);

        // Current graph:
        //     0       1
        //              \
        //               2
        //
        //       3    4
        assert!(list.order() == 5);
        assert!(list.undirected_size() == 1);
        assert!(!list.has_edge(0, 3));
        assert!(list.has_edge(2, 1));
        assert!(!list.connected());
    }

    #[test]
    fn undirected_graph_remove_node_4() {
        // Graph:
        //     0       1
        //       \   /   \
        //         3      2
        //       /   \
        //      4     5
        let mut list = AdjacencyList(vec![
            vec![3],
            vec![2, 3],
            vec![1],
            vec![0, 1, 4, 5],
            vec![3],
            vec![3],
        ]);
        list.remove_node(4);

        // Current graph:
        //     0       1
        //       \    /  \
        //         3      2
        //          \
        //           4
        assert!(list.order() == 5);
        assert!(list.undirected_size() == 4);
        assert!(list.has_edge(0, 3));
        assert!(list.connected());
    }

    #[test]
    fn node_degree_adjacency_list() {
        // Graph: 0 ── 1 ── 2
        let list = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

        assert_eq!(list.undirected_node_degree(0), 1);
        assert_eq!(list.undirected_node_degree(1), 2);
        assert_eq!(list.undirected_node_degree(2), 1);
    }

    #[test]
    fn adjacency_list_order() {
        // Graph: 0 ── 1
        //        │
        //        2
        let list = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);
        assert_eq!(list.order(), 3);
    }

    #[test]
    fn test_size_adjacency_list() {
        // Graph: 0 ── 1
        //        │
        //        2
        let list = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);

        assert_eq!(list.undirected_size(), 2);
    }

    #[test]
    fn test_node_degrees_list() {
        let mut list = AdjacencyList::default();
        list.add_node(0);
        list.add_node(1);
        list.add_node(2);
        list.add_edge(0, 1);
        list.add_edge(1, 2);
        list.add_edge(2, 0);

        let degrees_0 = list.node_degrees(0);
        let degrees_1 = list.node_degrees(1);
        let degrees_2 = list.node_degrees(2);

        assert_eq!(degrees_0, (1, 1)); // in: 2->0, out: 0->1
        assert_eq!(degrees_1, (1, 1)); // in: 0->1, out: 1->2
        assert_eq!(degrees_2, (1, 1)); // in: 1->2, out: 2->0
    }
}
