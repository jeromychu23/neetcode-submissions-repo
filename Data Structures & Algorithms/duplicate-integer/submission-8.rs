impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut res = HashSet::new();
        for n in nums {
            if !res.insert(n) {
                return true;
            }
        };
        false
    }
}
