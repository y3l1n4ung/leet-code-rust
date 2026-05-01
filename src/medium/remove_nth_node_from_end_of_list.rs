/// [19] Remove Nth Node From End of List
/// Difficulty: Medium
/// Topics: Linked List, Two Pointers
/// Tags: Blind75, NeetCode150
///
/// Given the head of a linked list, remove the nth node from the end of the list and return its head.
///
/// Link: https://leetcode.com/problems/remove-nth-node-from-end-of-list/

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: head = [1,2,3,4,5], n = 2
        // Output: [1,2,3,5]
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let result = Solution::remove_nth_from_end(head, 2);
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::remove_nth_from_end(Some(Box::new(ListNode::new(1))), 1),
            None
        );
    }

    #[test]
    fn test_3() {
        // Input: head = [1,2], n = 1
        // Output: [1]
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        let result = Solution::remove_nth_from_end(head, 1);
        let expected = Some(Box::new(ListNode::new(1)));
        assert_eq!(result, expected);
    }
}
