impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut res = 0;
        let mut max_count = 0;

        for num in nums {
            let c = count.entry(num).or_insert(0);
            *c += 1;
            if *c > max_count {
                res = num;
                max_count = *c
            }
        }
        res
    }
}
