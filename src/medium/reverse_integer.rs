/// [7] Reverse Integer
/// Difficulty: Medium
/// Topics: Math
/// Tags: NeetCode150
///
/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
///
/// Link: https://leetcode.com/problems/reverse-integer/

struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            let r = x % 10; // reminder
            x /= 10;
            // rev = (rev * 10) + r
            match rev.checked_mul(10).and_then(|f| f.checked_add(r)) {
                Some(result) => rev = result,
                None => return 0,
            }
        }
        rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
        /*
        x = 123
        result =0

        while x != 0{
            p = 123 % 10
            result = result * 10 + p

        }
        */
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse(123456789), 987654321);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::reverse(2147483647), 0);
    }
}
