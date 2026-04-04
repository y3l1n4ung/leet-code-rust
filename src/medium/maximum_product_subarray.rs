/// [152] Maximum Product Subarray
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums, find a subarray that has the largest product, and return the product.
///
/// Link: https://leetcode.com/problems/maximum-product-subarray/

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_product(vec![-2,0,-1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_product(vec![-2]), -2);
    }
}
