/// [46] Permutations
/// Difficulty: Medium
/// Topics: Array, Backtracking
/// Tags: NeetCode150
///
/// Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.
///
/// Link: https://leetcode.com/problems/permutations/

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::permute(vec![1, 2, 3]);
        result.sort();
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_2() {
        let mut result = Solution::permute(vec![0, 1]);
        result.sort();
        let mut expected = vec![vec![0, 1], vec![1, 0]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
