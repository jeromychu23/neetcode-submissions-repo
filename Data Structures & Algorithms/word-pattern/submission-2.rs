impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s_split: Vec<&str> = s.split(' ').collect();
        if pattern.len() != s_split.len() {
            return false;
        }

        let mut p_map = HashMap::new();
        let mut s_map = HashMap::new();

        for (i, c) in pattern.chars().enumerate() {
            if let Some(a) = p_map.get(&c)
                && s_split[i] != *a
            {
                return false;
            }
            if let Some(a) = s_map.get(&s_split[i])
                && c != *a
            {
                return false;
            }

            p_map.insert(c, s_split[i]);
            s_map.insert(s_split[i], c);
        }
        true
    }
}
