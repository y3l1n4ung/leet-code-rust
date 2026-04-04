/// [494] Target Sum
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming, Backtracking
/// Tags: NeetCode150
///
/// You are given an integer array nums and an integer target.
/// You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
/// Return the number of different expressions that you can build, which evaluates to target.
///
/// Link: https://leetcode.com/problems/target-sum/

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1,1,1,1,1], 3), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_target_sum_ways(vec![1,0], 1), 2);
    }
}
