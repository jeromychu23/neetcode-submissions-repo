impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        fn process(s: &str) -> Vec<u8> {
            s.bytes().fold(Vec::new(), |mut stack, c| {
                if c == b'#' {
                    stack.pop();
                } else {
                    stack.push(c);
                }
                stack
            })
        }
        process(&s) == process(&t)
    }
}
