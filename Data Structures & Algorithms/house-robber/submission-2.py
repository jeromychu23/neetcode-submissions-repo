class Solution:
    def rob(self, nums: List[int]) -> int:
        dp = [0] * len(nums)
        for i in range(len(nums)):
            if i > 2:
                dp[i] = nums[i] + max(dp[i - 2], dp[i - 3])
            elif i == 2:
                dp[i] = nums[i] + dp[i - 2]
            else:
                dp[i] = nums[i]
        
        return max(dp)