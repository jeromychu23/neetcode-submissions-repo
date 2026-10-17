impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut chars = HashMap::new();

        for c in s.chars() {
            *chars.entry(c).or_insert(0) += 1;
        }

        let mut res = 0;
        for &count in chars.values() {
            res += count / 2 * 2;
        }

        if chars.values().any(|&count| count % 2 == 1) {
            res += 1;
        }
        res

    }
}