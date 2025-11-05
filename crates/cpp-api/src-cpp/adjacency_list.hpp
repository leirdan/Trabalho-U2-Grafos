#pragma once

#include <cstddef>
#include <unordered_set>
#include <vector>
#include <queue>

class AdjacencyList {
   private:
    std::vector<std::vector<size_t>> data;

   public:
    AdjacencyList(size_t node_amt) : data(node_amt) {}

    size_t order() { return data.size(); }

    std::vector<size_t>& neighbors(const size_t node) {
        static std::vector<size_t> empty;
        return (node < order()) ? data[node] : empty;
    }

    void add_edge_unchecked(const size_t n, const size_t m) {
        if (n > order() || m > order()) return;
        data[n].push_back(m);
    }

    void dfs(const size_t start) {
        std::vector<size_t> stack;
        std::unordered_set<size_t> visited;

        stack.reserve(order());
        visited.reserve(order());

        stack.push_back(start);
        visited.insert(start);

        while (!stack.empty()) {
            const auto current = stack.back();
            stack.pop_back();

            for (const auto& neighbor : neighbors(current)) {
                if (visited.insert(neighbor).second) {
                    stack.push_back(neighbor);
                }
            }
        }
    }

    void bfs(const size_t start) {
        std::vector<size_t> res;
        std::queue<size_t> q;
        std::vector<bool> visited(order(), false);

        visited[start] = true;
        q.push(start);

        while (!q.empty()) {
            const auto current = q.front();
            q.pop();
            res.push_back(current);

            for (const auto& neighbor : neighbors(current)) {
                if (!visited[neighbor]) {
                    visited[neighbor] = true;
                    q.push(neighbor);
                }
            }
        }
    }
};
