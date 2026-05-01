// Find Median from Data Stream Practice 🦀

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    small: BinaryHeap<i32>,          // Max-Heap
    large: BinaryHeap<Reverse<i32>>, // Min-Heap
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        // Step 1: Push to max-heap
        self.small.push(num);

        // Step 2: Move from max-heap to min-heap to keep order
        if let Some(val) = self.small.pop() {
            self.large.push(Reverse(val));
        }

        // Step 3: Rebalance size
        if self.large.len() > self.small.len() {
            if let Some(Reverse(val)) = self.large.pop() {
                self.small.push(val);
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            let small_val = *self.small.peek().unwrap() as f64;
            let Reverse(large_val) = *self.large.peek().unwrap();
            (small_val + large_val as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_finder() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }
}
