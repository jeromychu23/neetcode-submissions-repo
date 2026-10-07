impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut s_mp = HashMap::new();
        let mut res_str = String::new();

        for c in s.chars() {
            *s_mp.entry(c).or_insert(0) += 1;
        }

        for c in order.chars() {
            if s_mp.contains_key(&c) {
                let count = s_mp.get(&c).unwrap();
                res_str.push_str(&c.to_string().repeat(*count));
                s_mp.remove(&c);
            }
        }
        for (reamin, count) in s_mp {
            res_str.push_str(&reamin.to_string().repeat(count));
        }
        res_str
    }
}
