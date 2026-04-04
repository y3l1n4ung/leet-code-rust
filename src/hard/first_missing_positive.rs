/// [41] First Missing Positive
/// Difficulty: Hard
/// Topics: Array, Hash Table
/// Tags: NeetCode150
///
/// Given an unsorted integer array nums, return the smallest missing positive integer.
/// You must implement a solution that runs in O(n) time and uses O(1) auxiliary space.
///
/// Link: https://leetcode.com/problems/first-missing-positive/

struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
