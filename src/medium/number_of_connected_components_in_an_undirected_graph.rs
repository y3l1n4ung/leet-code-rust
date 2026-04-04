/// [323] Number of Connected Components in an Undirected Graph
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Union Find, Graph
/// Tags: NeetCode150
///
/// You have a graph of n nodes. You are given an integer n and an array edges where edges[i] = [ai, bi] indicates that there is an edge between ai and bi in the graph.
/// Return the number of connected components in the graph.
///
/// Link: https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/ (Premium)
/// Free Link: https://www.lintcode.com/problem/591/

struct Solution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let edges = vec![vec![0,1], vec![1,2], vec![3,4]];
        assert_eq!(Solution::count_components(5, edges), 2);
    }

    #[test]
    fn test_2() {
        let edges = vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4]];
        assert_eq!(Solution::count_components(5, edges), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_components(1, vec![]), 1);
    }
}
