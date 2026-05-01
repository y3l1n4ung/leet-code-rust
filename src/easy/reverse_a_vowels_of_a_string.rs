//! # 345. Reverse Vowels of a String
//!
//! **Difficulty:** Easy
//!
//! ## Problem Description
//! Given a string `s`, reverse only all the vowels in the string and return it.
//! The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both
//! lower and upper cases, more than once.
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `s = "IceCreAm"`
//! - **Output:** `"AceCreIm"`
//! - **Explanation:** The vowels are ['I', 'e', 'e', 'A']. Reversing them results in "AceCreIm".
//!
//! ### Example 2:
//! - **Input:** `s = "leetcode"`
//! - **Output:** `"leotcede"`
//!
//! ## Constraints:
//! - `1 <= s.length <= 3 * 10^5`
//! - `s` consists of printable ASCII characters.

struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut left = 0;
        let mut right = bytes.len().saturating_sub(1);

        let is_vowel = |b: u8| {
            matches!(
                b,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        };

        while right > left {
            if !is_vowel(bytes[left]) {
                left += 1;
            } else if !is_vowel(bytes[right]) {
                right -= 1;
            } else {
                bytes.swap(left, right);
                left += 1;
                right -= 1;
            }
        }
        String::from_utf8(bytes).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::reverse_vowels("IceCreAm".to_string()),
            "AceCreIm".to_string()
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_string()),
            "leotcede".to_string()
        );
    }

    #[test]
    fn test_no_vowels() {
        // Case: No vowels present
        assert_eq!(
            Solution::reverse_vowels("rhythm".to_string()),
            "rhythm".to_string()
        );
    }

    #[test]
    fn test_max_constraints_all_vowels() {
        // Case: All vowels, full reversal
        assert_eq!(
            Solution::reverse_vowels("aeiouAEIOU".to_string()),
            "UOIEAuoiea".to_string()
        );
    }
}
