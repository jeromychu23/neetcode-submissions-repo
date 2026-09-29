impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut record = HashMap::new();

        for s in &arr {
            *record.entry(s).or_insert(0) += 1;
        }

        let mut count = 0;

        for s in &arr {
            let num = record.get(s).unwrap();
            if *num != 1 {
                continue;
            }
            count += 1;
            if count == k {
                return s.clone();
            }
        }

        "".to_string()
    }
}
