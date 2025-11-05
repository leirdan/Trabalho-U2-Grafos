use std::io::Error;

use graph_io::print;
use graph_io::{GraphIO, UndirectedGraphIO};
use graphs_algorithms::graphs::AdjacencyList;

static PATH: &str = "examples/dot/";
fn main() {
    let g1 = AdjacencyList(vec![
        vec![1, 7, 2],
        vec![3, 4, 0],
        vec![3, 0],
        vec![1, 2, 4, 5],
        vec![1, 3, 5, 8],
        vec![3, 4, 6, 7],
        vec![5, 7],
        vec![0, 5, 6],
        vec![4, 9],
        vec![8],
    ]);

    println!("BFS on following undirected graph: ");
    print::list(&g1);

    let path1 = format!("{PATH}bfs/bfs_1");
    let _ = g1.export_undirected_bfs_to_dot(1, &path1);
    println!(
        "Undirected graph was exported to dot file on path {}! ",
        path1
    );

    let res: Result<AdjacencyList, Error> = GraphIO::import_from_file("examples/data/DIGRAFO1.txt");

    match res {
        Ok(dg) => {
            println!("BFS on following digraph: ");
            print::list(&g1);

            let path2 = format!("{PATH}bfs/bfs_2");
            let _ = dg.export_directed_bfs_to_dot(1, &path2);
            println!("Digraph was exported to dot file on path {}! ", path2);
        }
        Err(_) => println!("oops.. shouldn't fall here"),
    }

    print::tip();
}
