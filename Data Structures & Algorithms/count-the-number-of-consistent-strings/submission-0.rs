impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allow_count = [0; 26];
        let mut res = 0;

        for c in allowed.as_bytes() {
            allow_count[(c - b'a') as usize] = 1;
        }
        println!("{:?}", allow_count);

        for word in words {
            let mp: HashSet<&u8> = HashSet::from_iter(word.as_bytes());
            let mut if_count = true;
            for w in mp {
                if allow_count[(w - b'a') as usize] == 0 {
                    if_count = false;
                    break;
                }
            }
            if if_count {
                println!("{word}");
                res += 1;
            }
        }
        res
    }
}
