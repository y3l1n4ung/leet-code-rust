/// [206] Reverse Linked List
/// Difficulty: Easy
/// Topics: Linked List, Recursion
/// Tags: Blind75, NeetCode150
///
/// Given the head of a singly linked list, reverse the list, and return the reversed list.
///
/// Link: https://leetcode.com/problems/reverse-linked-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    #[test]
    fn test_1() {
        let input = to_list(vec![1, 2, 3, 4, 5]);
        let output = Solution::reverse_list(input);
        assert_eq!(to_vec(output), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_2() {
        let input = to_list(vec![1, 2]);
        let output = Solution::reverse_list(input);
        assert_eq!(to_vec(output), vec![2, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
