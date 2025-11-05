use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};

pub fn list(list: &AdjacencyList) {
    for (i, neighbors) in list.0.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}

pub fn matrix(m: &AdjacencyMatrix) {
    for row in &m.0 {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}

pub fn tip() {
    println!("[TIP]: you should try 'make png' to see these graphs!");
}
