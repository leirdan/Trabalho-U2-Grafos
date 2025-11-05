use graph_io::print;
use graph_io::{GraphIO, UndirectedGraphIO};
use graphs_algorithms::graphs::AdjacencyList;

static PATH: &str = "examples/dot/graph_exports_example/";

fn main() {
    // UNDIRECTED GRAPHS
    let graph1 = AdjacencyList(vec![
        vec![1, 2],
        vec![2, 3, 7, 10],
        vec![0, 1, 4, 3],
        vec![2, 6, 8, 7],
        vec![2, 5],
        vec![4],
        vec![3, 9, 10],
        vec![1, 3],
        vec![3, 10],
        vec![6, 10],
        vec![1, 6, 9, 8],
    ]);

    println!("[UNDIRECTED] Graph 1: ");
    print::list(&graph1);
    let path1 = format!("{PATH}graph1.dot");

    let result1 = graph1.export_undirected_to_dot(&path1);
    match result1 {
        Ok(_) => println!("Created .dot file for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let path2 = format!("{PATH}bfs_graph1.dot");
    let result2 = graph1.export_undirected_bfs_to_dot(6, &path2);
    match result2 {
        Ok(_) => println!("Created BFS tree with 6 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let path3 = format!("{PATH}dfs_graph1.dot");
    let result3 = graph1.export_undirected_dfs_to_dot(6, &path3);
    match result3 {
        Ok(_) => println!("Created DFS tree with 6 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    // DIGRAPHS
    let graph2 = AdjacencyList(vec![
        vec![1, 3],
        vec![4],
        vec![4, 0],
        vec![4],
        vec![0],
        vec![2, 6],
        vec![7],
        vec![8],
    ]);

    println!("[DIRECTED] Graph 2: ");
    print::list(&graph2);

    let path4 = format!("{PATH}graph2.dot");
    let result4 = graph2.export_to_dot(&path4);
    match result4 {
        Ok(_) => println!("Created .dot file for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let path5 = format!("{PATH}bfs_graph2.dot");
    let result5 = graph2.export_directed_bfs_to_dot(5, &path5);
    match result5 {
        Ok(_) => println!("Created BFS tree with 5 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let path6 = format!("{PATH}dfs_graph2.dot");
    let result6 = graph2.export_directed_dfs_to_dot(5, &path6);
    match result6 {
        Ok(_) => println!("Created DFS tree with 5 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    print::tip();
}
