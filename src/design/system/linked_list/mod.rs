/// [707] Design Linked List
/// Difficulty: Medium
/// Topics: Linked List, Design
/// Tags: RustMastery
///
/// Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
///
/// Link: https://leetcode.com/problems/design-linked-list/

pub struct MyLinkedList {
    // Add your internal state here
}

impl MyLinkedList {
    pub fn new() -> Self {
        todo!("Initialize the linked list")
    }
    
    pub fn get(&self, index: i32) -> i32 {
        todo!("Get the value at the specific index or return -1")
    }
    
    pub fn add_at_head(&mut self, val: i32) {
        todo!("Add a node at the head")
    }
    
    pub fn add_at_tail(&mut self, val: i32) {
        todo!("Add a node at the tail")
    }
    
    pub fn add_at_index(&mut self, index: i32, val: i32) {
        todo!("Add a node at the specified index")
    }
    
    pub fn delete_at_index(&mut self, index: i32) {
        todo!("Delete the node at the specified index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_basic() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(list.get(1), 2);
        list.delete_at_index(1);
        assert_eq!(list.get(1), 3);
    }
}
