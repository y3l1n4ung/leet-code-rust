/// [424] Longest Repeating Character Replacement
/// Difficulty: Medium
/// Topics: Hash Table, String, Sliding Window
/// Tags: Blind75, NeetCode150
///
/// You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.
/// Return the length of the longest substring containing the same letter you can get after performing the above operations.
///
/// Link: https://leetcode.com/problems/longest-repeating-character-replacement/

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::character_replacement("AAAA".to_string(), 2), 4);
    }
}
