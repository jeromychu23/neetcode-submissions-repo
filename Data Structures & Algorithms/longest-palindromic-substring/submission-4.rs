use std::ops::Range;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        fn expand(bytes: &[u8], mut left: usize, mut right: usize) -> Range<usize> {
            while right < bytes.len() && bytes[left] == bytes[right] {
                let Some(next_left) = left.checked_sub(1) else {
                    return 0..right + 1;
                };
                left = next_left;
                right += 1;
            }
            left + 1..right
        }

        let mut best = 0..1;

        for center in 0..bytes.len() {
            let odd = expand(bytes, center, center);
            if odd.len() > best.len() {
                best = odd;
            }
            
            if center + 1 < bytes.len() && bytes[center] == bytes[center + 1] {
                let even = expand(bytes, center, center + 1);

                if even.len() > best.len() {
                    best = even;
                }
            }
        }
        s[best].to_owned()
    }
}
