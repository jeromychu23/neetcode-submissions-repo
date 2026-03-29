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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = Vec::new();
        Self::dfs(&root, &mut res);
        res.iter().all(|&x| x <= 1)
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                let left = Self::dfs(&node.left, res);
                let right = Self::dfs(&node.right, res);
                res.push((left - right).abs());
                1 + max(left, right)
            }
        }
    }
}
