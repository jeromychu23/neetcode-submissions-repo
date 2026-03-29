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
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) {
            let Some(rc) = node.as_ref() else { return };

            let (l, r) = {
                let mut n = rc.borrow_mut();
                let left = n.left.take();
                let right = n.right.take();
                n.left = right;
                n.right = left;
                (n.left.clone(), n.right.clone())
            };
            dfs(&l);
            dfs(&r);
        }
        dfs(&root);
        root
    }
}
