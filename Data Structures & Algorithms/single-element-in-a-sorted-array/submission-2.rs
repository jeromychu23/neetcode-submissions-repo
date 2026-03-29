impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let mut m = l + (r - l) / 2;
            if m % 2 != 0 {  
                m = m - 1;
            }
            if nums[m] == nums[m + 1] {
                l = m + 2;
            } else {
                r = m;
            }
        }
        return nums[l];
    }
}
