impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let sb = s.as_bytes();
        let mut seen = HashMap::new();
        let (mut longest, mut l) = (0, 0);
        for r in 0..sb.len() {
            if seen.contains_key(&sb[r]) {
                l = max(seen[&sb[r]] + 1, l);
            }
            seen.insert(sb[r], r);
            longest = max(r - l + 1, longest);
        }
        longest as i32
    }
}
