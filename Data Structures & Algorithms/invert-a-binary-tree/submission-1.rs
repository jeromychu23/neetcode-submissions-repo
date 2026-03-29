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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root.as_ref() {
            let (l, r) = {
                let mut node = node_rc.borrow_mut();
                let left = node.left.take();
                let right = node.right.take();
                node.left = right;
                node.right = left;
                (node.left.clone(), node.right.clone())
            };
            Self::invert_tree(l);
            Self::invert_tree(r);
        }
        root
    }
}
