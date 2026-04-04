/// [70] Climbing Stairs
/// Difficulty: Easy
/// Topics: Math, Dynamic Programming, Memoization
/// Tags: Blind75, NeetCode150
///
/// You are climbing a staircase. It takes n steps to reach the top.
/// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
///
/// Link: https://leetcode.com/problems/climbing-stairs/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
