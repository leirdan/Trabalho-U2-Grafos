use graph_io::print;
use graphs_algorithms::{Graph, UndirectedGraph, graphs::AdjacencyList};

fn main() {
    println!("===== DIGRAPH OPERATIONS =====");
    println!("Creating a digraph on adjacency list...");
    let mut dg1 = AdjacencyList(vec![vec![3], vec![], vec![1], vec![1, 4], vec![5], vec![2]]);
    print::list(&dg1);

    println!("Adding new node: 6");
    dg1.add_node(6);
    print::list(&dg1);

    println!("Adding new edge 0 -> 6");
    dg1.add_edge(0, 6);
    print::list(&dg1);

    println!("Adding new edges 1 -> 6 and 1 -> 4");
    dg1.add_edge(1, 6);
    dg1.add_edge(1, 4);
    print::list(&dg1);

    println!("Adding new edge 6 - 6");
    dg1.add_edge(6, 6);
    print::list(&dg1);

    println!("Deleting edge 3 -> 1");
    dg1.remove_edge(3, 1);
    print::list(&dg1);

    println!("Delete Node 6");
    dg1.remove_node(6);
    print::list(&dg1);

    println!("===== UNDIRECTED GRAPH OPERATIONS =====");
    println!("Creating an undirected graph on adjacency list...");
    let mut ug1 = AdjacencyList(vec![
        vec![1, 2],
        vec![0, 2],
        vec![0, 1, 3, 4],
        vec![2],
        vec![2],
    ]);

    print::list(&ug1);

    println!("Adding new node: 6");
    ug1.add_node(6);
    print::list(&ug1);

    println!("Adding edge 0 - 6");
    ug1.add_undirected_edge(0, 6);
    print::list(&ug1);

    println!("Adding edge 5 - 6");
    ug1.add_undirected_edge(5, 6);
    print::list(&ug1);

    println!("Deleting edge 2 -> 1");
    ug1.remove_undirected_edge(2, 1);
    print::list(&ug1);

    println!("Delete Node 2");
    ug1.remove_node(2);
    print::list(&ug1);
}
