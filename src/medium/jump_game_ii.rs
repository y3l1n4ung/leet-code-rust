/// [45] Jump Game II
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming, Greedy
/// Tags: NeetCode150
///
/// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
/// Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:
/// - 0 <= j <= nums[i] and
/// - i + j < n
/// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach the last index.
///
/// Link: https://leetcode.com/problems/jump-game-ii/

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::jump(vec![0]), 0);
    }
}
