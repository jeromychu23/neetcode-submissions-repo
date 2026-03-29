class Solution:
    def maxFrequency(self, nums: List[int], k: int) -> int:
        nums.sort();
        l, total, length = 0, 0, 0
        
        for r in range(len(nums)):
            total += nums[r]
            if nums[r] * (r - l + 1) > total + k:
                total -= nums[l]
                l += 1
            length = max(r - l + 1, length)
        
        return length