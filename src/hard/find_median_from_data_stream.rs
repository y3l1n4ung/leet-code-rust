/// [295] Find Median from Data Stream
/// Difficulty: Hard
/// Topics: Two Pointers, Design, Sorting, Heap (Priority Queue), Data Stream
/// Tags: Blind75, NeetCode150
///
/// The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.
/// For example, for arr = [2,3,4], the median is 3.
/// For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
/// Implement the MedianFinder class:
/// - MedianFinder() initializes the MedianFinder object.
/// - void addNum(int num) adds the integer num from the data stream to the data structure.
/// - double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
///
/// Link: https://leetcode.com/problems/find-median-from-data-stream/

struct MedianFinder {
    todo: ()
}

impl MedianFinder {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn add_num(&self, num: i32) {
        todo!()
    }
    
    pub fn find_median(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }
}
