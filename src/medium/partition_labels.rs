/// [763] Partition Labels
/// Difficulty: Medium
/// Topics: Hash Table, Two Pointers, String, Greedy
/// Tags: NeetCode150
///
/// You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.
/// Return a list of integers representing the size of these parts.
///
/// Link: https://leetcode.com/problems/partition-labels/

struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::partition_labels("ababcbacadefegdehijhklij".to_string()),
            vec![9, 7, 8]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::partition_labels("eccbbbbdec".to_string()),
            vec![10]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::partition_labels("a".to_string()), vec![1]);
    }
}
