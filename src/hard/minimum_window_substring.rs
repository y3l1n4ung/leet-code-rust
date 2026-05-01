/// [76] Minimum Window Substring
/// Difficulty: Hard
/// Topics: Hash Table, String, Sliding Window
/// Tags: Blind75, NeetCode150
///
/// Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".
///
/// Link: https://leetcode.com/problems/minimum-window-substring/

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
    }
}
