class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        if sum(nums) % 2:
            return False
        
        target = sum(nums) // 2
        dp = [False] * (target + 1)
        dp[0] = True

        for num in nums:
            if num > target:
                return False
            
            for cur_sum in range(target, num - 1, -1):
                dp[cur_sum] = dp[cur_sum] or dp[cur_sum - num]
        
        return dp[target]