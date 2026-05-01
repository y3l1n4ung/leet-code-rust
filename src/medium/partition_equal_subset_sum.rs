/// [416] Partition Equal Subset Sum
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming
/// Tags: NeetCode150
///
/// Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
///
/// Link: https://leetcode.com/problems/partition-equal-subset-sum/

struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_partition(vec![1, 1]), true);
    }
}
