/// [853] Car Fleet
/// Difficulty: Medium
/// Topics: Array, Stack, Sorting, Monotonic Stack
/// Tags: NeetCode150
///
/// There are n cars going to the same destination along a one-lane road. The destination is target miles away.
/// You are given two integer arrays position and speed, both of length n, where position[i] is the position of the ith car and speed[i] is the speed of the ith car (in miles per hour).
/// A car can never pass another car ahead of it, but it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).
/// A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.
/// If a car catches up to another car right at the destination point, it will still be considered as one car fleet.
/// Return the number of car fleets that will arrive at the destination.
///
/// Link: https://leetcode.com/problems/car-fleet/

struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);
    }
}
