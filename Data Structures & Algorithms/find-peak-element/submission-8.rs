impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0i32;
        let mut r = nums.len() as i32 - 1;
        while l <= r {
            let m = l + (r - l) / 2;
            if m > 0 && nums[m as usize] < nums[m as usize - 1] {
                r = m - 1;
            } else if m < nums.len() as i32 - 1 && nums[m as usize] < nums[m as usize+ 1] {
                l = m + 1;
            } else {
                return m;
            }
        }
        -1
    }
}
