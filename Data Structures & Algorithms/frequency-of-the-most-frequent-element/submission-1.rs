impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut l, mut total, mut length) = (0, 0, 0);
        for r in 0..nums.len() {
            total += nums[r];

            if ((r - l + 1) as i32) * nums[r] > total + k {
                total -= nums[l];
                l += 1;
            }
            length = max((r - l + 1) as i32, length);
        }
        length
    }
}
