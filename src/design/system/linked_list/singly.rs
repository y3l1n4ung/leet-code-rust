/// [707] Design Linked List (Singly Implementation)
/// Difficulty: Medium
/// Topics: Linked List, Design
/// Tags: RustMastery
///
/// Design your implementation of the linked list.
///
/// Link: https://leetcode.com/problems/design-linked-list/

#[derive(Debug, Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

pub struct MyLinkedList {
    pub head: Option<Box<Node>>,
}

impl MyLinkedList {
    pub fn new() -> Self {
        MyLinkedList { head: None }
    }
    /// Get the value at the specific index or return -1.
    pub fn get(&self, index: i32) -> i32 {
        if self.head.is_none() || index < 0 {
            return -1;
        }
        let mut current_index = 0;
        let mut current_head = self.head.as_ref();

        while let Some(node) = current_head {
            if current_index == index {
                return node.val;
            }
            current_index += 1;
            current_head = node.next.as_ref();
        }
        -1
    }

    /// Add a node at the head.
    pub fn add_at_head(&mut self, val: i32) {
        let mut new_node = Box::new(Node { val, next: None });
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    /// Add a node at the tail.
    pub fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(Node { val, next: None });

        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if node.next.is_none() {
                node.next = Some(new_node.clone());
                break;
            }
            current = node.next.as_mut();
        }
        if self.head.is_none() {
            self.head = Some(new_node);
        }
    }
    /// Add a node at the specified index
    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
            return;
        }
        let mut current_index = 0;
        let mut current_head = self.head.as_mut();

        while let Some(node) = current_head {
            if current_index == index - 1 {
                let next = node.next.take();
                node.next = Some(Box::new(Node { val, next }));
                return;
            }

            current_head = node.next.as_mut();
            current_index += 1;
        }
    }
    /// Delete the node at the specified index.
    pub fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        if index == 0 {
            if let Some(mut head) = self.head.take() {
                self.head = head.next.take();
            }
            return;
        }

        let mut current_index = 0;
        let mut current_head = self.head.as_mut();

        while let Some(node) = current_head {
            if current_index == index - 1 {
                if let Some(mut target) = node.next.take() {
                    node.next = target.next.take();
                }
                return;
            }
            current_index += 1;
            current_head = node.next.as_mut();
        }
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

    #[test]
    fn test_get_invalid_index() {
        let list = MyLinkedList::new();
        assert_eq!(list.get(0), -1);
        assert_eq!(list.get(5), -1);
    }

    #[test]
    fn test_delete_only_element() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.delete_at_index(0);
        assert_eq!(list.get(0), -1);
    }
}
