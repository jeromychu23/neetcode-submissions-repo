impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut char_count = HashMap::new();

        for c in chars.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }

        let mut res = 0;

        for word in words {
            let mut word_count = HashMap::new();

            for c in word.chars() {
                *word_count.entry(c).or_insert(0) += 1;
            }

            let valid = word_count
                .iter()
                .all(|(c, n)| char_count.get(c).unwrap_or(&0) >= n);

            if valid {
                res += word.len() as i32;
            }
        }

        res
    }
}
