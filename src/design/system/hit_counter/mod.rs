/// [LeetCode 362] Design Hit Counter
/// Difficulty: Medium
/// Topics: Array, Hash Table, Binary Search, Design, Queue
///
/// Link: https://leetcode.com/problems/design-hit-counter/

pub struct HitCounter {
    hits: Vec<i32>,
}

impl HitCounter {
    pub fn new() -> Self {
        Self { hits: Vec::new() }
    }

    /// Record a hit.
    /// @param timestamp - The current timestamp (in seconds granularity).
    pub fn hit(&mut self, timestamp: i32) {
        todo!("Record the hit at the given timestamp")
    }

    /// Return the number of hits in the past 5 minutes.
    /// @param timestamp - The current timestamp (in seconds granularity).
    pub fn get_hits(&self, timestamp: i32) -> i32 {
        todo!("Count the number of hits in the [timestamp - 300, timestamp] window")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_counter_basic() {
        let mut counter = HitCounter::new();
        counter.hit(1);
        counter.hit(2);
        counter.hit(3);
        assert_eq!(counter.get_hits(4), 3);
        counter.hit(300);
        assert_eq!(counter.get_hits(300), 4);
        assert_eq!(counter.get_hits(301), 3);
    }
}
