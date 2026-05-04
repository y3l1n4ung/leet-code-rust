//! # 643. Maximum Average Subarray I
//!
//! **Difficulty:** Easy
//!
//! ## Problem Description
//! You are given an integer array `nums` consisting of `n` elements, and an integer `k`.
//! Find a contiguous subarray whose length is equal to `k` that has the maximum average value
//! and return this value. Any answer with a calculation error less than 10^-5 will be accepted.
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `nums = [1,12,-5,-6,50,3], k = 4`
//! - **Output:** `12.75000`
//! - **Explanation:** Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
//!
//! ### Example 2:
//! - **Input:** `nums = [5], k = 1`
//! - **Output:** `5.00000`
//!
//! ## Constraints:
//! - `n == nums.length`
//! - `1 <= k <= n <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        // 1. Calulate the initial sum of the first window
        let mut current_sum: i64 = nums[..k].iter().map(|&x| x as i64).sum();
        let mut max_sum = current_sum;

        // 2. Slide the widow: add right, remove left
        for i in k..nums.len() {
            current_sum += nums[i] as i64 - nums[i - k] as i64;
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
        // 3. Divide by k
        max_sum as f64 / k as f64
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let result = Solution::find_max_average(nums, k);
        assert!((result - 12.75).abs() < 1e-5);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![5];
        let k = 1;
        let result = Solution::find_max_average(nums, k);
        assert!((result - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_all_negative() {
        let nums = vec![-1, -12, -5, -6, -50, -3];
        let k = 2;
        // Max average should be (-1 + -12)/2 = -6.5 or (-5 + -6)/2 = -5.5
        let result = Solution::find_max_average(nums, k);
        assert!((result - (-5.5)).abs() < 1e-5);
    }

    #[test]
    fn test_k_equals_n() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let result = Solution::find_max_average(nums, k);
        assert!((result - 2.0).abs() < 1e-5);
    }

    #[test]
    fn test_large_values() {
        let nums = vec![10000, 10000, 10000, 10000];
        let k = 2;
        let result = Solution::find_max_average(nums, k);
        assert!((result - 10000.0).abs() < 1e-5);
    }
}
