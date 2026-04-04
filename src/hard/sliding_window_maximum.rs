/// [239] Sliding Window Maximum
/// Difficulty: Hard
/// Topics: Array, Queue, Sliding Window, Heap (Priority Queue), Monotonic Queue
/// Tags: NeetCode150
///
/// You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
/// Return the max sliding window.
///
/// Link: https://leetcode.com/problems/sliding-window-maximum/

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
    }
}
