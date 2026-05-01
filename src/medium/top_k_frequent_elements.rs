/// [347] Top K Frequent Elements
/// Difficulty: Medium
/// Topics: Array, Hash Table, Divide and Conquer, Sorting, Heap (Priority Queue), Bucket Sort, Counting, Quickselect
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
///
/// Link: https://leetcode.com/problems/top-k-frequent-elements/

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_3() {
        let mut result = Solution::top_k_frequent(vec![1, 2], 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}
