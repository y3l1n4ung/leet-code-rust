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
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: list1 = [1,2,4], list2 = [1,3,4]
        // Output: [1,1,2,3,4,4]
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: list1 = [], list2 = []
        // Output: []
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_3() {
        // Input: list1 = [], list2 = [0]
        // Output: [0]
        todo!();
    }
}
