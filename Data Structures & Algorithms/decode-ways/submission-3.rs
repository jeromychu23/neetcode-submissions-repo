impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes.first().is_none_or(|&d| d == b'0') {
            return 0;
        }
        let mut prev2 = 1;
        let mut prev1 = 1;

        for pair in bytes.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            let mut cur = 0;

            if b != b'0' {
                cur += prev1
            }
            
            if matches! (
                (a, b),
                (b'1', _) | (b'2', b'0'..=b'6')
            ) {
                cur += prev2
            }
            
            prev2 = prev1;
            prev1 = cur
        }
        prev1
    }
}
