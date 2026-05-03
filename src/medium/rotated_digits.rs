
//! # 788. Rotated Digits
//!
//! **Difficulty:** Medium
//!
//! ## Problem Description
//! An integer `x` is **good** if after rotating each digit individually by 180 degrees, 
//! we get a valid number that is different from `x`. Each digit must be rotated.
//!
//! A number is valid if each digit remains a digit after rotation. 
//! - `0`, `1`, and `8` rotate to themselves.
//! - `2` and `5` rotate to each other.
//! - `6` and `9` rotate to each other.
//! - The rest of the numbers (`3`, `4`, `7`) do not rotate to any other number and become invalid.
//!
//! Given an integer `n`, return the number of **good** integers in the range `[1, n]`.
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `n = 10`
//! - **Output:** `4`
//! - **Explanation:** There are four good numbers in the range [1, 10]: 2, 5, 6, 9.
//!   Note that 1 and 10 are not good numbers, since they remain unchanged after rotating.
//!
//! ### Example 2:
//! - **Input:** `n = 1`
//! - **Output:** `0`
//!
//! ### Example 3:
//! - **Input:** `n = 2`
//! - **Output:** `1`
//!
//! ## Constraints:
//! - `1 <= n <= 10^4`

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
       
        let mut count =0;
        for i in 1..=n{
           let mut num =i;
           let mut has_rotating_digit = false;
           let mut is_invalid = false;

           while num > 0 {
               let digit = num % 10;
               match digit {
                   3 |4 |7  =>{
                    is_invalid =true;
                    break;
                   },
                   2 |5 |6 | 9 =>{
                    has_rotating_digit =true;
                   }
                _ => {}
               }
               num /=10;
           }

           if !is_invalid && has_rotating_digit{
            count +=1;
           }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::rotated_digits(10), 4);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::rotated_digits(1), 0);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::rotated_digits(2), 1);
    }

    #[test]
    fn test_invalid_digits() {
        // Numbers containing 3, 4, or 7 should never be good
        assert_eq!(Solution::rotated_digits(7), 3); // 2, 5, 6 are good, but 3,4,7 invalid
    }

    #[test]
    fn test_max_constraints() {
        // n = 10000
        assert_eq!(Solution::rotated_digits(10000), 2320);
    }
}