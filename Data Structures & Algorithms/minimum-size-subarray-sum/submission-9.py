class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        j, total, length = 0, 0, float('inf')

        for i in range(len(nums)):
            while j < len(nums) and target > total:
                total += nums[j]
                j += 1 
            if total >= target:
                length = min(j - i, length)
            total -= nums[i]
        
        return 0 if length == float('inf') else length
