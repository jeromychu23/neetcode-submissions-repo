impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_prod = nums[0];
        let mut prev_max = nums[0];
        let mut prev_min = nums[0];
        for &num in nums.iter().skip(1) {
            let old_max = prev_max;
            let old_min = prev_min;
            let cur_max = [num, old_max * num, old_min * num].into_iter().max().unwrap();
            let cur_min = [num, old_min * num, old_max * num].into_iter().min().unwrap();
            prev_max = cur_max;
            prev_min = cur_min;
            max_prod = max_prod.max(cur_max)
        }
        max_prod
    }
}
