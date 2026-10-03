// Definition for a Node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<Node>>>,
//     pub right: Option<Rc<RefCell<Node>>>,
//     pub next: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             left: None,
//             right: None,
//             next: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn connect(
        root: Option<Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        let mut leftmost = root.clone();

        while let Some(left_node) = leftmost.clone() {
            if left_node.borrow().left.is_none() {
                break;
            }

            let mut current = Some(left_node.clone());

            while let Some(node_rc) = current {
                let (left, right, next) = {
                    let node = node_rc.borrow();

                    (
                        node.left.clone(),
                        node.right.clone(),
                        node.next.clone(),
                    )
                };

                // current.left.next = current.right
                if let Some(left) = left {
                    left.borrow_mut().next = right.clone();
                }

                // current.right.next = current.next.left
                if let (Some(right), Some(next)) = (right, next.clone()) {
                    right.borrow_mut().next =
                        next.borrow().left.clone();
                }

                // current = current.next
                current = next;
            }

            // 下一層最左邊
            leftmost = left_node.borrow().left.clone();
        }

        root
    }
}