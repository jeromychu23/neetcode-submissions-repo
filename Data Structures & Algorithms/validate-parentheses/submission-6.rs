impl Solution {
    pub fn is_valid(s: String) -> bool {
        let pair = HashMap::from([(b')', b'('), (b'}', b'{'), (b']', b'[')]);
        let mut stack = vec![];

        for p in s.as_bytes() {
            if !pair.contains_key(p) {
                stack.push(*p);
                continue;
            }

            let find = pair.get(p).unwrap();
            if stack.last() != Some(find) {
                return false;
            }

            stack.pop();
        }
        stack.is_empty()
    }
}
