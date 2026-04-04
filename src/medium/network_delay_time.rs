/// [743] Network Delay Time
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Graph, Heap (Priority Queue), Shortest Path
/// Tags: NeetCode150
///
/// You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
/// We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.
///
/// Link: https://leetcode.com/problems/network-delay-time/

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let times = vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]];
        assert_eq!(Solution::network_delay_time(times, 4, 2), 2);
    }

    #[test]
    fn test_2() {
        let times = vec![vec![1,2,1]];
        assert_eq!(Solution::network_delay_time(times, 2, 1), 1);
    }

    #[test]
    fn test_3() {
        let times = vec![vec![1,2,1]];
        assert_eq!(Solution::network_delay_time(times, 2, 2), -1);
    }
}
