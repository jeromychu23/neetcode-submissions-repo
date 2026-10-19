impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }

        let mut substrings = HashSet::new();
        let mut repeats = HashSet::new();

        for c in s.as_bytes().windows(10) {
            let substr = std::str::from_utf8(c).unwrap().to_string();
            if !substrings.insert(substr.clone()) {
                repeats.insert(substr);
            }
        }
        repeats.into_iter().collect()
    }
}
