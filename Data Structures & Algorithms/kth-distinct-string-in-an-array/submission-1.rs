impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut record = HashMap::new();

        for str in &arr {
            *record.entry(str).or_insert(0) += 1;
        }

        let mut count = 0;

        for str in &arr {
            let num = record.get(&str).unwrap();
            if *num != 1 {
                continue;
            } else {
                count += 1;
                if count == k {
                    return str.clone();
                }
            }
        }

        "".to_string()
    }
}
