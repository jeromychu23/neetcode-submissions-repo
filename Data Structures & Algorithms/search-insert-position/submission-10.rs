impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0i32;
        let mut r = nums.len() as i32 - 1;
        while l <= r {
            let m = l + (r - l) / 2;
            if target == nums[m as usize] {
                return m;
            }
            if target > nums[m as usize] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        l
    }
}
