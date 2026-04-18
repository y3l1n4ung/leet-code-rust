/// [21] Merge Two Sorted Lists
/// Difficulty: Easy
/// Topics: Linked List, Recursion
/// Tags: Blind75, NeetCode150
///
/// You are given the heads of two sorted linked lists list1 and list2.
/// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.
///
/// Link: https://leetcode.com/problems/merge-two-sorted-lists/

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy.next;

        while list1.is_some() && list2.is_some() {
            let lh = list1.as_ref()?.val;
            let rh = list2.as_ref()?.val;
            if lh <= rh {
                let mut node = list1.take()?;
                list1 = node.next.take();

                *current = Some(node);
            } else {
                let mut node = list2.take()?;
                list2 = node.next.take();
                *current = Some(node);
            }
            current = &mut current.as_mut().unwrap().next;
        }
        *current = list1.or(list2);
        dummy.next
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
        let l1 = to_list(vec![1, 2, 4]);
        let l2 = to_list(vec![1, 3, 4]);
        let output = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(output), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_3() {
        let l1 = None;
        let l2 = to_list(vec![0]);
        let output = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(output), vec![0]);
    }

    #[test]
    fn test_unbalanced_lists() {
        let l1 = to_list(vec![1, 2, 3]);
        let l2 = to_list(vec![4, 5, 6]);
        let output = Solution::merge_two_lists(l1, l2);
        assert_eq!(to_vec(output), vec![1, 2, 3, 4, 5, 6]);
    }
}
