impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut max_int = -1;

        for i in arr {
            *count.entry(i).or_insert(0) += 1;
        }

        for (k, v) in count {
            if k == v {
                max_int = max_int.max(v);
            }
        }

        max_int
    }
}
