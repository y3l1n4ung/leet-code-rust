/// [48] Rotate Image
/// Difficulty: Medium
/// Topics: Array, Math, Matrix
/// Tags: Blind75, NeetCode150
///
/// You are given an n x n 2D matrix representing an image, rotate the image by 90 degrees (clockwise).
/// You have to rotate the image in-place, which means you have to modify the input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
///
/// Link: https://leetcode.com/problems/rotate-image/

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let expected = vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_2() {
        let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        let expected = vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_3() {
        let mut matrix = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![1]]);
    }
}
