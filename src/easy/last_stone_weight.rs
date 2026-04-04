/// [1046] Last Stone Weight
/// Difficulty: Easy
/// Topics: Array, Heap (Priority Queue)
/// Tags: NeetCode150
///
/// You are given an array of integers stones where stones[i] is the weight of the ith stone.
/// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together.
/// Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
/// - If x == y, both stones are destroyed.
/// - If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
/// At the end of the game, there is at most one stone left. Return the weight of the last remaining stone or 0 if there are no stones left.
///
/// Link: https://leetcode.com/problems/last-stone-weight/

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::last_stone_weight(vec![2, 2]), 0);
    }
}
