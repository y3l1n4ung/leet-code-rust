/// [684] Redundant Connection
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Union Find, Graph
/// Tags: Blind75, NeetCode150
///
/// In this problem, a tree is an undirected graph that is connected and has no cycles.
/// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
/// Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
///
/// Link: https://leetcode.com/problems/redundant-connection/

struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_redundant_connection(vec![vec![1,2], vec![1,3], vec![2,3]]), vec![2,3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_redundant_connection(vec![vec![1,2], vec![2,3], vec![3,4], vec![1,4], vec![1,5]]), vec![1,4]);
    }

    #[test]
    fn test_3() {
        // Just a basic test to ensure it handles input
        assert_eq!(Solution::find_redundant_connection(vec![vec![1,2], vec![2,1]]), vec![2,1]);
    }
}
