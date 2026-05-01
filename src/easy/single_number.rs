/// [136] Single Number
/// Difficulty: Easy
/// Topics: Array, Bit Manipulation
/// Tags: NeetCode150
///
/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
///
/// Link: https://leetcode.com/problems/single-number/

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // In [14]: 2 ^ 2 ^ 1
        // Out[14]: 1

        // In [15]: 2 ^ 2 ^ 1 ^ 2
        // Out[15]: 3

        // In [16]: 2 ^ 2 ^ 1 ^ 4 ^ 4
        // Out[16]: 1
        let mut result = 0;
        for i in nums {
            result ^= i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
