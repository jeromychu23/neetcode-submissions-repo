class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        current = 0
        _max = 0
        for n in nums:
            if n == 1:
                current += 1
                _max = max(current, _max)
            else:
                current = 0
        
        return _max