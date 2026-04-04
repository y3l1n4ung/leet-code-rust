/// [91] Decode Ways
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// A message containing letters from A-Z can be encoded into numbers using the following mapping:
/// 'A' -> "1", 'B' -> "2", ..., 'Z' -> "26"
/// To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
/// - "AAJF" with the grouping (1 1 10 6)
/// - "KJF" with the grouping (11 10 6)
/// Given a string s containing only digits, return the number of ways to decode it.
///
/// Link: https://leetcode.com/problems/decode-ways/

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_decodings("12".to_string()), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }
}
