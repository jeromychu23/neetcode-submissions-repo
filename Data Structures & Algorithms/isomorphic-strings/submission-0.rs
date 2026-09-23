impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_to_t = HashMap::new();
        let mut t_to_s = HashMap::new();

        for (&s_ch, &t_ch) in s.as_bytes().iter().zip(t.as_bytes()) {
            if let Some(&ch) = s_to_t.get(&s_ch)
                && ch != t_ch
            {
                return false;
            }

            if let Some(&ch) = t_to_s.get(&t_ch)
                && ch != s_ch
            {
                return false;
            }

            s_to_t.insert(s_ch, t_ch);
            t_to_s.insert(t_ch, s_ch);
        }
        true
    }
}
