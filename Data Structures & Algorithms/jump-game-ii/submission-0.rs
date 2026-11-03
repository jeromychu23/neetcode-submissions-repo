impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let target = nums.len() - 1;
        let mut l = 0;
        let mut r = 0;
        let mut count = 0;
        let mut farest_r = 0;

        while r < target {
            for i in l..=r {
                farest_r = farest_r.max(i + nums[i] as usize);
            }
            
            l = r + 1;
            r = farest_r;
            count += 1;
        }
        count
    }
}
