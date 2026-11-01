impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut cur = nums[0];
        let mut max = nums[0];
        
        for &n in nums.iter().skip(1) {
            cur = n.max(cur + n);
            max = max.max(cur);
        }
        max
    }
}
