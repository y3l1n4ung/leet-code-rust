/// [33] Search in Rotated Sorted Array
/// Difficulty: Medium
/// Topics: Array, Binary Search
/// Tags: Blind75, NeetCode150
///
/// There is an integer array nums sorted in ascending order (with distinct values).
/// Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].
/// Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Link: https://leetcode.com/problems/search-in-rotated-sorted-array/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
