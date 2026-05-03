//! # 283. Move Zeroes
//!
//! **Difficulty:** Easy
//!
//! ## Problem Description
//! Given an integer array `nums`, move all `0`'s to the end of it while maintaining
//! the relative order of the non-zero elements.
//!
//! **Note** that you must do this in-place without making a copy of the array.
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `nums = [0,1,0,3,12]`
//! - **Output:** `[1,3,12,0,0]`
//!
//! ### Example 2:
//! - **Input:** `nums = [0]`
//! - **Output:** `[0]`
//!
//! ## Constraints:
//! - `1 <= nums.length <= 10^4`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//!
//! ## Follow up:
//! - Could you minimize the total number of operations done?

struct Solution;
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;

        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }

    #[test]
    fn test_no_zeroes() {
        let mut nums = vec![1, 2, 3];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_all_zeroes() {
        let mut nums = vec![0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }
    #[test]
    fn test_case_8() {
        let mut nums = vec![1, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
    #[test]
    fn test_two_zeroes() {
        let mut nums = vec![0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0, 0]);
    }
    #[test]
    fn test_with_two() {
        let mut nums = vec![1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0]);
    }

    #[test]
    fn test_large_constraints_mixed() {
        let mut nums = vec![4, 2, 4, 0, 0, 3, 0, 5, 1, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![4, 2, 4, 3, 5, 1, 0, 0, 0, 0]);
    }
}
// 0 1
// [, 0, ]
