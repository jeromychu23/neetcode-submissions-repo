impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if target == nums[m] {
                return m as i32;
            }
            if nums[l] <= nums[m] {
                if target > nums[m] || target < nums[l] {
                    l = m + 1
                } else {
                    r = m - 1
                }
            } else {
                if target < nums[m] || target > nums[r] {
                    r = m - 1
                } else {
                    l = m + 1
                }
            }
        }
        -1
    }
}
