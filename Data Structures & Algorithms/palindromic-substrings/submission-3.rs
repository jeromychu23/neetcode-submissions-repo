impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut res = 0i32;
        for center in 0..bytes.len() {
            res += Self::count_pali(bytes, 0, center, center);
            res += Self::count_pali(bytes, 0, center, center + 1);
        }
        res
    }
    fn count_pali(
        bytes: &[u8],
        mut count: i32,
        mut left: usize,
        mut right: usize
    ) -> i32 {
        while right < bytes.len() && bytes[left] == bytes[right] {
            count += 1;
            let Some(next_left) = left.checked_sub(1) else {
                return count;
            };
            left = next_left;
            right += 1
        }
        count
    }
}
