impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);

        fn is_palindrome(s: &[u8], mut i: usize, mut j: usize) -> bool {
            while i < j {
                if s[i] != s[j] {
                    return false
                }
                i += 1;
                j -= 1;
            }
            true
        }

        while i < j {
            if s[i] != s[j] {
                return is_palindrome(s, i + 1, j)
                    || is_palindrome(s, i, j - 1);
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
