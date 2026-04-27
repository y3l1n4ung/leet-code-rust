use std::cell::RefCell;
use std::rc::{Rc, Weak};

/// [707] Design Linked List (Doubly Implementation)
/// Difficulty: Medium
/// Topics: Linked List, Design
/// Tags: RustMastery
///
/// Design your implementation of the linked list.
///
/// Link: https://leetcode.com/problems/design-linked-list/

#[derive(Debug)]
pub struct DoublyNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<DoublyNode>>>,
    pub prev: Option<Weak<RefCell<DoublyNode>>>,
}

pub struct MyDoublyLinkedList {
    pub head: Option<Rc<RefCell<DoublyNode>>>,
    pub tail: Option<Rc<RefCell<DoublyNode>>>,
    pub size: i32,
}

impl MyDoublyLinkedList {
    /// Initialize your data structure here.
    pub fn new() -> Self {
        MyDoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Get the value of the index-th node in the linked list. If the index is invalid, return -1.
    pub fn get(&self, index: i32) -> i32 {
        if self.head.clone().is_none() {
            return -1;
        }
        if index == 0 {
            if let Some(node) = &self.head {
                return node.borrow().val;
            }
        }
        let mut current_index = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            if current_index == index {
                return node.borrow().val;
            }
            current = node.borrow().next.clone();
            current_index += 1;
        }
        -1
    }

    /// Add a node of value val before the first element of the linked list.
    /// After the insertion, the new node will be the first node of the linked list.
    pub fn add_at_head(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(DoublyNode {
            prev: None,
            next: self.head.clone(),
            val,
        }));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node.clone())
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    /// Append a node of value val as the last element of the linked list.
    pub fn add_at_tail(&mut self, val: i32) {
        // [A , B , C]
        // <-prev D next-> [none]

        // head -> A
        // tail -> D
        let new_node = Rc::new(RefCell::new(DoublyNode {
            prev: None,
            next: None,
            val,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
        self.size += 1;
    }

    /// Add a node of value val before the index-th node in the linked list.
    /// If index equals the length of the linked list, the node will be appended to the end of the linked list.
    /// If index is greater than the length, the node will not be inserted.
    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size {
            return;
        }
        let current_index = 0;
        let current = self.head.clone();
        let new_node = Rc::new(RefCell::new(DoublyNode {
            next: None,
            prev: None,
            val,
        }));
        // while let Some(_node) = current {
        //     if (current_index - 1) == index {
        //         _node.borrow_mut().next = Some(new_node.clone());
        //     }
        //     current_index += 1;
        // }
    }

    /// Delete the index-th node in the linked list, if the index is valid.
    pub fn delete_at_index(&mut self, index: i32) {
        todo!("Implement Doubly Linked List delete_at_index()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_new() {
        let list = MyDoublyLinkedList::new();
        assert_eq!(list.size, 0);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_doubly_add_at_head() {
        let mut list = MyDoublyLinkedList::new();
        list.add_at_head(1);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_doubly_add_at_tail() {
        let mut list = MyDoublyLinkedList::new();
        list.add_at_tail(1);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.size, 1);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_doubly_get() {
        let mut list = MyDoublyLinkedList::new();
        list.add_at_head(1);
        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), -1);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_doubly_add_at_index() {
        let mut list = MyDoublyLinkedList::new();
        list.add_at_head(1);
        list.add_at_tail(3);
        list.add_at_index(1, 2);
        assert_eq!(list.get(1), 2);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn test_doubly_delete_at_index() {
        let mut list = MyDoublyLinkedList::new();
        list.add_at_head(1);
        list.delete_at_index(0);
        assert_eq!(list.size, 0);
    }
}
