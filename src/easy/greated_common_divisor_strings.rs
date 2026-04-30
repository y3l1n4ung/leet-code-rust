//! # 1071. Greatest Common Divisor of Strings
//!
//! For two strings `s` and `t`, we say "t divides s" if and only if
//! `s = t + t + t + ... + t` (i.e., `t` is concatenated with itself one or more times).
//!
//! Given two strings `str1` and `str2`, return the largest string `x` such that
//! `x` divides both `str1` and `str2`.
//!
//! Example 1:
//! Input: str1 = "ABCABC", str2 = "ABC"
//! Output: "ABC"
//!
//! Example 2:
//! Input: str1 = "ABABAB", str2 = "ABAB"
//! Output: "AB"

struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return String::new();
        }
        fn gcd(a: usize, b: usize) -> usize {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        let gcd_len= gcd(str1.len(), str2.len());

        str1[..gcd_len].to_string()
    }
}

// --- Testing Suite ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_divisor_exists() {
        assert_eq!(
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string()),
            "ABC".to_string()
        );
        assert_eq!(
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string()),
            "AB".to_string()
        );
    }

    #[test]
    fn test_no_common_divisor() {
        assert_eq!(
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn test_repeated_patterns() {
        assert_eq!(
            Solution::gcd_of_strings("AAAAAA".to_string(), "AAA".to_string()),
            "AAA".to_string()
        );
    }
}
