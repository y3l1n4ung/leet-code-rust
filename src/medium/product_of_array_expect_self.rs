/// [238] Product of Array Except Self
/// Difficulty: Medium
/// Topics: Array, Prefix Sum
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
/// You must write an algorithm that runs in O(n) time and without using the division operation.
///
/// Link: https://leetcode.com/problems/product-of-array-except-self/


struct Solution;

impl Solution {
    
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::product_except_self(vec![1, 1]), vec![1, 1]);
    }
}
