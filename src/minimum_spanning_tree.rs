use std::collections::{HashMap, HashSet};

use crate::graph::{Node, UndirectedGraph};

/*
Kruskal — descrição teórica e correlação com o código abaixo

Pseudocódigo (resumo):
Kruskal(Grafo G, matriz D)
  Ordene as arestas de G em ordem crescente dos pesos d_ij no vetor H = {h_i}
  T ← ∅
  i ← 1
  Enquanto |T| < n-1 faça
    Se T ∪ h_i é acíclico então
      T ← T ∪ h_i
    i ← i + 1
  Escreva T

No código abaixo:
- A coleta e canonicalização das arestas corresponde à construção do vetor H.
- A ordenação `edges.sort_by(...)` implementa a ordenação por peso (com tie-break determinístico).
- A estrutura `parent`/`rank` implementa o Union-Find (para testar se T ∪ h_i forma ciclo).
  - `find` corresponde à operação de encontrar o representante da componente.
  - `union` corresponde à operação de unir componentes quando a aresta é aceita.
- O método Iterator::next percorre as arestas de H na ordem: cada chamada considera a próxima aresta h_i
  e retorna um evento EdgeAdded (aceita) ou EdgeSkipped (descartada por formar ciclo).
- Ao final das iterações, as arestas EdgeAdded formam a árvore geradora mínima T (ou floresta, se desconexo).
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KruskalEvent<T>
where
    T: Node + Ord,
{
    EdgeConsidered(T, T, i32),
    EdgeAdded(T, T, i32),
    EdgeSkipped(T, T, i32),
}

/// Iterador passo-a-passo do algoritmo de Kruskal.
///
/// Implementação:
/// - new(...) inicializa as estruturas (coleta arestas H, ordena, inicializa union-find).
/// - cada chamada a next() realiza um único passo: considera a próxima aresta h_i e decide adicioná-la
///   à MST (EdgeAdded) ou descartá-la (EdgeSkipped) usando o Union-Find para detectar ciclos.
pub struct KruskalIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    _graph: &'a G,
    edges: Vec<(T, T, i32)>,
    idx: usize,
    parent: HashMap<T, T>,
    rank: HashMap<T, usize>,
}

impl<'a, T, G> KruskalIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    /// Inicializa o iterador: coleta nós e arestas
    pub fn new(graph: &'a G) -> Self {
        let nodes: Vec<T> = graph.nodes().collect();
        let mut parent: HashMap<T, T> = HashMap::with_capacity(nodes.len());
        let mut rank: HashMap<T, usize> = HashMap::with_capacity(nodes.len());
        for &n in &nodes {
            parent.insert(n, n);
            rank.insert(n, 0);
        }

        // coleta arestas sem duplicatas
        let mut seen: HashSet<(T, T)> = HashSet::with_capacity(nodes.len() * 2);
        let mut edges: Vec<(T, T, i32)> = Vec::new();

        for &u in &nodes {
            if let Some(neis) = graph.weighted_neighbors(u) {
                for &(v, w) in neis {
                    let (a, b) = if u <= v { (u, v) } else { (v, u) };
                    if seen.insert((a, b)) {
                        edges.push((a, b, w));
                    }
                }
            }
        }

        // ordena por peso, tie-break por (u, v)
        edges.sort_by(|(ua, va, wa), (ub, vb, wb)| {
            wa.cmp(wb).then_with(|| ua.cmp(ub)).then_with(|| va.cmp(vb))
        });

        KruskalIter {
            _graph: graph,
            edges,
            idx: 0,
            parent,
            rank,
        }
    }

    // find com compressão de caminho (corresponde à operação FIND do Union-Find)
    fn find(&mut self, x: T) -> T {
        let p = *self.parent.get(&x).unwrap();
        if p != x {
            let r = self.find(p);
            self.parent.insert(x, r);
            r
        } else {
            x
        }
    }

    // union by rank (corresponde à operação UNION do Union-Find)
    fn union(&mut self, x: T, y: T) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        let rrx = *self.rank.get(&rx).unwrap_or(&0);
        let rry = *self.rank.get(&ry).unwrap_or(&0);
        if rrx < rry {
            self.parent.insert(rx, ry);
        } else if rrx > rry {
            self.parent.insert(ry, rx);
        } else {
            self.parent.insert(ry, rx);
            self.rank.insert(rx, rrx + 1);
        }
    }
}

impl<'a, T, G> Iterator for KruskalIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    type Item = KruskalEvent<T>;

    /// Considera a próxima aresta; retorna EdgeAdded se a aresta foi incluída na MST, EdgeSkipped caso contrário.
    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.edges.len() {
            let (u, v, w) = self.edges[self.idx];
            self.idx += 1;

            let ru = self.find(u);
            let rv = self.find(v);
            if ru != rv {
                // adiciona à MST
                self.union(ru, rv);
                return Some(KruskalEvent::EdgeAdded(u, v, w));
            } else {
                return Some(KruskalEvent::EdgeSkipped(u, v, w));
            }
        }
        None
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/*
Prim — descrição teórica e correlação com o código abaixo

Pseudocódigo (resumo):
Prim(Grafo G , matriz D)
  Escolha qualquer vértice i ∈ V
  Z ← { i }
  N ← V \ { i }
  T ← ∅
  Enquanto |N| > 0 faça
    Encontrar aresta (j,k) com j ∈ Z, k ∈ N que minimize d_jk
    Z ← Z ∪ { k }
    N ← N \ { k }
    T ← T ∪ (j,k)
  Escreva T

No código abaixo:
- `visited` corresponde ao conjunto Z (vértices já incorporados).
- `heap` (fila de prioridade) mantém as arestas candidatas que conectam Z a N, ordenadas por (peso, tie-break).
- `seed_next_component` escolhe um novo vértice inicial quando o grafo é desconexo (equivalente a escolher i ∈ V para cada componente).
- A cada `next()` o iterador:
  - extrai a aresta de menor peso do heap (mínimo d_jk com j∈Z, k∈N).
  - se a aresta conecta um vértice visitado a um não-visitado, ela é adicionada à árvore (EdgeAdded) e o novo vértice é marcado;
  - caso contrário (ambos visitados) a aresta é descartada (EdgeSkipped).
- A expansão das arestas do novo vértice para o heap corresponde a atualizar as arestas candidatas que ligam Z a N.
*/

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimEvent<T>
where
    T: Node + Ord,
{
    EdgeAdded(T, T, i32),
    EdgeSkipped(T, T, i32),
}

/// Iterador passo-a-passo do algoritmo de Prim.
///
/// Implementação:
/// - new(...) prepara `visited`, `heap` e sementeia a primeira componente.
/// - cada next() realiza um passo: retira a aresta mínima do heap e decide adicioná-la à MST
///   se ela conecta Z a N (EdgeAdded) ou descartá-la se conecta vértices já visitados (EdgeSkipped).
#[allow(dead_code)]
pub struct PrimIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    _graph: &'a G,
    visited: HashSet<T>,
    heap: BinaryHeap<Reverse<(i32, T, T)>>,
    nodes: Vec<T>,
    next_seed: usize,
}

#[allow(dead_code)]
impl<'a, T, G> PrimIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    /// Inicializa o iterador de Prim; não executa todo o algoritmo, apenas prepara estruturas.
    pub fn new(graph: &'a G) -> Self {
        let nodes: Vec<T> = graph.nodes().collect();
        let visited: HashSet<T> = HashSet::with_capacity(nodes.len());
        let heap = BinaryHeap::new();

        let mut it = PrimIter {
            _graph: graph,
            visited,
            heap,
            nodes,
            next_seed: 0,
        };

        // seed primeira componente (se houver)
        it.seed_next_component();
        it
    }

    // marca o próximo nó não visitado como visitado e insere suas arestas no heap
    fn seed_next_component(&mut self) {
        while self.next_seed < self.nodes.len() && self.visited.len() == self.nodes.len() {
            self.next_seed += 1;
        }
        while self.next_seed < self.nodes.len()
            && self.visited.contains(&self.nodes[self.next_seed])
        {
            self.next_seed += 1;
        }
        if self.next_seed < self.nodes.len() {
            let s = self.nodes[self.next_seed];
            self.visited.insert(s);
            if let Some(neis) = self._graph.weighted_neighbors(s) {
                for &(v, w) in neis {
                    let (a, b) = if s <= v { (s, v) } else { (v, s) };
                    self.heap.push(Reverse((w, a, b)));
                }
            }
            self.next_seed += 1;
        }
    }
}

impl<'a, T, G> Iterator for PrimIter<'a, T, G>
where
    T: Node + Ord,
    G: UndirectedGraph<T> + ?Sized,
{
    type Item = PrimEvent<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // se heap vazio, tentar seed em próxima componente; se não houver, termina
            if self.heap.is_empty() {
                // se ainda houver algum nó não visitado, seed e continue
                if self.visited.len() < self.nodes.len() {
                    self.seed_next_component();
                    if self.heap.is_empty() {
                        continue;
                    }
                } else {
                    return None;
                }
            }

            if let Some(Reverse((w, u, v))) = self.heap.pop() {
                let u_vis = self.visited.contains(&u);
                let v_vis = self.visited.contains(&v);

                // ambas visitadas -> aresta descartada
                if u_vis && v_vis {
                    return Some(PrimEvent::EdgeSkipped(u, v, w));
                }

                // deve haver exatamente um visitado; se nenhum (pode acontecer em grafos desconexos),
                // ignorar e continuar
                if u_vis ^ v_vis {
                    // determina qual é o novo vértice
                    let new = if u_vis { v } else { u };
                    // adiciona aresta canonizada (min, max)
                    let (a_out, b_out) = if u <= v { (u, v) } else { (v, u) };
                    self.visited.insert(new);
                    // empilha arestas do novo vértice
                    if let Some(neis) = self._graph.weighted_neighbors(new) {
                        for &(nv, w2) in neis {
                            let (aa, bb) = if new <= nv { (new, nv) } else { (nv, new) };
                            self.heap.push(Reverse((w2, aa, bb)));
                        }
                    }
                    return Some(PrimEvent::EdgeAdded(a_out, b_out, w));
                } else {
                    // nenhum visitado: aresta entre nós não iniciados — ignorar e continuar
                    continue;
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adjacency_list::AdjacencyList;
    use std::collections::{HashMap, HashSet};

    fn build_test_map() -> HashMap<usize, HashSet<(usize, i32)>> {
        let mut map: HashMap<usize, HashSet<(usize, i32)>> = HashMap::new();
        let insert_undir =
            |m: &mut HashMap<usize, HashSet<(usize, i32)>>, a: usize, b: usize, w: i32| {
                m.entry(a).or_insert_with(HashSet::new).insert((b, w));
                m.entry(b).or_insert_with(HashSet::new).insert((a, w));
            };

        // Grafo:
        // 1-3 7
        // 2-3 2
        // 2-5 8
        // 2-6 7
        // 3-4 6
        // 3-6 1
        // 4-7 6
        // 5-6 2
        // 5-8 1
        // 6-8 4
        // 6-9 1
        // 6-7 5
        // 7-10 2
        // 8-9 6
        // 9-10 5
        insert_undir(&mut map, 1, 3, 7);
        insert_undir(&mut map, 2, 3, 2);
        insert_undir(&mut map, 2, 5, 8);
        insert_undir(&mut map, 2, 6, 7);
        insert_undir(&mut map, 3, 4, 6);
        insert_undir(&mut map, 3, 6, 1);
        insert_undir(&mut map, 4, 7, 6);
        insert_undir(&mut map, 5, 6, 2);
        insert_undir(&mut map, 5, 8, 1);
        insert_undir(&mut map, 6, 8, 4);
        insert_undir(&mut map, 6, 9, 1);
        insert_undir(&mut map, 6, 7, 5);
        insert_undir(&mut map, 7, 10, 2);
        insert_undir(&mut map, 8, 9, 6);
        insert_undir(&mut map, 9, 10, 5);

        map
    }

    #[test]
    fn kruskal_graph_adjlist() {
        let map = build_test_map();
        let g: AdjacencyList<usize> = AdjacencyList(map);

        let mut it = g.minimum_spanning_tree_kruskal();

        let mut added: Vec<(usize, usize, i32)> = Vec::new();
        while let Some(ev) = it.next() {
            match ev {
                KruskalEvent::EdgeAdded(u, v, w) => added.push((u, v, w)),
                KruskalEvent::EdgeSkipped(_, _, _) | KruskalEvent::EdgeConsidered(_, _, _) => {}
            }
        }

        // 10 vértices -> 9 arestas na MST
        assert_eq!(added.len(), 9);

        let total_weight: i32 = added.iter().map(|(_, _, w)| *w).sum();
        assert_eq!(total_weight, 27);

        let mut got: HashSet<(usize, usize)> = HashSet::new();
        for (u, v, _) in &added {
            let e = if u < v { (*u, *v) } else { (*v, *u) };
            got.insert(e);
        }

        let mut expected: HashSet<(usize, usize)> = HashSet::new();
        expected.insert((3, 6));
        expected.insert((5, 8));
        expected.insert((6, 9));
        expected.insert((2, 3));
        expected.insert((5, 6));
        expected.insert((7, 10));
        expected.insert((6, 7));
        expected.insert((3, 4));
        expected.insert((1, 3));

        assert_eq!(got, expected);
    }

    #[test]
    fn prim_graph_adjlist() {
        let map = build_test_map();
        let g: AdjacencyList<usize> = AdjacencyList(map);

        let mut it = PrimIter::new(&g);

        let mut added: Vec<(usize, usize, i32)> = Vec::new();
        while let Some(ev) = it.next() {
            match ev {
                PrimEvent::EdgeAdded(u, v, w) => added.push((u, v, w)),
                PrimEvent::EdgeSkipped(_, _, _) => {}
            }
        }

        // 10 vértices -> 9 arestas na MST
        assert_eq!(added.len(), 9);

        let total_weight: i32 = added.iter().map(|(_, _, w)| *w).sum();
        assert_eq!(total_weight, 27);

        let mut got: HashSet<(usize, usize)> = HashSet::new();
        for (u, v, _) in &added {
            let e = if u < v { (*u, *v) } else { (*v, *u) };
            got.insert(e);
        }

        let mut expected: HashSet<(usize, usize)> = HashSet::new();
        expected.insert((3, 6));
        expected.insert((5, 8));
        expected.insert((6, 9));
        expected.insert((2, 3));
        expected.insert((5, 6));
        expected.insert((7, 10));
        expected.insert((6, 7));
        expected.insert((3, 4));
        expected.insert((1, 3));

        assert_eq!(got, expected);
    }
}
