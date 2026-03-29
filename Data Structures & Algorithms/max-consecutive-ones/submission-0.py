class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        current = 0
        _max = 0
        for n in nums:
            if n == 1:
                current += 1
            else:
                _max = max(current, _max)
                current = 0
        
        return max(current, _max)