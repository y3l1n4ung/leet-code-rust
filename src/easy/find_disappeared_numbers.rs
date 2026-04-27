//! # Find All Numbers Disappeared in an Array
//!
//! Given an array `nums` of `n` integers where `nums[i]` is in the range `[1, n]`,
//! return an array of all the integers in the range `[1, n]` that do not appear in `nums`.
//!
//! Example 1:
//! Input: nums = [4,3,2,7,8,2,3,1]
//! Output: [5,6]
//!
//! Example 2:
//! Input: nums = [1,1]
//! Output: [2]

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut slot = vec![0; n + 1];
        for &num in &nums {
            slot[num as usize] += 1;
        }
        slot.into_iter()
            .enumerate()
            .skip(1)
            .filter(|&(_, count)| count == 0)
            .map(|(index, _)| index as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let mut result = Solution::find_disappeared_numbers(nums);
        result.sort(); // Order doesn't matter, but sorting helps assertion
        assert_eq!(result, vec![5, 6]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 1];
        let result = Solution::find_disappeared_numbers(nums);
        assert_eq!(result, vec![2]);
    }
}
