//! # 1431. Kids With the Greatest Number of Candies
//!
//! Given the array `candies` and the integer `extra_candies`, where `candies[i]`
//! represents the number of candies that the ith kid has.
//!
//! For each kid check if there is a way to distribute `extra_candies` among the kids
//! such that he or she can have the greatest number of candies among all of them.
//! Multiple kids can have the greatest number of candies.

struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = *candies.iter().max().unwrap();
        let mut result: Vec<bool> = Vec::with_capacity(candies.len());

        for c in candies {
            if c + extra_candies >= max {
                result.push(true);
            }else {
                result.push(false);
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra = 3;
        let expected = vec![true, true, true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, extra), expected);
    }

    #[test]
    fn test_all_same() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra = 1;
        let expected = vec![true, false, false, false, false];
        assert_eq!(Solution::kids_with_candies(candies, extra), expected);
    }

    #[test]
    fn test_large_extra() {
        let candies = vec![12, 1, 12];
        let extra = 10;
        let expected = vec![true, false, true];
        assert_eq!(Solution::kids_with_candies(candies, extra), expected);
    }
}
