/// [134] Gas Station
/// Difficulty: Medium
/// Topics: Array, Greedy
/// Tags: NeetCode150
///
/// There are n gas stations along a circular route, where the amount of gas at the ith station is gas[i].
/// You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from the ith station to its next (i + 1)th station. You begin the journey with an empty tank at one of the gas stations.
/// Given two integer arrays gas and cost, return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1. If there exists a solution, it is guaranteed to be unique.
///
/// Link: https://leetcode.com/problems/gas-station/

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let gas = vec![1,2,3,4,5];
        let cost = vec![3,4,5,1,2];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
    }

    #[test]
    fn test_2() {
        let gas = vec![2,3,4];
        let cost = vec![3,4,3];
        assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
    }

    #[test]
    fn test_3() {
        let gas = vec![5,1,2,3,4];
        let cost = vec![4,4,1,5,1];
        assert_eq!(Solution::can_complete_circuit(gas, cost), 4);
    }
}
