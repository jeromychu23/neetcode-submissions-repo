struct PrefixTree {
    words: HashSet<String>,
}

impl PrefixTree {
    pub fn new() -> Self {
        let mut words = HashSet::new();
        PrefixTree {
            words
        }
    }

    pub fn insert(&mut self, word: String) {
        self.words.insert(word);
    }

    pub fn search(&self, word: String) -> bool {
        self.words.contains(&word)
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        for w in &self.words {
            if w.starts_with(&prefix) {
                return true;
            }
        }
        false
    }
}
