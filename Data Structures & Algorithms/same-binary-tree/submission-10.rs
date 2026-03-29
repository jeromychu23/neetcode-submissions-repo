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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let a_ref = a.borrow();
                let b_ref = b.borrow();
                a_ref.val == b_ref.val
                    && Self::is_same_tree(a_ref.left.clone(), b_ref.left.clone())
                    && Self::is_same_tree(a_ref.right.clone(), b_ref.right.clone())
            },
            _ => false,
        }
    }
}
