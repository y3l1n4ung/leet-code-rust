use std::cell::RefCell;
/// [236] Lowest Common Ancestor of a Binary Tree
/// Difficulty: Medium
/// Topics: Tree, Depth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
/// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”
///
/// Link: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
        // root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
        // Output: 3
        let root = node(3);
        let n5 = node(5);
        let n1 = node(1);
        let n6 = node(6);
        let n2 = node(2);
        let n0 = node(0);
        let n8 = node(8);
        let n7 = node(7);
        let n4 = node(4);

        if let Some(ref n) = n2 {
            n.borrow_mut().left = n7;
            n.borrow_mut().right = n4;
        }
        if let Some(ref n) = n5 {
            n.borrow_mut().left = n6;
            n.borrow_mut().right = n2.clone();
        }
        if let Some(ref n) = n1 {
            n.borrow_mut().left = n0;
            n.borrow_mut().right = n8;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n5.clone();
            r.borrow_mut().right = n1.clone();
        }

        let lca = Solution::lowest_common_ancestor(root, n5, n1);
        assert_eq!(lca.as_ref().unwrap().borrow().val, 3);
    }
}
