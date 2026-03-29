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
            (Some(r1), Some(s1)) => {
                if Self::same_tree(&Some(r1.clone()), &Some(s1.clone())) {
                    return true;
                }
                
                let r1_ref = r1.borrow();
                Self::is_subtree(r1_ref.left.clone(), Some(s1.clone())) ||
                Self::is_subtree(r1_ref.right.clone(), Some(s1.clone()))
            }
        }
    }
    fn same_tree(r: &Option<Rc<RefCell<TreeNode>>>, s: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (r, s) {
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
