impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split(' ').count() {
            return false;
        }
        let split: HashSet<&str> = s.split(' ').collect();
        let pattern_set: HashSet<char> = pattern.chars().collect();

        split.len() == pattern_set.len()
    }
}
