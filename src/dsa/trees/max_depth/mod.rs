// Maximum Depth of Binary Tree Practice 🦀

use std::cell::RefCell;
use std::rc::Rc;

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
    /// Practice: Max Depth of Binary Tree
    /// Return the maximum depth of the given binary tree.
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left_depth = Self::max_depth(node.borrow().left.clone());
                let right_depth = Self::max_depth(node.borrow().right.clone());
                1 + std::cmp::max(left_depth, right_depth)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_max_depth() {
        // [3,9,20,null,null,15,7]
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

        assert_eq!(Solution::max_depth(root), 3);
        assert_eq!(Solution::max_depth(None), 0);
        assert_eq!(Solution::max_depth(node(1)), 1);
    }
}
