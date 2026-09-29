impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = HashSet::new();
        let range = nums.len();
        let mut missing = Vec::new();

        for num in &nums {
            seen.insert(num);
        }

        for i in 1..=range {
            let i = i as i32;
            if !seen.contains(&i) {
                missing.push(i);
            }
        }

        missing
    }
}
