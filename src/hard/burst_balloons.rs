/// [312] Burst Balloons
/// Difficulty: Hard
/// Topics: Array, Dynamic Programming
/// Tags: NeetCode150
///
/// You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.
/// If you burst the ith balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. After the burst, the (i - 1)th and (i + 1)th balloons become adjacent.
/// Find the maximum coins you can collect by bursting the balloons wisely.
///
/// Link: https://leetcode.com/problems/burst-balloons/

struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_coins(vec![3,1,5,8]), 167);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_coins(vec![1,5]), 10);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_coins(vec![]), 0);
    }
}
