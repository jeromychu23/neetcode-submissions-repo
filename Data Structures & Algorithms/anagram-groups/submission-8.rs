impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for s in &strs {
            let mut count = [0u8; 26];
            for c in s.bytes() {
                count[(c - b'a') as usize] += 1
            }
            map.entry(count).or_default().push(s.clone());
        }
        map.into_values().collect()
    }
}
