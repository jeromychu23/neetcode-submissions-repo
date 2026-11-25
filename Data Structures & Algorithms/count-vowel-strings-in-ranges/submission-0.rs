impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut counts = vec![0;words.len() + 1];
        let mut cur = 0;
        for (i, w) in words.iter().enumerate() {
            let w = w.as_bytes();
            let first = w[0] as char;
            let last = *w.last().unwrap() as char;
            if vowel.contains(&first) && vowel.contains(&last) {
                cur += 1;
            }
            counts[i + 1] = cur;
        }

        let mut res = Vec::new();
        for q in queries {
            let start = q[0] as usize;
            let end = q[1] as usize;
            res.push(counts[end + 1] - counts[start])
        }
        res
    }
}