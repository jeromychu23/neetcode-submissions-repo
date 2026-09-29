impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut missing = Vec::new();

        for i in 0..nums.len() {
            let j = (nums[i].abs() - 1) as usize;
            nums[j] = -nums[j].abs()
        }

        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                missing.push((i + 1) as i32);
            }
        }
        missing

    }
}
