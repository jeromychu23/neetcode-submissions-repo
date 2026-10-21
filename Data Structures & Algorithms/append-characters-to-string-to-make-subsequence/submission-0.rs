impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut t_count = [0;26];
        
        for c in t {
            t_count[(c - b'a') as usize] += 1;
        }

        let mut i = 0;
        let mut j = 0;
        while i < s.len() {
            if s[i] != t[j] {
                i += 1;
                continue;
            }
            t_count[(s[i] - b'a') as usize] -= 1;
            if t_count.iter().sum::<i32>() == 0 {
                return 0;
            }
            i += 1;
            j += 1;

        }
        t_count.iter().sum()
        
    }
}
