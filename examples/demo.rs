fn main() {
    println!("Hello, world!");
}
// use graph_io::{GraphIO, UndirectedGraphIO, print};
// use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};
// use graphs_algorithms::{Graph, UndirectedGraph};
// use std::collections::{HashMap, HashSet};
// use std::io;
// use std::io::{Error, Write};
//
// fn clear() {
//     print!("\x1B[2J\x1B[1;1H");
// }
//
// fn main() {
//     clear();
//     loop {
//         println!("========================================");
//         println!("Bem-vindo/a à DEMO do trabalho de Grafos!");
//         println!("Escolha a execução que deseja visualizar:");
//         println!("1 - Grafos Não Orientados");
//         println!("2 - Digrafos");
//         println!("0 - Sair");
//         print!("> ");
//         io::stdout().flush().unwrap();
//
//         let mut opt = String::new();
//         io::stdin().read_line(&mut opt).unwrap();
//         let opt = opt.trim();
//
//         match opt {
//             "1" => demo_undirected(),
//             "2" => demo_directed(),
//             "0" => {
//                 println!("Encerrando a execução...");
//                 break;
//             }
//             _ => println!("Opção inválida. Tente novamente."),
//         }
//     }
// }
//
// fn demo_directed() {
//     clear();
//     println!("SEÇÃO DE DIGRAFOS!");
//     loop {
//         println!("========================================");
//         println!("Escolha a questão que deseja executar:");
//         println!("1 - Matriz de Adjacência");
//         println!("2 - Matriz de Incidência");
//         println!("3 - Grafo Subjacente");
//         println!("4 - Busca em Largura (BFS)");
//         println!("5 - Busca em Profundidade (DFS)");
//         println!("0 - Voltar");
//         print!("> ");
//         io::stdout().flush().unwrap();
//
//         let mut opt = String::new();
//         io::stdin().read_line(&mut opt).unwrap();
//         let opt = opt.trim();
//
//         match opt {
//             "1" => digraph1(),
//             "2" => digraph2(),
//             "3" => digraph3(),
//             "4" => digraph4(),
//             "5" => digraph5(),
//             "0" => break,
//             _ => println!("Opção inválida."),
//         }
//
//         println!("\nPressione ENTER para continuar...");
//         let _ = io::stdin().read_line(&mut String::new());
//         clear();
//     }
//
//     clear();
// }
//
// fn digraph1() {
//     println!("1 - Representação do digrafo a partir da Matriz de Adjacência");
//     println!("- Importando do arquivo DIGRAFO1.txt...");
//     let res1: Result<AdjacencyMatrix, Error> =
//         GraphIO::import_from_file("examples/data/DIGRAFO1.txt");
//     let am1 = res1.unwrap();
//     println!("- Temos a seguinte matriz: ");
//     print::matrix(&am1);
// }
//
// fn digraph2() {
//     println!("2 - Representação do digrafo a partir da Matriz de Incidência");
//     println!("- Importando do arquivo DIGRAFO1.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         GraphIO::import_from_file("examples/data/DIGRAFO1.txt");
//     let im1 = IncidenceMatrix::from_adjacency_matrix(&res.unwrap());
//     println!("- Temos a seguinte matriz: ");
//     print::incidence_matrix(&im1);
// }
//
// fn digraph3() {
//     println!("3 - Determinação do grafo subjacente");
//     println!("- Criando lista de Adjacência a partir do DIGRAFO2.txt...");
//     let res3: Result<AdjacencyList, Error> =
//         GraphIO::import_from_file("examples/data/DIGRAFO2.txt");
//     let l = res3.unwrap();
//     println!("- Temos o seguinte digrafo: ");
//     print::list(&l);
//     println!("- Seu grafo subjacente é, portanto: ");
//     print::list(&l.underlying_graph());
// }
//
// fn digraph4() {
//     println!("4 - Busca em Largura");
//     println!("- Importando grafo do arquivo DIGRAFO2.txt e adicionando aresta (8, 10)...");
//     let res: Result<AdjacencyList, Error> = GraphIO::import_from_file("examples/data/DIGRAFO2.txt");
//     let mut l = res.unwrap();
//     l.add_edge(8, 10);
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Vamos aplicar a BFS na lista a partir de dois vértices.");
//     let _ = l.export_directed_bfs_to_dot(0, "examples/dot/demo/digraph/bfs1");
//     println!("- Vértice 0: resultado salvo em examples/dot/demo/digraph/bfs1.dot");
//     let _ = l.export_directed_bfs_to_dot(5, "examples/dot/demo/digraph/bfs2");
//     println!("- Vértice 5: resultado salvo em examples/dot/demo/digraph/bfs2.dot");
//     println!("- Para visualizar, compile os .dot com 'make png' no terminal!");
// }
//
// fn digraph5() {
//     println!(
//         "5 - Busca em Profundidade com determinação de profundidade de entrada e saída e classificação de arestas"
//     );
//     println!("- Importando grafo do arquivo DIGRAFO2.txt e adicionando aresta (8, 10)...");
//     let res: Result<AdjacencyList, Error> = GraphIO::import_from_file("examples/data/DIGRAFO2.txt");
//     let mut l = res.unwrap();
//     l.add_edge(8, 10);
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!(
//         "- Vamos aplicar a DFS com classificação de arestas na lista a partir de dois vértices."
//     );
//     println!("- Vértice 0: resultado salvo em examples/dot/demo/digraph/dfs1.dot");
//     let _ = l.export_directed_dfs_to_dot(0, "examples/dot/demo/digraph/dfs1");
//     println!("- Vértice 5: resultado salvo em examples/dot/demo/digraph/dfs2.dot");
//     let _ = l.export_directed_dfs_to_dot(5, "examples/dot/demo/digraph/dfs2");
//     println!("- Para visualizar, compile os .dot com 'make png' no terminal!");
// }
//
// fn demo_undirected() {
//     clear();
//     println!("SEÇÃO DE GRAFOS NÃO ORIENTADOS!");
//     loop {
//         println!("========================================");
//         println!("Escolha a questão que deseja executar:");
//         println!("1 - Criação do grafo a partir da Lista de Adjacências");
//         println!("2 - Criação do grafo a partir da Matriz de Adjacências");
//         println!("3 - Criação do grafo a partir da Matriz de Incidência");
//         println!("4 - Conversão entre Matriz e Lista de Adjacência");
//         println!("5 - Cálculo do grau de cada vértice");
//         println!("6 - Determinar adjacência entre dois vértices");
//         println!("7 - Determinar número total de vértices");
//         println!("8 - Determinar número total de arestas");
//         println!("9 - Inclusão de um novo vértice");
//         println!("10 - Exclusão de um vértice");
//         println!("11 - Verificar se o grafo é conexo");
//         println!("12 - Verificar se o grafo é bipartido");
//         println!("13 - Busca em Largura");
//         println!("14 - Busca em Profundidade");
//         println!("15 - Determinação de articulações e blocos");
//         println!("0 - Voltar");
//         print!("> ");
//         io::stdout().flush().unwrap();
//
//         let mut opt = String::new();
//         io::stdin().read_line(&mut opt).unwrap();
//         let opt = opt.trim();
//
//         match opt {
//             "1" => undirected1(),
//             "2" => undirected2(),
//             "3" => undirected3(),
//             "4" => undirected4(),
//             "5" => undirected5(),
//             "6" => undirected6(),
//             "7" => undirected7(),
//             "8" => undirected8(),
//             "9" => undirected9(),
//             "10" => undirected10(),
//             "11" => undirected11(),
//             "12" => undirected12(),
//             "13" => undirected13(),
//             "14" => undirected14(),
//             "15" => undirected15(),
//             "0" => break,
//             _ => println!("Opção inválida."),
//         }
//
//         println!("\nPressione ENTER para continuar...");
//         let _ = io::stdin().read_line(&mut String::new());
//         clear();
//     }
//     clear();
// }
//
// fn undirected1() {
//     println!("1 - Criação do grafo a partir da Lista de Adjacências");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     print::list(&res.unwrap());
// }
//
// fn undirected2() {
//     println!("2 - Criação do grafo a partir da Matriz de Adjacências");
//     println!("Importando do arquivo GRAFO_2.txt...");
//
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     print::matrix(&res.unwrap());
// }
//
// fn undirected3() {
//     println!("3 - Criação do grafo a partir da Matriz de Incidência");
//     println!("Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//
//     let im = IncidenceMatrix::from_adjacency_matrix(&res.unwrap());
//     print::incidence_matrix(&im);
// }
//
// fn undirected4() {
//     println!("4 - Conversão entre Matriz de Adjacência e Lista de Adjacência");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     println!("4.1 - Matriz -> Lista");
//     let mut m = res.unwrap();
//     println!("- Temos a seguinte matriz:");
//     print::matrix(&m);
//
//     println!("- Convertendo para lista..");
//     let l = AdjacencyList::from_adjacency_matrix(&m);
//     println!("- Temos a seguinte lista:");
//     print::list(&l);
//
//     println!("4.2 - Lista -> Matriz");
//     println!("- Convertendo a lista anterior para matriz...");
//     m = AdjacencyMatrix::from_adjacency_list(&l);
//     println!("- Temos a seguinte matriz:");
//     print::matrix(&m);
// }
//
// fn undirected5() {
//     println!("5 - Função que calcula o grau de cada vértice");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let m = res.unwrap();
//     println!("- Grau de cada vértice do grafo:");
//     for i in 0..m.order() {
//         println!("Vértice {} tem grau {}.", i, m.undirected_node_degree(i));
//     }
// }
//
// fn undirected6() {
//     println!("6 - Função que determina a adjacência entre dois vértices");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let m = res.unwrap();
//     println!("Aresta (0, 1): {} ", m.has_edge(0, 1));
//     println!("Aresta (5, 0): {} ", m.has_edge(5, 0));
//     println!("Aresta (1, 3): {} ", m.has_edge(1, 3));
// }
//
// fn undirected7() {
//     println!("7 - Função que determina o número total de vértices");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let m = res.unwrap();
//
//     println!("- A ordem do grafo é {} ", m.order());
// }
//
// fn undirected8() {
//     println!("8 - Função que determina o número total de arestas");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let m = res.unwrap();
//
//     println!("- O tamanho do grafo é {} ", m.undirected_size());
// }
//
// fn undirected9() {
//     println!("9 - Inclusão de um novo vértice");
//     println!("9.1 - Lista de Adjacência");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut l = res.unwrap();
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Adicionemos dois novos vértices inicialmente sem arestas: ");
//     l.add_node(11);
//     l.add_node(12);
//     println!("- Agora, a lista é: ");
//     print::list(&l);
//
//     println!("9.2 - Matriz de Adjacência");
//     println!("- Convertendo da Lista anterior -> Matriz");
//     let mut m = AdjacencyMatrix::from_adjacency_list(&l);
//     println!("Temos a seguinte matriz: ");
//     print::matrix(&m);
//     println!("- Adicionemos dois novos vértices inicialmente sem arestas.");
//     m.add_node(11);
//     m.add_node(12);
//     println!("- Agora, a matriz é: ");
//     print::matrix(&m);
// }
//
// fn undirected10() {
//     println!("10 - Exclusão de um vértice");
//     println!("10.1 - Lista de Adjacência");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut l = res.unwrap();
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Vamos remover os vértices 1 e 10.");
//     l.remove_node(1);
//     l.remove_node(10);
//     println!("- A lista com os vértices atualizados é: ");
//     print::list(&l);
//
//     println!("10.2 - Matriz de Adjacência");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyMatrix, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut m = res.unwrap();
//     println!("- Temos a seguinte matriz: ");
//     print::matrix(&m);
//     println!("- Vamos eliminar os vértices 1 e 10. ");
//     m.remove_node(1);
//     m.remove_node(10);
//     println!("- Agora, a matriz com os nós atualizados é: ");
//     print::matrix(&m);
// }
//
// fn undirected11() {
//     println!("11 - Função que determina se um grafo é conexo");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut l = res.unwrap();
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Considerando a lista acima, vamos investigar. ");
//     println!("- O grafo é conexo: {}", l.connected());
//     println!("- Adicionando arestas (4, 5), (5, 10) e (9, 0), temos a lista: ");
//     l.add_undirected_edge(4, 5);
//     l.add_undirected_edge(5, 10);
//     l.add_undirected_edge(9, 0);
//     print::list(&l);
//     println!("- O grafo é conexo: {}", l.connected());
// }
//
// fn undirected12() {
//     println!("12 - Determinar se um grafo é bipartido");
//     println!("- Importando do arquivo GRAFO_2.txt...");
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let l = res.unwrap();
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- O grafo é bipartido: {}", l.bipartite());
// }
//
// fn undirected13() {
//     println!("13 - Busca em Largura");
//     println!(
//         "- Importando do arquivo GRAFO_2.txt e adicionando as arestas (1, 5), (4, 5), (5, 10), (9, 0)..."
//     );
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut l = res.unwrap();
//     l.add_undirected_edge(1, 5);
//     l.add_undirected_edge(4, 5);
//     l.add_undirected_edge(5, 10);
//     l.add_undirected_edge(9, 0);
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Vamos aplicar a BFS na lista acima a partir de dois vértices.");
//     println!("- Vértice 0: resultado salvo em examples/dot/demo/undirected_graph/bfs1.dot");
//     let _ = l.export_undirected_bfs_to_dot(0, "examples/dot/demo/undirected_graph/bfs1");
//     println!("- Vértice 5: resultado salvo em examples/dot/demo/undirected_graph/bfs2.dot");
//     let _ = l.export_undirected_bfs_to_dot(5, "examples/dot/demo/undirected_graph/bfs2");
//     println!("- Para visualizar, compile os .dot com 'make png' no terminal!");
// }
//
// fn undirected14() {
//     println!("14 - Busca em Profundidade com Arestas de Retorno");
//     println!(
//         "- Importando do arquivo GRAFO_2.txt e adicionando as arestas (1, 5), (4, 5), (5, 10), (9, 0)..."
//     );
//     let res: Result<AdjacencyList, Error> =
//         UndirectedGraphIO::import_undirected_from_file("examples/data/GRAFO_2.txt");
//     let mut l = res.unwrap();
//     l.add_undirected_edge(1, 5);
//     l.add_undirected_edge(4, 5);
//     l.add_undirected_edge(5, 10);
//     l.add_undirected_edge(9, 0);
//     println!("- Temos a seguinte lista: ");
//     print::list(&l);
//     println!("- Vamos aplicar a DFS na lista acima a partir de dois vértices.");
//     println!("- Vértice 0: resultado salvo em examples/dot/demo/undirected_graph/dfs1.dot");
//     let _ = l.export_undirected_dfs_to_dot(0, "examples/dot/demo/undirected_graph/dfs1");
//     println!("- Vértice 5: resultado salvo em examples/dot/demo/undirected_graph/dfs2.dot");
//     let _ = l.export_undirected_dfs_to_dot(5, "examples/dot/demo/undirected_graph/dfs2");
//     println!("- Para visualizar, compile os .dot com 'make png' no terminal!");
// }
//
// fn undirected15() {
//     println!("15 - Determinação de articulações e blocos (biconectividade)");
//     let l = AdjacencyList(vec![
//         vec![1, 2],
//         vec![0, 2],
//         vec![0, 1, 3, 4, 5],
//         vec![2, 5],
//         vec![2, 6, 7],
//         vec![2, 3],
//         vec![4, 7, 8],
//         vec![4, 6, 8],
//         vec![6, 7],
//     ]);
//     print::list(&l);
//
//     println!(
//         "- Vamos aplicar o algoritmo de biconectividade no grafo acima a partir do vértice 0."
//     );
//
//     let mut vertex_blocks: HashMap<usize, usize> = HashMap::new();
//     let mut block_vertices: Vec<HashSet<usize>> = Vec::new();
//
//     for (i, comp) in l.biconnected_components(0).enumerate() {
//         let mut vertices = HashSet::new();
//
//         println!("  Componente {}:", i + 1);
//         for (u, v) in &comp {
//             println!("    ({}, {})", u, v);
//             vertices.insert(*u);
//             vertices.insert(*v);
//         }
//
//         for v in &vertices {
//             *vertex_blocks.entry(*v).or_insert(0) += 1;
//         }
//
//         block_vertices.push(vertices);
//     }
//
//     let articulations: HashSet<usize> = vertex_blocks
//         .iter()
//         .filter(|(_, count)| **count > 1)
//         .map(|(v, _)| *v)
//         .collect();
//
//     for (i, vertices) in block_vertices.iter().enumerate() {
//         let block_articulations: Vec<_> = vertices.intersection(&articulations).copied().collect();
//         println!(
//             "Componente {} → articulações: {:?}",
//             i + 1,
//             block_articulations
//         );
//     }
// }
