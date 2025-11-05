use graph_io::print;
use graphs_algorithms::{Graph, UndirectedGraph, graphs::AdjacencyMatrix};

fn main() {
    println!("===== DIGRAPH OPERATIONS =====");
    println!("Creating a digraph on adjacency matrix...");

    let mut dm1 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ]);
    print::matrix(&dm1);

    println!("Adding new node 5");
    dm1.add_node(5);
    print::matrix(&dm1);

    println!("Adding edge 1 -> 5");
    dm1.add_edge(1, 5);
    print::matrix(&dm1);

    println!("Adding edge 2 -> 5");
    dm1.add_edge(2, 5);
    print::matrix(&dm1);

    println!("Adding edges 0 -> 4 and 0 -> 0 ");
    dm1.add_edge(0, 4);
    dm1.add_edge(0, 0);
    print::matrix(&dm1);

    println!("Deleting edge 2 -> 5");
    dm1.remove_edge(2, 5);
    print::matrix(&dm1);

    println!("Deleting edge 2 -> 4 (which doesn't exist)");
    dm1.remove_edge(2, 4);
    print::matrix(&dm1);

    println!("Delete node 2");
    dm1.remove_node(2);
    print::matrix(&dm1);

    println!("===== UNDIRECTED GRAPH OPERATIONS =====");
    println!("Creating an undirected graph on adjacency matrix...");

    let mut um1 = AdjacencyMatrix(vec![
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 1, 0],
    ]);

    print::matrix(&um1);

    println!("Adding edge 0 - 5");
    um1.add_undirected_edge(0, 5);
    print::matrix(&um1);

    println!("Adding edge 1 - 1");
    um1.add_undirected_edge(1, 1);
    print::matrix(&um1);

    println!("Delete edge 3 - 4 ");
    um1.remove_undirected_edge(3, 4);
    print::matrix(&um1);

    println!("Delete node 5");
    um1.remove_node(5);
    print::matrix(&um1);
}
