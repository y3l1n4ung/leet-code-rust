/// [11] Container With Most Water
/// Difficulty: Medium
/// Topics: Array, Two Pointers, Greedy
/// Tags: Blind75, NeetCode150
///
/// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
/// Find two lines that together with the x-axis form a container, such that the container contains the most water.
/// Return the maximum amount of water a container can store.
/// Notice that you may not slant the container.
///
/// Link: https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
    }
}
