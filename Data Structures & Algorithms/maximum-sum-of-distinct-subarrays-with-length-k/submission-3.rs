use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (mut l, mut sum, mut res) = (0, 0i64, 0i64);
        let mut seen = HashSet::new();
        let k = k as usize;

        for (r, &num) in nums.iter().enumerate() {
            while !seen.insert(num) {
                seen.remove(&nums[l]);
                sum -= nums[l] as i64;
                l += 1;
            }

            sum += num as i64;

            if r - l + 1 == k {
                res = res.max(sum);
                seen.remove(&nums[l]);
                sum -= nums[l] as i64;
                l += 1;
            }
        }

        res
    }
}