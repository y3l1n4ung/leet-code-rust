/// [994] Rotting Oranges
/// Difficulty: Medium
/// Topics: Array, Breadth-First Search, Matrix
/// Tags: NeetCode150
///
/// You are given an m x n grid where each cell can have one of three values:
/// - 0 representing an empty cell,
/// - 1 representing a fresh orange, or
/// - 2 representing a rotten orange.
/// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
/// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
///
/// Link: https://leetcode.com/problems/rotting-oranges/

struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn test_2() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }

    #[test]
    fn test_3() {
        let grid = vec![vec![0, 2]];
        assert_eq!(Solution::oranges_rotting(grid), 0);
    }
}
