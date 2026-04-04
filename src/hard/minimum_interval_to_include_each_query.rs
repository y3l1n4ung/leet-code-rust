/// [1851] Minimum Interval to Include Each Query
/// Difficulty: Hard
/// Topics: Array, Binary Search, Line Sweep, Sorting, Heap (Priority Queue)
/// Tags: NeetCode150
///
/// You are given a 2D integer array intervals, where intervals[i] = [lefti, righti] describes the ith interval starting at lefti and ending at righti (inclusive). The size of an interval is defined as the number of integers it contains, or more simply righti - lefti + 1.
/// You are also given an integer array queries. The answer to the jth query is the size of the smallest interval i such that lefti <= queries[j] <= righti. If no such interval exists, the answer is -1.
/// Return an array consisting of the answers to the queries.
///
/// Link: https://leetcode.com/problems/minimum-interval-to-include-each-query/

struct Solution;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1,4],vec![2,4],vec![3,6],vec![4,4]];
        let queries = vec![2,3,4,5];
        assert_eq!(Solution::min_interval(intervals, queries), vec![3,3,1,4]);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![2,3],vec![2,5],vec![1,8],vec![20,25]];
        let queries = vec![2,19,5,22];
        assert_eq!(Solution::min_interval(intervals, queries), vec![2,-1,4,6]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_interval(vec![], vec![1]), vec![-1]);
    }
}
