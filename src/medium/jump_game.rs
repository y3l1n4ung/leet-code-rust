/// [55] Jump Game
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming, Greedy
/// Tags: Blind75, NeetCode150
///
/// You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.
/// Return true if you can reach the last index, or false otherwise.
///
/// Link: https://leetcode.com/problems/jump-game/

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_jump(vec![2,3,1,1,4]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_jump(vec![3,2,1,0,4]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_jump(vec![0]), true);
    }
}
