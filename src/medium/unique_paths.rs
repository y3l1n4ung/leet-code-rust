/// [62] Unique Paths
/// Difficulty: Medium
/// Topics: Math, Dynamic Programming, Combinatorics
/// Tags: Blind75, NeetCode150
///
/// There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.
/// Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.
///
/// Link: https://leetcode.com/problems/unique-paths/

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
