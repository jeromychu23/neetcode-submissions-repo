use std::collections::VecDeque;

struct MyStack {
    stack: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack.push_back(x);
        for _ in 0..self.stack.len() - 1 {
            let val = self.stack.pop_front().unwrap();
            self.stack.push_back(val);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
