/// [42] Trapping Rain Water
/// Difficulty: Hard
/// Topics: Array, Two Pointers, Dynamic Programming, Stack, Monotonic Stack
/// Tags: Blind75, NeetCode150
///
/// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
///
/// Link: https://leetcode.com/problems/trapping-rain-water/

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::trap(vec![1, 1, 1]), 0);
    }
}
