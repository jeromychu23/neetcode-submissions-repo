impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = "".to_string();
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let (mut i, mut j) = (0, 0);

        while i < word1.len() && j < word2.len() {
            let c1 = word1[i] as char;
            let c2 = word2[j] as char;
            res.push(c1);
            res.push(c2);
            i += 1;
            j += 1;
            if i == word1.len() && j < word2.len() {
                let remaining = std::str::from_utf8(&word2[j..]).unwrap();
                res.push_str(remaining);
            } else if i < word1.len() && j == word2.len() {
                let remaining = std::str::from_utf8(&word1[j..]).unwrap();
                res.push_str(remaining);
            }
        }
        res
    }
}
