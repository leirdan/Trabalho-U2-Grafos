#include "adjacency_list.hpp"

#include <cstddef>

extern "C" {
void* mk_adjacency_list(size_t node_amt) { return new AdjacencyList(node_amt); }

void add_edge_unchecked(void* graph, size_t n, size_t m) {
    AdjacencyList* adj_list = static_cast<AdjacencyList*>(graph);
    adj_list->add_edge_unchecked(n, m);
}

void dfs(void* graph, size_t start) {
    AdjacencyList* adj_list = static_cast<AdjacencyList*>(graph);
    return adj_list->dfs(start);
}

void bfs(void* graph, size_t start) {
    AdjacencyList* adj_list = static_cast<AdjacencyList*>(graph);
    return adj_list->bfs(start);
}
}
