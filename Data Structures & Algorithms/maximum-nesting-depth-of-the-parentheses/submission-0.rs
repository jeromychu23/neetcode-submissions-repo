impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;

        for &p in s.as_bytes() {
            match p {
                b'(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                b')' => {
                    depth -= 1;
                }
                _ => {}
            }
        }
        max_depth
    }
}
