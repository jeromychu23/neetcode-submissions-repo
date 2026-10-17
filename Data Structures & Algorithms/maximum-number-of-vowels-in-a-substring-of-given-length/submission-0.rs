impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();
        let k = k as usize;
        let mut count = 0;
        
        for (i, &c) in chars.iter().enumerate() {
            if c == 'a' ||
                c == 'e' ||
                c == 'i' ||
                c == 'o' ||
                c == 'u' {
                    count += 1;
                } 
            if i + 1 >= k {
                let first = chars[i + 1 - k];
                res = res.max(count);
                if first == 'a' ||
                    first == 'e' ||
                    first == 'i' ||
                    first == 'o' ||
                    first == 'u' {
                        count -= 1;
                    }
            }
        }
        res
    }
}
