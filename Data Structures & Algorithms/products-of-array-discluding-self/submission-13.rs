impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1i32; nums.len()];
        
        let mut pref = 1;
        for i in 0..nums.len() {
            res[i] = pref;
            pref *= nums[i];
        }

        let mut suf = 1;
        for i in (0..nums.len()).rev() {
            res[i] *= suf;
            suf *= nums[i];
        }
        res
    }
}
