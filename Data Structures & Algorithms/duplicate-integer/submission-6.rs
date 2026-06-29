impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut res = HashSet::new();
        for n in nums {
            if res.contains(&n) {
                return true;
            };
            res.insert(n);
        };
        false
    }
}
