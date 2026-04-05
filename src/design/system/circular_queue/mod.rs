/// [622] Design Circular Queue
/// Difficulty: Medium
/// Topics: Array, Linked List, Design, Queue
/// Tags: RustMastery
///
/// Design your implementation of the circular queue.
///
/// Link: https://leetcode.com/problems/design-circular-queue/

pub struct MyCircularQueue {
    // Add your internal state here
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        todo!("Initialize the queue with a fixed capacity k")
    }
    
    pub fn en_queue(&mut self, value: i32) -> bool {
        todo!("Insert the element, return true if successful")
    }
    
    pub fn de_queue(&mut self) -> bool {
        todo!("Delete an element, return true if successful")
    }
    
    pub fn front(&self) -> i32 {
        todo!("Return the front element, or -1 if empty")
    }
    
    pub fn rear(&self) -> i32 {
        todo!("Return the rear element, or -1 if empty")
    }
    
    pub fn is_empty(&self) -> bool {
        todo!("Check if the queue is empty")
    }
    
    pub fn is_full(&self) -> bool {
        todo!("Check if the queue is full")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_queue_basic() {
        let mut q = MyCircularQueue::new(3);
        assert_eq!(q.en_queue(1), true);
        assert_eq!(q.en_queue(2), true);
        assert_eq!(q.en_queue(3), true);
        assert_eq!(q.en_queue(4), false);
        assert_eq!(q.rear(), 3);
        assert_eq!(q.is_full(), true);
        assert_eq!(q.de_queue(), true);
        assert_eq!(q.en_queue(4), true);
        assert_eq!(q.rear(), 4);
    }
}
