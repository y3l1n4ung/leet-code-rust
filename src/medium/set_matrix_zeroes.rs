/// [73] Set Matrix Zeroes
/// Difficulty: Medium
/// Topics: Array, Hash Table, Matrix
/// Tags: Blind75, NeetCode150
///
/// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
/// You must do it in place.
///
/// Link: https://leetcode.com/problems/set-matrix-zeroes/

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        let expected = vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
        let expected = vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_3() {
        let mut matrix = vec![vec![1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }
}
