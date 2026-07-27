impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.as_bytes();
        let mut dp = vec![false; s.len() + 1];

        dp[0] = true;

        for end in 1..=s.len() {
            dp[end] = word_dict.iter().any(|word| {
                let word = word.as_bytes();
                let start = end.saturating_sub(word.len());

                word.len() <= end && dp[start] && &s[start..end] == word
            })
        }

        dp[s.len()]
    }
}
