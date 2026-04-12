# Coin Change - Thinking 🦀

## Problem Statement
Given an array of integers `coins` representing coins of different denominations and an integer `amount`, return the fewest number of coins that you need to make up that amount.

### Example
`coins = [1, 2, 5], amount = 11` -> `11 = 5 + 5 + 1`, result: 3.

## Theoretical Explanation
This is a classic "Unbounded Knapsack" problem, solvable via DP.

### Key Logic
- Let `dp[i]` be the minimum coins required to make up amount `i`.
- Initialize `dp[0] = 0` and all other `dp[i] = amount + 1` (a sentinel for infinity).
- For each amount from 1 to `amount`:
  - For each `coin` in `coins`:
    - If `i - coin >= 0`:
      - `dp[i] = min(dp[i], 1 + dp[i - coin])`.

### Complexity
- **Time**: O(amount * len(coins)).
- **Space**: O(amount), to store the DP table.
