impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        let mut i = 0;

        while i < w1.len() && i < w2.len() {
            res.push(w1[i] as char);
            res.push(w2[i] as char);
            i += 1;
        }
        
        res.push_str(&word1[i..]);
        res.push_str(&word2[i..]);
        res
    }
}
