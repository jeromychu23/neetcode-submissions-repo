impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_prefix = 0;
        let mut res = i32::MIN;

        for n in nums {
            sum += n;

            res = res.max(sum - min_prefix);

            min_prefix = min_prefix.min(sum);
        }

        res
    }
}