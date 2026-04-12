// BFS Traversal Practice 🦀

use std::collections::{VecDeque, HashSet, HashMap};

pub struct Graph {
    // Adjacency list: Node -> [Neighbors]
    pub adj: HashMap<i32, Vec<i32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_edge(&mut self, u: i32, v: i32) {
        self.adj.entry(u).or_insert(Vec::new()).push(v);
        // For undirected graph, add the reverse edge too
        self.adj.entry(v).or_insert(Vec::new()).push(u);
    }
}

pub struct Solution;

impl Solution {
    /// Practice: BFS Traversal
    /// Returns a list of nodes in the order they were visited.
    pub fn bfs(graph: &Graph, start_node: i32) -> Vec<i32> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        queue.push_back(start_node);
        visited.insert(start_node);

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = graph.adj.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let mut graph = Graph::new();
        // 1 - 2
        // |   |
        // 3 - 4
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 4);
        graph.add_edge(3, 4);

        let result = Solution::bfs(&graph, 1);
        
        // BFS for this graph starting at 1: [1, 2, 3, 4] or [1, 3, 2, 4]
        assert_eq!(result[0], 1);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
        assert_eq!(result[3], 4);
    }
}
