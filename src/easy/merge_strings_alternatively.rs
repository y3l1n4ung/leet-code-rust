//! # 1768. Merge Strings Alternately
//! 
//! You are given two strings `word1` and `word2`. 
//! Merge the strings by adding letters in alternating order, starting with `word1`. 
//! If a string is longer than the other, append the additional letters onto the end of the merged string.
//!
//! Example 1:
//! Input: word1 = "abc", word2 = "pqr"
//! Output: "apbqcr"
//!
//! Example 2:
//! Input: word1 = "ab", word2 = "pqrs"
//! Output: "apbqrs"

use std::ops::Add;

struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {

        let mut result = String::with_capacity(word1.len() + word2.len());


        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();
        loop {
            let char1 = chars1.next();
            let char2 = chars2.next();

            if char1.is_none() && char2.is_none() {
                break;
            }
            if let Some(ch) =char1{

                result.push(ch);
            } 
            if let Some(ch) =char2  {
                result.push(ch);
            }
        }
        result
    }
}

// --- Testing ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_length() {
        let w1 = String::from("abc");
        let w2 = String::from("pqr");
        assert_eq!(Solution::merge_alternately(w1, w2), "apbqcr");
    }

    #[test]
    fn test_word1_shorter() {
        let w1 = String::from("ab");
        let w2 = String::from("pqrs");
        assert_eq!(Solution::merge_alternately(w1, w2), "apbqrs");
    }

    #[test]
    fn test_word2_shorter() {
        let w1 = String::from("abcd");
        let w2 = String::from("pq");
        assert_eq!(Solution::merge_alternately(w1, w2), "apbqcd");
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(
            Solution::merge_alternately(String::from(""), String::from("abc")),
            "abc"
        );
        assert_eq!(
            Solution::merge_alternately(String::from("xyz"), String::from("")),
            "xyz"
        );
    }
}