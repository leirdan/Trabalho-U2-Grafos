use graph_io::print;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};
use graphs_algorithms::{Graph, UndirectedGraph};

fn main() {
    let dg1 = AdjacencyList(vec![vec![1, 3], vec![2], vec![1], vec![2, 4], vec![]]);

    println!("Current digraph on adjacency list:");
    print::list(&dg1);

    println!("Graph order: {} ", dg1.order());
    println!("Graph size: {} ", dg1.size());
    println!("Degree calculation!");
    for i in 0..dg1.order() {
        let degree = dg1.node_degrees(i);
        println!(
            "Vertex {} has internal degree {} and external degree {}",
            i, degree.0, degree.1
        );
    }

    let ug1 = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);
    println!("Current undirected graph on adjacency matrix:");
    print::matrix(&ug1);

    println!("Graph order: {} ", ug1.order());
    println!("Graph size: {} ", ug1.undirected_size());
    println!("Degree calculation!");
    for i in 0..ug1.order() {
        println!("Vertex {} has degree {}", i, ug1.undirected_node_degree(i));
    }
}
