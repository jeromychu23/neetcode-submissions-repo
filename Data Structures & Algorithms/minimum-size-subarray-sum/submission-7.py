class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        l, total, length = 0, 0, float('inf')
        for r in range(len(nums)):
            total += nums[r]
            while total >= target:
                length = min(r - l + 1, length)
                total -= nums[l]
                l += 1
        return 0 if length == float('inf') else length
