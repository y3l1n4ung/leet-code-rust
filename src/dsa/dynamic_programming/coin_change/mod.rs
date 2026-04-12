// Coin Change Practice 🦀

pub struct Solution;

impl Solution {
    /// Practice: Coin Change
    /// Returns the minimum number of coins to reach amount.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![amount + 1; amt + 1];
        dp[0] = 0;

        for i in 1..=amt {
            for &coin in &coins {
                let c = coin as usize;
                if i >= c {
                    dp[i] = std::cmp::min(dp[i], 1 + dp[i - c]);
                }
            }
        }

        if dp[amt] > amount { -1 } else { dp[amt] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }
}
