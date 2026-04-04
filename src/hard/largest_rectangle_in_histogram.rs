/// [84] Largest Rectangle in Histogram
/// Difficulty: Hard
/// Topics: Array, Stack, Monotonic Stack
/// Tags: NeetCode150
///
/// Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.
///
/// Link: https://leetcode.com/problems/largest-rectangle-in-histogram/

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_rectangle_area(vec![1, 1, 1, 1]), 4);
    }
}
