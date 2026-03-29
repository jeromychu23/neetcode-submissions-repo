impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let mut l = 0;
        let mut product = 1;
        for r in 0..nums.len() {
            product *= nums[r];
            while l <= r && product >= k {
                product = product / nums[l];
                l += 1
            }
            res += r - l + 1;
        }
        res as i32
    }
}
