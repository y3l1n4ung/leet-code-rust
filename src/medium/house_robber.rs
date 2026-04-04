/// [198] House Robber
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
/// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.
///
/// Link: https://leetcode.com/problems/house-robber/

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::rob(vec![2,7,9,3,1]), 12);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::rob(vec![1,1]), 1);
    }
}
