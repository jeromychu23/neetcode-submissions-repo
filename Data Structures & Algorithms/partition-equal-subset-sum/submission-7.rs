impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: usize = nums.iter().map(|&num| num as usize).sum();

        if !total.is_multiple_of(2) {
            return false;
        }

        let target = total / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums {
            let value = num as usize;

            for cur_sum in (value..=target).rev() {
                dp[cur_sum] = dp[cur_sum] || dp[cur_sum - value]
            }
        }

        dp[target]
    }
}
