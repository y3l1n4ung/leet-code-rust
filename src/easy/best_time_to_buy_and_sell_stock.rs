/// [121] Best Time to Buy and Sell Stock
/// Difficulty: Easy
/// Topics: Array, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
/// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
///
/// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
    }
}
