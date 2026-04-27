//! # How Many Numbers Are Smaller Than the Current Number
//!
//! Given the array `nums`, for each `nums[i]` find out how many numbers in the array are smaller than it.
//! That is, for each `nums[i]` you have to count the number of valid `j's` such that `j != i` and `nums[j] < nums[i]`.
//!
//! Example 1:
//! Input: nums = [8,1,2,2,3]
//! Output: [4,0,1,1,3]
struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            let mut count: i32 = 0;

            for j in 0..nums.len() {
                let current_number = nums[i];
                let other_number = nums[j];

                if current_number != other_number && other_number < current_number {
                    count += 1;
                }
            }
            result.push(count);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![8, 1, 2, 2, 3];
        let result = Solution::smaller_numbers_than_current(nums);
        assert_eq!(result, vec![4, 0, 1, 1, 3]);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![6, 5, 4, 8];
        let result = Solution::smaller_numbers_than_current(nums);
        assert_eq!(result, vec![2, 1, 0, 3]);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![7, 7, 7, 7];
        let result = Solution::smaller_numbers_than_current(nums);
        assert_eq!(result, vec![0, 0, 0, 0]);
    }
}
