# Climbing Stairs - Thinking 🦀

## Problem Statement
You are climbing a staircase. It takes `n` steps to reach the top.
Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

## Theoretical Explanation
This is a classic Dynamic Programming (DP) problem that can be reduced to the Fibonacci sequence.

### Key Logic
- To reach step `n`, you can either come from step `n-1` (by taking 1 step) or step `n-2` (by taking 2 steps).
- Let `dp[i]` be the number of ways to reach step `i`.
- `dp[i] = dp[i-1] + dp[i-2]`.

### Base Cases
- `dp[1] = 1`
- `dp[2] = 2`

### Complexity
- **Time**: O(n).
- **Space**: O(1), if we only store the last two steps.

## Implementation Pattern
We can use a simple iterative approach with two variables to optimize space from O(n) to O(1).
