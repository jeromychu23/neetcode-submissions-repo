impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mut m = (l + r) / 2;
            if nums[m] > nums[r] {
                l = m + 1
            } else {
                r = m
            }
        }
        nums[l]
    }
}
