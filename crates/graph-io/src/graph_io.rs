use graphs_algorithms::BfsEvent;
use graphs_algorithms::{DfsEvent, Edge, Graph, UndirectedGraph};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::path::PathBuf;

/// Provides input/output capabilities for a directed graph.
///
/// This trait extends `Graph` with methods for importing a graph from a file
/// and exporting it to DOT format. It also supports exporting traversal results
/// (BFS and DFS) to DOT files for visualization and analysis.
/// # Type Parameters
/// - `Node`: The trait is generic over the node type `Node`,
///   which must implement basic traits like `Copy`,
///   `Eq`, `Hash`, `Display`, and `From<usize>` (the `From<usize>` requirement will be removed
///   in the future when non-integer node types are supported).
pub trait GraphIO<Node: Copy + Eq + Hash + Display + From<usize>>: Graph<Node> {
    /// Imports a graph from a file. The file should define the number of nodes
    /// on the first line and edges on subsequent lines.
    ///
    /// # Arguments
    /// * `path` - The path to the input file containing the graph.
    fn import_from_file(path: &str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let mut graph = Self::default();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            let content = line.unwrap();
            if content.is_empty() {
                continue;
            }

            if idx == 0 {
                let n: usize = content.parse().unwrap();
                for i in 0..n {
                    graph.add_node(Node::from(i));
                }
            } else {
                let values: Vec<usize> = content
                    .split(',')
                    .map(|x| {
                        x.trim().parse::<usize>().map(|v| v - 1).map_err(|_| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("Invalid data was found during file creation: {} ", x),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                graph.add_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        Ok(graph)
    }

    /// Exports the graph to a DOT file for visualization.
    ///
    /// # Arguments
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_to_dot(&self, path: &str) -> Result<(), Error> {
        let new_path = get_path(path);

        let mut file: File = File::create(new_path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        let mut count = 1;
        for node in self.nodes() {
            writeln!(file, " {} ", count)?;
            let this_id = count;
            count += 1;

            for neighbor in self.neighbors(node) {
                writeln!(file, " {} -> {} ", this_id, neighbor)?;
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    /// Exports a directed BFS traversal starting from `start` to a DOT file.
    /// # Arguments
    /// * `start` - The starting node for the BFS traversal.
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_directed_bfs_to_dot(&self, start: Node, path: &str) -> Result<(), Error>
    where
        Self: Sized,
    {
        let new_path = get_path(path);

        let iter = self.bfs(start);
        let mut file: File = File::create(&new_path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        for events in iter {
            for event in events {
                match event {
                    BfsEvent::Discover(node, items) => {
                        writeln!(file, " {} ", node)?;
                        for i in items {
                            writeln!(file, " {} -> {} ", node, i)?;
                        }
                    }
                    BfsEvent::CrossEdge(node_1, node_2) => {
                        writeln!(file, " {} -> {} [style=dashed]; ", node_1, node_2)?
                    }
                }
            }
        }

        writeln!(file, "}}")?;
        Ok(())
    }

    /// Exports a directed DFS traversal with edge classification to a DOT file.
    ///
    /// # Arguments
    /// * `start` - The starting node for the DFS traversal.
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_directed_dfs_to_dot(&self, start: Node, path: &str) -> Result<(), Error>
    where
        Self: Sized,
        Node: Copy + Eq + Hash + Display + From<usize>,
    {
        let new_path = get_path(path);

        let iter = self.classify_edges(start);
        let mut file: File = File::create(&new_path)?;

        writeln!(file, "digraph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        for event in iter {
            match event {
                Edge::Tree(parent, node) => {
                    writeln!(file, " {} ", node)?;
                    writeln!(file, " {} -> {} ", parent, node)?;
                }
                Edge::Back(node, parent) => writeln!(
                    file,
                    " {} -> {} [color=green, style=dashed]; ",
                    node, parent
                )?,
                Edge::ParentBack(_, _) => continue,
                Edge::Forward(parent, node) => {
                    writeln!(file, " {} -> {} [color=pink, style=dashed]; ", parent, node)?
                }
                Edge::Cross(node_1, node_2) => writeln!(
                    file,
                    " {} -> {} [color=purple, style=dashed]; ",
                    node_1, node_2
                )?,
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }
}

/// Provides input/output capabilities for an undirected graph.
///
/// This trait extends `GraphIO` with additional methods specific to undirected graphs,
/// including importing/exporting undirected edges and traversals (BFS/DFS) to DOT files.
/// It requires that the implementor also implements `UndirectedGraph`, ensuring access
/// to undirected-specific operations.
/// # Type Parameters
/// - `Node`: The trait is generic over the node type `Node`,
///   which must implement basic traits like `Copy`,
///   `Eq`, `Hash`, `Display`, and `From<usize>` (the `From<usize>` requirement will be removed
///   in the future when non-integer node types are supported).
pub trait UndirectedGraphIO<Node: Copy + Eq + Hash + Display + From<usize>>: GraphIO<Node> {
    /// Imports an undirected graph from a file. The file should define the number
    /// of nodes on the first line and edges on subsequent lines.
    ///
    /// # Arguments
    /// * `path` - The path to the input file containing the undirected graph.
    fn import_undirected_from_file(path: &str) -> Result<Self, Error>
    where
        Self: Sized + UndirectedGraph<Node>,
    {
        let mut graph = Self::default();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for (idx, line) in reader.lines().enumerate() {
            let content = line.unwrap();
            if content.is_empty() {
                continue;
            }

            if idx == 0 {
                let n: usize = content.parse().map_err(|_| {
                    Error::new(ErrorKind::InvalidData, "Line 1 from file is invalid")
                })?;

                for i in 0..n {
                    graph.add_node(Node::from(i));
                }
            } else {
                let values: Vec<usize> = content
                    .split(',')
                    .map(|x| {
                        x.trim().parse::<usize>().map(|v| v - 1).map_err(|_| {
                            Error::new(
                                ErrorKind::InvalidData,
                                format!("Invalid node was found during file creation: {} ", x),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, Error>>()?;

                graph.add_undirected_edge(Node::from(values[0]), Node::from(values[1]));
            }
        }

        Ok(graph)
    }

    /// Exports an undirected graph to a DOT file.
    ///
    /// # Arguments
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_undirected_to_dot(&self, path: &str) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
    {
        let new_path = get_path(path);

        let mut file: File = File::create(&new_path)?;

        let mut visited: Vec<Node> = vec![];

        writeln!(file, "graph G {{")?;
        writeln!(file, "  rankdir=LR;")?;
        writeln!(file, "  node [shape=circle];")?;

        for node in self.nodes() {
            writeln!(file, " {} ", node)?;
            for neighbor in self.neighbors(node) {
                if !visited.contains(&neighbor) {
                    writeln!(file, " {} -- {} ", node, neighbor)?;
                }
            }
            visited.push(node);
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    /// Exports an undirected DFS traversal to a DOT file.
    ///
    /// # Arguments
    /// * `start` - The starting node for the DFS traversal.
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_undirected_dfs_to_dot(&self, start: Node, path: &str) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
        Node: Copy + Eq + Hash + Display + From<usize> + PartialOrd,
    {
        if Node::from(self.nodes().count()) < start {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Node {} isn't present in graph!", start),
            ));
        }

        let new_path = get_path(path);
        let iter = self.dfs(start);
        let mut file: File = File::create(&new_path)?;
        let mut visited_edges: HashSet<(Node, Node)> = HashSet::new();

        writeln!(file, "graph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        for event in iter {
            match event {
                DfsEvent::Discover(node, option) => {
                    writeln!(file, " {} ", node)?;
                    if let Some(parent) = option
                        && !visited_edges.contains(&(node, parent))
                        && !visited_edges.contains(&(parent, node))
                    {
                        writeln!(file, " {} -- {} ", parent, node)?;
                        visited_edges.insert((node, parent));
                    }
                }
                DfsEvent::NonTreeEdge(node, parent) => {
                    if !visited_edges.contains(&(node, parent))
                        && !visited_edges.contains(&(parent, node))
                    {
                        writeln!(file, " {} -- {} [style=dashed];", node, parent)?;
                        visited_edges.insert((node, parent));
                    }
                }
                DfsEvent::Finish(_) => continue,
            }
        }

        writeln!(file, " }}")?;

        Ok(())
    }

    /// Exports an undirected BFS traversal to a DOT file.
    ///
    /// # Arguments
    /// * `start` - The starting node for the BFS traversal.
    /// * `path` - The path to the output DOT file. If it's inside a folder, this folder must exists earlier.
    fn export_undirected_bfs_to_dot(&self, start: Node, path: &str) -> Result<(), Error>
    where
        Self: Sized + UndirectedGraph<Node>,
        Node: Copy + Eq + Hash + Display + From<usize> + PartialOrd,
    {
        if Node::from(self.nodes().count()) < start {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Node {} isn't present in graph!", start),
            ));
        }

        let new_path = get_path(path);

        let iter = self.bfs(start);
        let mut file: File = File::create(&new_path)?;
        let mut visited_edges: HashSet<(Node, Node)> = HashSet::new();

        writeln!(file, "graph G {{")?;
        writeln!(file, "  node [shape=circle];")?;

        for events in iter {
            for event in events {
                match event {
                    BfsEvent::Discover(node, items) => {
                        writeln!(file, " {} ", node)?;
                        for i in items {
                            if visited_edges.insert((node, i)) {
                                writeln!(file, " {} -- {} ", node, i)?;
                            }
                        }
                    }
                    BfsEvent::CrossEdge(node_1, node_2) => {
                        if !visited_edges.contains(&(node_1, node_2))
                            && !visited_edges.contains(&(node_2, node_1))
                        {
                            writeln!(file, " {} -- {} [style=dashed]; ", node_1, node_2)?;
                            visited_edges.insert((node_1, node_2));
                        }
                    }
                }
            }
        }

        writeln!(file, "}}")?;
        Ok(())
    }
}

impl<N: Copy + Eq + Hash + Display + From<usize>, G: Graph<N>> GraphIO<N> for G {}

impl<N: Copy + Eq + Hash + Display + From<usize>, G: GraphIO<N>> UndirectedGraphIO<N> for G {}

// fn get_path(path: &str) -> &str {
//     if !path.contains(".dot") {
//         path.to_string().push_str(".dot");
//         return path;
//     }
//     path
// }

fn get_path(path: &str) -> PathBuf {
    let mut buf = PathBuf::from(path);
    if buf.extension().is_none() || buf.extension().unwrap() != "dot" {
        buf.set_extension("dot");
    }
    buf
}

#[cfg(test)]
mod test {
    use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};

    use super::*;

    static PATH: &str = "../../examples/data/";

    #[test]
    fn adj_list_new_digraph_1() {
        let path = format!("{PATH}DIGRAFO01.txt");
        let result: std::io::Result<AdjacencyList> = GraphIO::import_from_file(&path);

        assert!(result.is_ok());

        if let Ok(matrix) = result {
            assert!(matrix.order() == 13);
            assert!(matrix.size() == 16);
        }
    }

    #[test]
    fn adj_list_new_digraph_2() {
        let path = format!("{PATH}DIGRAFO02.txt");
        let result: Result<AdjacencyList, Error> = GraphIO::import_from_file(&path);

        assert!(result.is_ok());

        if let Ok(matrix) = result {
            assert!(matrix.order() == 13);
            assert!(matrix.size() == 17);
        }
    }

    #[test]
    fn adj_list_new_digraph_error_1() {
        let path = format!("{PATH}GRAFO_0.txt");
        let result: Result<AdjacencyList, Error> = GraphIO::import_from_file(&path);

        assert!(result.is_err());

        match result {
            Ok(_) => {}
            Err(err) => {
                assert!(err.kind() == ErrorKind::InvalidData);
                assert!(err.to_string().contains("Invalid data was found"));
            }
        }
    }

    #[test]
    fn adj_list_new_undirected_graph_1() {
        let path = format!("{PATH}GRAFO_2.txt");
        let res: Result<AdjacencyList, Error> =
            UndirectedGraphIO::import_undirected_from_file(&path);

        assert!(res.is_ok());

        if let Ok(list) = res {
            assert!(list.order() == 11);
            assert!(list.undirected_size() == 13);
        }
    }

    #[test]
    fn adj_list_new_undirected_graph_2() {
        let path = format!("{PATH}GRAFO_0.txt");
        let res: Result<AdjacencyList, Error> =
            UndirectedGraphIO::import_undirected_from_file(&path);

        assert!(res.is_err());
    }

    #[test]
    fn adj_matrix_new_digraph_1() {
        let path = format!("{PATH}DIGRAFO1.txt");
        let result: Result<AdjacencyMatrix, Error> = GraphIO::import_from_file(&path);

        assert!(result.is_ok());

        if let Ok(matrix) = result {
            assert!(matrix.order() == 13);
            assert!(matrix.size() == 16);
        }
    }

    #[test]
    fn adj_matrix_new_digraph_2() {
        let path = format!("{PATH}DIGRAFO2.txt");
        let result: Result<AdjacencyMatrix, Error> = GraphIO::import_from_file(&path);

        assert!(result.is_ok());

        if let Ok(matrix) = result {
            assert!(matrix.order() == 13);
            assert!(matrix.size() == 17);
        }
    }

    #[test]
    fn adj_matrix_new_digraph_error_1() {
        let path = format!("{PATH}GRAFO_0.txt");
        let result: Result<AdjacencyMatrix, Error> = GraphIO::import_from_file(&path);

        assert!(result.is_err());

        match result {
            Ok(_) => {}
            Err(err) => {
                assert!(err.kind() == ErrorKind::InvalidData);
                assert!(err.to_string().contains("Invalid data was found"));
            }
        }
    }

    #[test]
    fn adj_matrix_new_undirected_graph_1() {
        let path = format!("{PATH}GRAFO_2.txt");
        let res: Result<AdjacencyMatrix, Error> =
            UndirectedGraphIO::import_undirected_from_file(&path);

        assert!(res.is_ok());

        if let Ok(list) = res {
            assert!(list.order() == 11);
            assert!(list.undirected_size() == 13);
        }
    }

    #[test]
    fn adj_matrix_new_undirected_graph_2() {
        let path = format!("{PATH}GRAFO_0.txt");
        let res: Result<AdjacencyMatrix, Error> =
            UndirectedGraphIO::import_undirected_from_file(&path);

        assert!(res.is_err());
    }
}
