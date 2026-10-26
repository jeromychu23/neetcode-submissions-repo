impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let (mut res, mut cur) = (0i64, 0i64);
        let mut l = 0;
        let k = k as usize;
        let mut seen = HashSet::new();

        for r in 0..nums.len() {
            while seen.contains(&nums[r]) {
                seen.remove(&nums[l]);
                cur -= nums[l] as i64;
                l += 1;
            }
            seen.insert(nums[r]);
            cur += nums[r] as i64;

            if r - l + 1 == k {
                res = res.max(cur);
                seen.remove(&nums[l]);
                cur -= nums[l] as i64;
                l += 1;
            }
        }        
        res
    }
}
