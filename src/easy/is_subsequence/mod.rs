//! # 392. Is Subsequence
//!
//! **Difficulty:** Easy
//!
//! ## Problem Description
//! Given two strings `s` and `t`, return `true` if `s` is a **subsequence** of `t`, 
//! or `false` otherwise.
//!
//! A **subsequence** of a string is a new string that is formed from the original 
//! string by deleting some (can be none) of the characters without disturbing the 
//! relative positions of the remaining characters. (i.e., `"ace"` is a subsequence 
//! of `"abcde"` while `"aec"` is not).
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `s = "abc"`, `t = "ahbgdc"`
//! - **Output:** `true`
//!
//! ### Example 2:
//! - **Input:** `s = "axc"`, `t = "ahbgdc"`
//! - **Output:** `false`
//!
//! ## Constraints:
//! - `0 <= s.length <= 100`
//! - `0 <= t.length <= 10^4`
//! - `s` and `t` consist only of lowercase English letters.
//!
//! ## Follow up:
//! Suppose there are lots of incoming `s`, say `s1, s2, ..., sk` where `k >= 10^9`, 
//! and you want to check one by one to see if `t` has its subsequence. In this 
//! scenario, how would you change your code?

struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty(){
            return true;
        }
        let s_byte:Vec<u8>= s.bytes().collect();
        let t_byte:Vec<u8> = t.bytes().collect();
        let mut chars:Vec<char> = Vec::new();
        let (s_len,t_len) = (s.len(),t.len());
        let (mut l_cur, mut r_cur) = (0,0);

        while l_cur < s_len && r_cur < t_len {
           if s_byte[l_cur] == t_byte[r_cur]{
                chars.push(s_byte[l_cur] as char);
                l_cur +=1;
           }
           r_cur+=1;
            
        }
        
        chars.iter().collect::<String>()  == s

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn test_empty_s() {
        // An empty string is always a subsequence of any string
        assert_eq!(
            Solution::is_subsequence("".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn test_both_empty() {
        assert_eq!(
            Solution::is_subsequence("".to_string(), "".to_string()),
            true
        );
    }

    #[test]
    fn test_large_t_false() {
        // s is not in t even if t is long
        let t = "a".repeat(10000);
        assert_eq!(
            Solution::is_subsequence("b".to_string(), t),
            false
        );
    }
}