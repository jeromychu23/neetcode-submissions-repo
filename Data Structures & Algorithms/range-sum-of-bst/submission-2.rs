use std::rc::Rc;
use std::cell::RefCell;

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
//             right: None
//         }
//     }
// }

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {

        let mut total = 0;

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            low: i32,
            high: i32,
            total: &mut i32
        ) {
            let Some(node) = node else {
                return;
            };

            let node = node.borrow();

            if node.val < low {
                dfs(&node.right, low, high, total);
                return;
            }

            if node.val > high {
                dfs(&node.left, low, high, total);
                return;
            }

            *total += node.val;
            dfs(&node.left, low, high, total);
            dfs(&node.right, low, high, total);
        }

        dfs(&root, low, high, &mut total);
        total
    }
}
