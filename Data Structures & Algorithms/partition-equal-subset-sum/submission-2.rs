impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();

        if total % 2 != 0 {
            return false;
        }

        let target = total as usize / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums {
            let value = num as usize;
            if value > target {
                return false;
            }
            for cur_sum in (value..=target).rev() {
                dp[cur_sum] |= dp[cur_sum - value]
            }
        }

        dp[target]
    }
}
