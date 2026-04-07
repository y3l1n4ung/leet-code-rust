use std::collections::HashMap;

/// [1] Two Sum
/// Difficulty: Easy
/// Topics: Array, Hash Table
/// Tags: Blind75, NeetCode150
///
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
///
/// Link: https://leetcode.com/problems/two-sum/

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // [ 7 : 0 ]
        // [-3: 1]
        // [-6:2]

        // expected = current - target

        let mut has_map: HashMap<i32, i32> = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            let excepted = target - value;
            if has_map.contains_key(&excepted) {
                let left = *has_map.get(&excepted).unwrap();
                return vec![left as i32, index as i32];
            } else {
                has_map.insert(*value, index as i32);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 11, 15, 7], 9), vec![0, 3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }
    #[test]
    fn test_5() {
        assert_eq!(Solution::two_sum(vec![0, 4, 4, 0], 0), vec![0, 3]);
    }
}
