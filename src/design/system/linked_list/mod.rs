/// [707] Design Linked List
/// Difficulty: Medium
/// Topics: Linked List, Design
/// Tags: RustMastery
///
/// Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
///
/// Link: https://leetcode.com/problems/design-linked-list/
#[derive(Debug, Clone)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}
pub struct MyLinkedList {
    head: Option<Box<Node>>,
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
    fn test_add_at_head_multiple() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);
        list.add_at_head(2);
        list.add_at_head(3);

        assert_eq!(list.get(0), 3);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), 1);
        assert_eq!(list.get(3), -1);
    }

    #[test]
    fn test_add_at_tail_multiple() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), 3);
        assert_eq!(list.get(3), -1);
    }

    #[test]
    fn test_add_at_index_zero() {
        let mut list = MyLinkedList::new();
        list.add_at_index(0, 10);
        list.add_at_index(1, 9);

        assert_eq!(list.get(0), 10);
        assert_eq!(list.get(1), 9);
        assert_eq!(list.get(2), -1);
    }

    #[test]
    fn test_add_at_index_equal_length() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_index(2, 3);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), 3);
    }

    #[test]
    fn test_add_at_index_greater_than_length() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_index(5, 2);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), -1);
    }

    #[test]
    fn test_add_at_index_negative() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_index(-1, 2);

        // LeetCode behavior: negative index means add at head
        assert_eq!(list.get(0), 2);
        assert_eq!(list.get(1), 1);
    }

    #[test]
    fn test_delete_at_head() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);

        list.delete_at_index(0);

        assert_eq!(list.get(0), 2);
        assert_eq!(list.get(1), 3);
        assert_eq!(list.get(2), -1);
    }

    #[test]
    fn test_delete_at_tail() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);

        list.delete_at_index(2);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), -1);
    }

    #[test]
    fn test_delete_middle() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);
        list.add_at_tail(2);
        list.add_at_tail(3);

        list.delete_at_index(1);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), 3);
        assert_eq!(list.get(2), -1);
    }

    #[test]
    fn test_delete_invalid_index() {
        let mut list = MyLinkedList::new();
        list.add_at_tail(1);

        list.delete_at_index(5);
        list.delete_at_index(-1);

        assert_eq!(list.get(0), 1);
        assert_eq!(list.get(1), -1);
    }

    #[test]
    fn test_delete_only_element() {
        let mut list = MyLinkedList::new();
        list.add_at_head(1);

        list.delete_at_index(0);

        assert_eq!(list.get(0), -1);
    }

    #[test]
    fn test_mixed_operations() {
        let mut list = MyLinkedList::new();

        list.add_at_head(1); // [1]
        list.add_at_tail(3); // [1, 3]
        list.add_at_index(1, 2); // [1, 2, 3]
        list.delete_at_index(0); // [2, 3]
        list.add_at_head(4); // [4, 2, 3]
        list.delete_at_index(2); // [4, 2]

        assert_eq!(list.get(0), 4);
        assert_eq!(list.get(1), 2);
        assert_eq!(list.get(2), -1);
    }
}
