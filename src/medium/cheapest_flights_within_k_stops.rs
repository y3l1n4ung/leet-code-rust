/// [787] Cheapest Flights Within K Stops
/// Difficulty: Medium
/// Topics: Dynamic Programming, Depth-First Search, Breadth-First Search, Graph, Heap (Priority Queue), Shortest Path
/// Tags: NeetCode150
///
/// There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.
/// You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.
///
/// Link: https://leetcode.com/problems/cheapest-flights-within-k-stops/

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let flights = vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]];
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 1), 200);
    }

    #[test]
    fn test_2() {
        let flights = vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]];
        assert_eq!(Solution::find_cheapest_price(3, flights, 0, 2, 0), 500);
    }

    #[test]
    fn test_3() {
        let flights = vec![vec![0,1,100]];
        assert_eq!(Solution::find_cheapest_price(2, flights, 0, 1, 1), 100);
    }
}
