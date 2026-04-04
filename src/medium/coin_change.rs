/// [322] Coin Change
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming, Breadth-First Search
/// Tags: Blind75, NeetCode150
///
/// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
/// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
/// You may assume that you have an infinite number of each kind of coin.
///
/// Link: https://leetcode.com/problems/coin-change/

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
