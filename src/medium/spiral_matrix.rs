/// [54] Spiral Matrix
/// Difficulty: Medium
/// Topics: Array, Matrix
/// Tags: Blind75, NeetCode150
///
/// Given an m x n matrix, return all elements of the matrix in spiral order.
///
/// Link: https://leetcode.com/problems/spiral-matrix/

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,6,9,8,7,4,5]);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        assert_eq!(Solution::spiral_order(matrix), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::spiral_order(vec![]), Vec::<i32>::new());
    }
}
