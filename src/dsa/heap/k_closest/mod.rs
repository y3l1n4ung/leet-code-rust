// K Closest Points to Origin Practice 🦀

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Point {
    dist: i32,
    coords: Vec<i32>,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want a Max-Heap based on distance to find the K closest
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    /// Practice: K Closest Points to Origin
    /// Returns the k closest points to (0,0).
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::with_capacity(k as usize);

        for p in points {
            let dist = p[0] * p[0] + p[1] * p[1];
            let point = Point { dist, coords: p };

            if heap.len() < k as usize {
                heap.push(point);
            } else if let Some(top) = heap.peek() {
                if point.dist < top.dist {
                    heap.pop();
                    heap.push(point);
                }
            }
        }

        heap.into_iter().map(|p| p.coords).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_closest() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let mut result = Solution::k_closest(points, 1);
        result.sort();
        assert_eq!(result, vec![vec![-2, 2]]);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let mut result = Solution::k_closest(points, 2);
        result.sort();
        let mut expected = vec![vec![3, 3], vec![-2, 4]];
        expected.sort();
        assert_eq!(result, expected);
    }
}
