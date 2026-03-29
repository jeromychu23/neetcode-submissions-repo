// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root, sub_root) {
            (_, None) => true,
            (None, _) => false,
            (Some(a), Some(b)) => {
                if Self::same_tree(&Some(a.clone()), &Some(b.clone())) {
                    return true;
                };
                let a = a.borrow();
                Self::is_subtree(a.left.clone(), Some(b.clone())) ||
                Self::is_subtree(a.right.clone(), Some(b.clone()))
            }
        }
    }
    fn same_tree(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a = a.borrow();
                let b = b.borrow();
                a.val == b.val
                    && Self::same_tree(&a.left, &b.left)
                    && Self::same_tree(&a.right, &b.right)
            },
            _ => false,
        }
    }
}
