// Definition for a N-ary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub children: Vec<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             children: Vec::new(),
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        let mut res = vec![];

        fn dfs(node: &Rc<RefCell<Node>>, res: &mut Vec<i32>) {

            let node = node.borrow();

            for child in &node.children {
                dfs(child, res);
            }

            res.push(node.val);
        } 

        if let Some(root) = root {
            dfs(&root, &mut res);
        }

        res
    }
}
