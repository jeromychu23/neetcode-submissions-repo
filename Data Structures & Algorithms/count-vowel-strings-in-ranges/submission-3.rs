impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowel = HashSet::from([b'a', b'e', b'i', b'o', b'u']);
        let mut counts = vec![0;words.len() + 1];
        for (i, w) in words.iter().enumerate() {
            let w = w.as_bytes();
            let first = w[0];
            let last = w.last().unwrap();

            counts[i + 1] = counts[i];
            if vowel.contains(&first) && vowel.contains(&last) {
                counts[i + 1] += 1;
            }
        }

        queries
            .iter()
            .map(|q| {
                let (start, end) = (q[0] as usize, q[1] as usize);
                counts[end + 1] - counts[start]
            }) 
            .collect()
    }
}