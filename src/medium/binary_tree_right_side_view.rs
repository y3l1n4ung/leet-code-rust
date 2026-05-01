use std::cell::RefCell;
/// [199] Binary Tree Right Side View
/// Difficulty: Medium
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: NeetCode150
///
/// Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
///
/// Link: https://leetcode.com/problems/binary-tree-right-side-view/
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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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
        // Input: root = [1,2,3,null,5,null,4]
        // Output: [1,3,4]
        let root = node(1);
        let n2 = node(2);
        let n3 = node(3);
        let n5 = node(5);
        let n4 = node(4);

        if let Some(ref n) = n2 {
            n.borrow_mut().right = n5;
        }
        if let Some(ref n) = n3 {
            n.borrow_mut().right = n4;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n2;
            r.borrow_mut().right = n3;
        }

        assert_eq!(Solution::right_side_view(root), vec![1, 3, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::right_side_view(None), Vec::<i32>::new());
    }

    #[test]
    fn test_3() {
        // Input: root = [1,null,3]
        // Output: [1,3]
        let root = node(1);
        root.as_ref().unwrap().borrow_mut().right = node(3);
        assert_eq!(Solution::right_side_view(root), vec![1, 3]);
    }
}
