impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut res = 0;
        let mut max_count = 0;

        for num in nums {
            *count.entry(num).or_insert(0) += 1;
            if *count.get(&num).unwrap() > max_count {
                res = num;
            }
            max_count = max_count.max(*count.get(&num).unwrap())
        }
        res
    }
}
