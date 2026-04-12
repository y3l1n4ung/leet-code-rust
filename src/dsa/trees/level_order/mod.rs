// Binary Tree Level Order Traversal Practice 🦀

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    /// Practice: Level Order Traversal
    /// Returns the values of each level in a 2D vector.
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if root.is_none() { return result; }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let level_size = queue.len();
            let mut current_level = Vec::new();

            for _ in 0..level_size {
                if let Some(node) = queue.pop_front() {
                    current_level.push(node.borrow().val);
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
            result.push(current_level);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_level_order() {
        let root = node(3);
        let left = node(9);
        let right = node(20);
        let r_left = node(15);
        let r_right = node(7);

        if let Some(ref r) = right {
            r.borrow_mut().left = r_left;
            r.borrow_mut().right = r_right;
        }
        if let Some(ref r_node) = root {
            r_node.borrow_mut().left = left;
            r_node.borrow_mut().right = right;
        }

        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }
}
