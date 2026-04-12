/// [23] Merge k Sorted Lists
/// Difficulty: Hard
/// Topics: Linked List, Divide and Conquer, Heap (Priority Queue), Merge Sort
/// Tags: Blind75, NeetCode150
///
/// You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
/// Merge all the linked-lists into one sorted linked-list and return it.
///
/// Link: https://leetcode.com/problems/merge-k-sorted-lists/

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: lists = [[1,4,5],[1,3,4],[2,6]]
        // Output: [1,1,2,3,4,4,5,6]
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(5))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));
        let l3 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(6))),
        }));
        
        let result = Solution::merge_k_lists(vec![l1, l2, l3]);
        
        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode::new(6))),
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
    }
}
