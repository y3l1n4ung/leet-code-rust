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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: head = [1,2,3,4,5]
        // Output: [5,4,3,2,1]
        todo!("Add linked list test helpers or manual construction");
    }

    #[test]
    fn test_2() {
        // Input: head = [1,2]
        // Output: [2,1]
        todo!();
    }

    #[test]
    fn test_3() {
        // Input: head = []
        // Output: []
        assert_eq!(Solution::reverse_list(None), None);
    }
}
