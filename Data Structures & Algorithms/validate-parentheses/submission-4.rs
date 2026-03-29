impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mp = HashMap::from([(')', '('), ('}', '{'), (']', '[')]);
        for c in s.chars() {
            if let Some(&open) = mp.get(&c) {
                if stack.pop() != Some(open) {
                    return false;
                }
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
