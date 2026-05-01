/// [215] Kth Largest Element in an Array
/// Difficulty: Medium
/// Topics: Array, Divide and Conquer, Sorting, Heap (Priority Queue), Quickselect
/// Tags: NeetCode150
///
/// Given an integer array nums and an integer k, return the kth largest element in the array.
/// Note that it is the kth largest element in the sorted order, not the kth distinct element.
/// You must solve it in O(n) time complexity.
///
/// Link: https://leetcode.com/problems/kth-largest-element-in-an-array/

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_kth_largest(vec![1], 1), 1);
    }
}
