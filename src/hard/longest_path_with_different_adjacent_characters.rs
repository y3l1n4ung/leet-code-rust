/// [2246] Longest Path With Different Adjacent Characters
/// Difficulty: Hard
/// Topics: Array, String, Tree, Depth-First Search, Graph, Topological Sort
/// Tags: NeetCode150
///
/// You are given a tree (i.e. a connected, undirected graph that has no cycles) rooted at node 0 consisting of n nodes numbered from 0 to n - 1. The tree is represented by a 0-indexed array parent of size n, where parent[i] is the parent of node i. Since node 0 is the root, parent[0] == -1.
/// You are also given a string s of length n, where s[i] is the character assigned to node i.
/// Return the length of the longest path in the tree such that no two adjacent nodes on the path have the same character assigned to them.
///
/// Link: https://leetcode.com/problems/longest-path-with-different-adjacent-characters/

struct Solution;

impl Solution {
    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_path(vec![-1, 0, 0, 0], "aabc".to_string()),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_path(vec![-1], "z".to_string()), 1);
    }
}
