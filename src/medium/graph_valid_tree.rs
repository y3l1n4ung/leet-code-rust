/// [261] Graph Valid Tree
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Union Find, Graph
/// Tags: NeetCode150
///
/// You have a graph of n nodes labeled from 0 to n - 1. You are given an integer n and a list of edges where edges[i] = [ai, bi] indicates that there is an undirected edge between nodes ai and bi in the graph.
/// Return true if the edges of the given graph make up a valid tree, and false otherwise.
///
/// Link: https://leetcode.com/problems/graph-valid-tree/ (Premium)
/// Free Link: https://www.lintcode.com/problem/178/

struct Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let edges = vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]];
        assert_eq!(Solution::valid_tree(5, edges), true);
    }

    #[test]
    fn test_2() {
        let edges = vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]];
        assert_eq!(Solution::valid_tree(5, edges), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::valid_tree(1, vec![]), true);
    }
}
