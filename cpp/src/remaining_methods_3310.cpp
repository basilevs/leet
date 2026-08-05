// https://leetcode.com/problems/remove-methods-from-project

#include "test.hpp"

#include <cstddef>
#include <vector>

    std::vector<int> remainingMethods(int n, int k, const std::vector<std::vector<int>>& invocations) {
        using namespace std;
        // size_t n = static_cast<size_t>(n);
        vector<vector<int>> graph(n);
        vector<int> in_degree(n, 0);
        for (const auto& edge : invocations) {
            graph[edge[0]].push_back(edge[1]);
            in_degree[edge[1]]++;
        }
        vector<bool> visited(n, false);
        vector<int> stack;
        stack.push_back(k);
        visited[k] = true;
        while (!stack.empty()) {
            int node = stack.back();
            stack.pop_back();
            for (int neighbor : graph[node]) {
                in_degree[neighbor]--;
                if (!visited[neighbor]) {
                    visited[neighbor] = true;
                    stack.push_back(neighbor);
                }
            }
        }
        vector<int> result;
        bool external_reference_found = false;
        for (int i = 0; i < n; ++i) {
            if (visited[i]) {
                if (in_degree[i] > 0) {
                    external_reference_found = true;
                    break;
                }
            } else {
                result.push_back(i);
            }
        }
        if (!external_reference_found) {
            return result;
        }
        result.clear();
        for (int i = 0; i < n; ++i) {
            result.push_back(i);
        }
        return result;
    }

TEST(official1) {
    check_eq(
        std::vector<int>{0, 1, 2, 3},
        remainingMethods(4, 1, {{1, 2}, {0, 1}, {3, 2}})
    );
}

TEST(official2) {
    check_eq(
        std::vector<int>{3, 4},
        remainingMethods(5, 0, {{1, 2}, {0, 2}, {0, 1}, {3, 4}})
    );
}

TEST(official3) {
    check_eq(
        std::vector<int>{},
        remainingMethods(3, 2, {{1, 2}, {0, 1}, {2, 0}})
    );
}
