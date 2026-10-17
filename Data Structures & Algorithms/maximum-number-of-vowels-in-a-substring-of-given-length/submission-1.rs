impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let is_vowel = |c| matches!(c, b'a' | b'e' | b'i' | b'o' | b'u');
        let mut res = 0;
        let mut count = 0;
        
        for (i, &c) in s.iter().enumerate() {
            count += is_vowel(c) as i32;

            if i >= k {
                count -= is_vowel(s[i - k]) as i32;
            }

            if i + 1 >= k {
                res = res.max(count);
            }
        }
        res
    }
}
