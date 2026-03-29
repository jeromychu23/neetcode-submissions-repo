impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mp: HashMap<char, char> = [(')', '('), ('}', '{'), (']', '[')].into(); 
        for c in s.chars() {
            if let Some(&open) = mp.get(&c) {
                if !stack.is_empty() && *stack.last().unwrap() == open {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
