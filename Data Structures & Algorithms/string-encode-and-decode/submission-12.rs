impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();
        for s in strs {
            res.push_str(&s.len().to_string());
            res.push('#');
            res.push_str(&s);
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let s_bytes = s.as_bytes();
        let mut res = Vec::new();
        let mut i: usize = 0;
        while i < s_bytes.len() {
            let mut j = i;
            while s_bytes[j] != b'#' {
                j += 1
            }
            let length: usize = s[i..j].parse().unwrap();
            i = j + 1;
            j = i + length;
            res.push(s[i..j].to_string());
            i = j;
        }
        res
    }
}
