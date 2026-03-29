class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        mp = {}
        for i, n in enumerate(numbers):
            diff = target - n
            if diff in mp:
                return [mp[diff] + 1, i + 1]
            mp[n] = i