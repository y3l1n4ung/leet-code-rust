/// [300] Longest Increasing Subsequence
/// Difficulty: Medium
/// Topics: Array, Binary Search, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums, return the length of the longest strictly increasing subsequence.
///
/// Link: https://leetcode.com/problems/longest-increasing-subsequence/

struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
