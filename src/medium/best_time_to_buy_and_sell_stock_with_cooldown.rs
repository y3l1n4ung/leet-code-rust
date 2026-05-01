/// [309] Best Time to Buy and Sell Stock with Cooldown
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming
/// Tags: NeetCode150
///
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following constraints:
/// - After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
/// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
///
/// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![1, 2, 4]), 3);
    }
}
