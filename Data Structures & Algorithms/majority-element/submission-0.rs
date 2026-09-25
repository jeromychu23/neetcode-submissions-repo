impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        *count.iter().max_by_key(|val| val.1).unwrap().0
    }
}
