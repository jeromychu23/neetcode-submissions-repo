impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        if s_bytes.is_empty() || s_bytes[0] == b'0' {
            return 0;
        }

        let mut dp_minus_1 = 1i32;
        let mut dp_minus_2 = 1i32;

        for i in 2..=s_bytes.len() {
            let mut dp_i = 0i32;

            if s_bytes[i - 1] != b'0' {
                dp_i += dp_minus_1
            }

            if s_bytes[i - 2] == b'1' || (
                s_bytes[i - 2] == b'2' && s_bytes[i - 1] <= b'6'
            ) {
                dp_i += dp_minus_2
            }

            dp_minus_2 = dp_minus_1;
            dp_minus_1 = dp_i
        }
        dp_minus_1
    }
}
