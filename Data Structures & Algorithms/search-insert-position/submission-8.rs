impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0i32, nums.len() as i32 - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m as usize] == target {
                return m;
            }
            if nums[m as usize] < target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        l
    }
}
