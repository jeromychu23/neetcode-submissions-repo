impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];
        
        let mut pref = 1;
        for i in 0..n {
            res[i] = pref;
            pref *= nums[i];
        }

        let mut suf = 1;
        for i in (0..n).rev() {
            res[i] *= suf;
            suf *= nums[i];
        }
        res
    }
}
