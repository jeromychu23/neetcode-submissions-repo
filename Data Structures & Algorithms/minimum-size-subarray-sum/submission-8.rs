impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut total = 0;
        let mut length: Option<i32> = None;

        for r in 0..nums.len() {
            total += nums[r];
            while total >= target {
                let curr = (r - l + 1) as i32;
                length = Some(match length {
                    Some(prev) => min(prev, curr),
                    None => curr
                });
                total -= nums[l];
                l += 1;
            }
        }
        length.unwrap_or(0)
    }
}
