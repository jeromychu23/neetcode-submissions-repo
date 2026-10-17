impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return vec![];
        }
        let s = s.as_bytes();
        let mut res = Vec::new();
        let mut p_count = vec![0; 26 as usize];
        let len = p.len();

        for &c in p.as_bytes() {
            p_count[(c - b'a') as usize] += 1;
        }
        println!("{:?}", p_count);

        for (i, &c) in s.iter().enumerate() {
            if p_count[(c - b'a') as usize] == 0 {
                continue;
            }
            let mut r = i;
            let mut s_count = vec![0; 26 as usize];
            while r < s.len() && r - i + 1 <= p.len() {
                let rc = s[r];
                s_count[(rc - b'a') as usize] += 1;
                r += 1;
            }
            if s_count == p_count {
                res.push(i as i32);
            }

        }
        res
    }
}
