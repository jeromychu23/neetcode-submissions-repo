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
    pub fn connect(root: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let node = root.clone()?;

        let (left, right, next) = {
            let n = node.borrow();
            (n.left.clone(), n.right.clone(), n.next.clone())
        };

        if let (Some(left), Some(right)) = (&left, &right) {
            left.borrow_mut().next = Some(right.clone());

            if let Some(next) = next {
                right.borrow_mut().next = next.borrow().left.clone();
            }

            Self::connect(Some(left.clone()));
            Self::connect(Some(right.clone()));
        }

        root
    }
}
