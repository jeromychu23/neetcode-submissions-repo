impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut char_count = HashMap::new();
        let mut word_count = vec![HashMap::new(); words.len()];

        let mut res = 0;

        for c in chars.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        for (i, word) in words.iter().enumerate() {
            for w in word.chars() {
                *word_count[i].entry(w).or_insert(0) += 1;
            }
        }

        for w_cnt in word_count {
            let if_pass = w_cnt
                .iter()
                .all(|(c, n)| char_count.contains_key(c) && char_count.get(c) >= Some(n));
            if if_pass {
                for (c, n) in w_cnt {
                    if char_count.contains_key(&c) && char_count.get(&c) >= Some(&n) {
                        res += n;
                    }
                }
            }
        }

        res
    }
}
