// DFS Traversal Practice 🦀

use std::collections::{HashSet, HashMap};

pub struct Graph {
    pub adj: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj.entry(u).or_insert(Vec::new()).push(v);
        self.adj.entry(v).or_insert(Vec::new()).push(u);
    }
}

pub struct Solution;

impl Solution {
    /// Practice: DFS Traversal
    /// Returns a list of nodes in the order they were visited.
    pub fn dfs(graph: &Graph, start_node: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        Self::dfs_recursive(graph, start_node, &mut visited, &mut result);
        result
    }

    fn dfs_recursive(
        graph: &Graph,
        node: i32,
        visited: &mut HashSet<i32>,
        result: &mut Vec<i32>
    ) {
        if visited.contains(&node) { return; }

        visited.insert(node);
        result.push(node);

        if let Some(neighbors) = graph.adj.get(&node) {
            for &neighbor in neighbors {
                Self::dfs_recursive(graph, neighbor, visited, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let mut graph = Graph::new();
        // 1 - 2
        // |   |
        // 3 - 4
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let result = Solution::dfs(&graph, 1);
        
        // DFS starting at 1: [1, 2, 4, 3] or [1, 3, 4, 2]
        assert_eq!(result[0], 1);
        assert!(result.contains(&4));
        assert_eq!(result.len(), 4);
    }
}
