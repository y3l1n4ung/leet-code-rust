/// [2013] Detect Squares
/// Difficulty: Medium
/// Topics: Array, Hash Table, Design, Geometry
/// Tags: NeetCode150
///
/// You are given a stream of points on a 2D plane. Design an algorithm that:
/// - Adds new points from the stream into a data structure. Duplicate points are allowed and should be treated as different points.
/// - Given a query point, counts the number of ways to choose three points from the data structure such that the three points and the query point form an axis-aligned square with a positive area.
///
/// Link: https://leetcode.com/problems/detect-squares/

struct DetectSquares {
    todo: (),
}

impl DetectSquares {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&self, point: Vec<i32>) {
        todo!()
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let ds = DetectSquares::new();
        ds.add(vec![3, 10]);
        ds.add(vec![11, 2]);
        ds.add(vec![3, 2]);
        assert_eq!(ds.count(vec![11, 10]), 1);
        assert_eq!(ds.count(vec![14, 8]), 0);
        ds.add(vec![11, 2]);
        assert_eq!(ds.count(vec![11, 10]), 2);
    }
}
