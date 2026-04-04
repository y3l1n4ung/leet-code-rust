/// [4] Median of Two Sorted Arrays
/// Difficulty: Hard
/// Topics: Array, Binary Search, Divide and Conquer
/// Tags: NeetCode150
///
/// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
/// The overall run time complexity should be O(log (m+n)).
///
/// Link: https://leetcode.com/problems/median-of-two-sorted-arrays/


struct Solution;

impl Solution {
    
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
    }
}
