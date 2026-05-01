/// [621] Task Scheduler
/// Difficulty: Medium
/// Topics: Array, Hash Table, Greedy, Sorting, Heap (Priority Queue), Counting
/// Tags: NeetCode150
///
/// You are given an array of CPU tasks, each represented by letters A to Z, and a cooling time n. Each cycle or interval allows the completion of one task. Tasks can be completed in any order, but there's a constraint: identical tasks must be separated by at least n intervals due to cooling time.
/// Return the minimum number of intervals required to complete all tasks.
///
/// Link: https://leetcode.com/problems/task-scheduler/

struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
            8
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1),
            6
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
            6
        );
    }
}
