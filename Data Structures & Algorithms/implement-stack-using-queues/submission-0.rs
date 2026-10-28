use std::collections::VecDeque;

struct MyStack {
    stack: Vec<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push(x)
    }

    pub fn pop(&mut self) -> i32 {
        let res = self.stack.pop().unwrap();
        res
    }

    pub fn top(&self) -> i32 {
        let res = self.stack.last().unwrap();
        *res 
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
