/// [329] Longest Increasing Path in a Matrix
/// Difficulty: Hard
/// Topics: Array, Dynamic Programming, Depth-First Search, Breadth-First Search, Graph, Topological Sort, Matrix
/// Tags: NeetCode150
///
/// Given an m x n integers matrix, return the length of the longest increasing path in matrix.
/// From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
///
/// Link: https://leetcode.com/problems/longest-increasing-path-in-a-matrix/

struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]];
        assert_eq!(Solution::longest_increasing_path(matrix), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }
}
