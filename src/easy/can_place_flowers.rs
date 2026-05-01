//! # 605. Can Place Flowers
//!
//! **Difficulty:** Easy
//! **Topic:** Array, Greedy
//!
//! ## Problem Description
//! You have a long flowerbed in which some of the plots are planted, and some are not.
//! However, flowers cannot be planted in **adjacent** plots.
//!
//! Given an integer array `flowerbed` containing `0`'s and `1`'s, where `0` means empty
//! and `1` means not empty, and an integer `n`, return `true` if `n` new flowers can be
//! planted in the `flowerbed` without violating the no-adjacent-flowers rule and `false` otherwise.
//!
//! ## Examples
//!
//! ### Example 1:
//! - **Input:** `flowerbed = [1,0,0,0,1]`, `n = 1`
//! - **Output:** `true`
//!
//! ### Example 2:
//! - **Input:** `flowerbed = [1,0,0,0,1]`, `n = 2`
//! - **Output:** `false`
//!
//! ## Constraints:
//! - `1 <= flowerbed.length <= 2 * 10^4`
//! - `flowerbed[i]` is `0` or `1`.
//! - There are no two adjacent flowers in `flowerbed`.
//! - `0 <= n <= flowerbed.length`
//!

struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if n <= 0 {
                return true;
            }
            let prev_empty = i == 0 || flowerbed[i - 1] == 0;
            let next_empty = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;

            if prev_empty && flowerbed[i] == 0 && next_empty {
                flowerbed[i] = 1;
                n -= 1;
            }
        }
        n <= 0
    }
}

// --- Testing Suite ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Input: [1,0,0,0,1], n = 1 -> Output: true
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    }

    #[test]
    fn test_example_2() {
        // Input: [1,0,0,0,1], n = 2 -> Output: false
        assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    }

    #[test]
    fn test_example_3() {
        // Input: [1,0,0,0,0,1], n = 2 -> Output: false
        assert_eq!(
            Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
            false
        );
    }

    #[test]
    fn test_example_4() {
        // Input: [0,0,0,1], n = 1 -> Output: false
        assert_eq!(Solution::can_place_flowers(vec![0, 0, 0, 1], 1), true);
    }

    #[test]
    fn test_edge_case_single_zero() {
        // Input: [0], n = 1 -> Output: true
        assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
    }

    #[test]
    fn test_consecutive_zeros() {
        // Input: [0,0,0], n = 2 -> Output: true [1,0,1]
        assert_eq!(Solution::can_place_flowers(vec![0, 0, 0], 2), true);
    }
}
