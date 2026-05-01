/// [846] Hand of Straights
/// Difficulty: Medium
/// Topics: Array, Hash Table, Greedy, Sorting
/// Tags: NeetCode150
///
/// Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size groupSize, and consists of groupSize consecutive cards.
/// Given an integer array hand where hand[i] is the value written on the ith card and an integer groupSize, return true if she can rearrange the cards, or false otherwise.
///
/// Link: https://leetcode.com/problems/hand-of-straights/

struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3),
            true
        );
    }
}
