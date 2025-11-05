use graph_io::GraphIO;
use graph_io::print;
use graphs_algorithms::Edge;
use graphs_algorithms::Graph;
use graphs_algorithms::graphs::AdjacencyList;

fn main() {
    let digraph = AdjacencyList(vec![
        vec![1, 2, 7],
        vec![3],
        vec![],
        vec![2, 4],
        vec![1, 5],
        vec![3],
        vec![5, 7],
        vec![6],
        vec![4, 9],
        vec![],
    ]);
    println!("Digraph: ");
    print::list(&digraph);

    let print_edge = |e| match e {
        Edge::Tree(v, u) => println!("Tree: {} -> {}", v, u),
        Edge::Back(v, u) | Edge::ParentBack(v, u) => {
            println!("Back: {} -> {}", v, u)
        }
        Edge::Forward(v, u) => println!("Forward: {} -> {}", v, u),
        Edge::Cross(v, u) => println!("Cross: {} -> {}", v, u),
    };

    let mut iter = digraph.classify_edges(0);
    for e in &mut iter {
        print_edge(e);
    }
    iter.new_start(8);
    for e in &mut iter {
        print_edge(e)
    }

    let path = "examples/dot/classify_edges/directed.dot";
    let _ = digraph.export_directed_dfs_to_dot(0, path);

    println!("Graph was exported to dot file on path {}! ", path);

    print::tip();
}
