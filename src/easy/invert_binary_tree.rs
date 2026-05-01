use std::cell::RefCell;
/// [226] Invert Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary tree, invert the tree, and return its root.
///
/// Link: https://leetcode.com/problems/invert-binary-tree/
use std::rc::Rc;

// Definition for a binary tree node.
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

struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_1() {
        // Input: root = [4,2,7,1,3,6,9]
        // Output: [4,7,2,9,6,3,1]
        let root = node(4);
        let n2 = node(2);
        let n7 = node(7);
        let n1 = node(1);
        let n3 = node(3);
        let n6 = node(6);
        let n9 = node(9);

        if let Some(ref n) = n2 {
            n.borrow_mut().left = n1;
            n.borrow_mut().right = n3;
        }
        if let Some(ref n) = n7 {
            n.borrow_mut().left = n6;
            n.borrow_mut().right = n9;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n2;
            r.borrow_mut().right = n7;
        }

        let inverted = Solution::invert_tree(root);

        // Manual verification of a few nodes
        if let Some(r) = inverted {
            let r_borrow = r.borrow();
            assert_eq!(r_borrow.val, 4);
            assert_eq!(r_borrow.left.as_ref().unwrap().borrow().val, 7);
            assert_eq!(r_borrow.right.as_ref().unwrap().borrow().val, 2);
        }
    }

    #[test]
    fn test_2() {
        // Input: root = [2,1,3]
        // Output: [2,3,1]
        let root = node(2);
        root.as_ref().unwrap().borrow_mut().left = node(1);
        root.as_ref().unwrap().borrow_mut().right = node(3);

        let inverted = Solution::invert_tree(root);
        if let Some(r) = inverted {
            let r_borrow = r.borrow();
            assert_eq!(r_borrow.val, 2);
            assert_eq!(r_borrow.left.as_ref().unwrap().borrow().val, 3);
            assert_eq!(r_borrow.right.as_ref().unwrap().borrow().val, 1);
        }
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::invert_tree(None), None);
    }
}
