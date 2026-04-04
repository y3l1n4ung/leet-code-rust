/// [746] Min Cost Climbing Stairs
/// Difficulty: Easy
/// Topics: Array, Dynamic Programming
/// Tags: NeetCode150
///
/// You are given an integer array cost where cost[i] is the cost of ith step on a staircase. Once you pay the cost, you can either climb one or two steps.
/// You can either start from the step with index 0, or the step with index 1.
/// Return the minimum cost to reach the top of the floor.
///
/// Link: https://leetcode.com/problems/min-cost-climbing-stairs/

struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![0, 0, 1, 1]), 1);
    }
}
