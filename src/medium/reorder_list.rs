/// [143] Reorder List
/// Difficulty: Medium
/// Topics: Linked List, Two Pointers, Stack, Recursion
/// Tags: Blind75, NeetCode150
///
/// You are given the head of a singly linked-list. The list can be represented as:
/// L0 → L1 → … → Ln - 1 → Ln
/// Reorder the list to be on the following form:
/// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
/// You may not modify the values in the list's nodes. Only nodes themselves may be changed.
///
/// Link: https://leetcode.com/problems/reorder-list/

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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: head = [1,2,3,4]
        // Output: [1,4,2,3]
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        }));
        Solution::reorder_list(&mut head);

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3))),
                })),
            })),
        }));
        assert_eq!(head, expected);
    }

    #[test]
    fn test_2() {
        // Input: head = [1,2,3,4,5]
        // Output: [1,5,2,4,3]
        let mut head = Some(Box::new(ListNode {
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
        Solution::reorder_list(&mut head);

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            })),
        }));
        assert_eq!(head, expected);
    }

    #[test]
    fn test_3() {
        let mut head = None;
        Solution::reorder_list(&mut head);
        assert_eq!(head, None);
    }
}
