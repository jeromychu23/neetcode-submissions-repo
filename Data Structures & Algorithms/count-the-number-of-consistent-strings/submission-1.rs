impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allow_set: HashSet<u8> = allowed.bytes().collect();
        let mut res = words.len() as i32;

        for word in words {
            for b in word.as_bytes() {
                if !allow_set.contains(b) {
                    res -= 1;
                    break;
                }
            }
        }

        res
    }
}
