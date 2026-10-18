impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut res = Vec::new();
        let mut p_count = [0; 26];
        let mut s_count = [0; 26];
        let window = p.len();

        for &c in p {
            p_count[(c - b'a') as usize] += 1;
        }

        for (i, &c) in s.iter().enumerate() {
            s_count[(c - b'a') as usize] += 1;

            if i >= window {
                s_count[(s[i - window] - b'a') as usize] -= 1;
            }
            if i + 1 >= window && s_count == p_count {
                res.push((i + 1 - window) as i32);
            }
            if p_count[(c - b'a') as usize] == 0 {
                continue;
            }
        }
        res
    }
}
