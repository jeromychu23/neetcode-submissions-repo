impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut cur_max = 0;
        let mut cur_min = 0;
        let mut global_max = nums[0];
        let mut global_min = nums[0];

        for n in nums {
            total += n;

            cur_max = n.max(cur_max + n);
            global_max = global_max.max(cur_max);

            cur_min = n.min(cur_min + n);
            global_min = global_min.min(cur_min);
        }
        if global_max < 0 {
            global_max
        } else {
            global_max.max(total - global_min)
        }
    }
}
