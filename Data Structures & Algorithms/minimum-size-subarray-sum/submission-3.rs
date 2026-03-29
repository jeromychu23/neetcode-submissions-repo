impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut total = 0;
        let mut length = i32::MAX;
        for r in 0..nums.len() {
            total += nums[r as usize];
            while total >= target {
                length = min((r - l + 1) as i32, length);
                total -= nums[l as usize];
                l += 1;
            }
        }
        if length == i32::MAX {
            0
        } else {
            length
        }
    }
}
