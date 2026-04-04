/// [518] Coin Change II
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming
/// Tags: NeetCode150
///
/// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
/// Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.
/// You may assume that you have an infinite number of each kind of coin.
///
/// Link: https://leetcode.com/problems/coin-change-ii/

struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::change(3, vec![2]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
}
