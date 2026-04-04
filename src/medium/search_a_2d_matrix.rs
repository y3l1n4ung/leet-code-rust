/// [74] Search a 2D Matrix
/// Difficulty: Medium
/// Topics: Array, Binary Search, Matrix
/// Tags: NeetCode150
///
/// You are given an m x n integer matrix matrix with the following two properties:
/// - Each row is sorted in ascending order from left to right.
/// - The first integer of each row is greater than the last integer of the previous row.
/// Given an integer target, return true if target is in matrix or false otherwise.
/// You must write a solution in O(log(m * n)) time complexity.
///
/// Link: https://leetcode.com/problems/search-a-2d-matrix/

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix, 3), true);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(Solution::search_matrix(matrix, 13), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
    }
}
