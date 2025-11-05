use std::io::Error;

use graph_io::UndirectedGraphIO;
use graph_io::print;
use graphs_algorithms::graphs::AdjacencyMatrix;

static PATH: &str = "examples/data/";
fn main() {
    let path1 = format!("{PATH}GRAFO_2.txt");
    println!("Importing valid graph from {} ", path1);
    let res: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file(&path1);

    let m1 = res.unwrap();
    print::matrix(&m1);

    let path2 = format!("{PATH}GRAFO_0.txt");
    println!(
        "Trying to import graph without integer nodes from {} ",
        path2
    );
    let res2: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file(&path2);

    match res2 {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{e}"),
    }
}
