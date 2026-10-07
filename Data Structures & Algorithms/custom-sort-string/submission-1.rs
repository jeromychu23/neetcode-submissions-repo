impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut s_mp = HashMap::new();
        let mut res_str = String::new();

        for c in s.chars() {
            *s_mp.entry(c).or_insert(0) += 1;
        }

        for c in order.chars() {
            if let Some(n) = s_mp.remove(&c) {
                res_str.extend(std::iter::repeat_n(c, n));
            }
        }
        for (reamin, count) in s_mp {
            res_str.extend(std::iter::repeat_n(reamin, count));
        }
        res_str
    }
}
