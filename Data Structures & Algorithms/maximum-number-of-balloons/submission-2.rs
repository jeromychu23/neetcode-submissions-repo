impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut count = HashMap::from([(b'b', 0), (b'a', 0), (b'l', 0), (b'o', 0), (b'n', 0)]);

        for b in text.as_bytes() {
            if let Some(n) = count.get_mut(b) {
                *n += 1;
            }
        }
        *count.get_mut(&b'l').unwrap() /= 2;
        *count.get_mut(&b'o').unwrap() /= 2;

        let mut res = 100000;
        for (b, c) in count {
            if c == 0 {
                return 0;
            }
            res = res.min(c)
        }

        res
    }
}
