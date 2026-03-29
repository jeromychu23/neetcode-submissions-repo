class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        set_nums = set(nums)
        res = 0
        for n in set_nums:
            if n - 1 not in set_nums:
                length = 1
                while n + length in set_nums:
                    length += 1
                res = max(res, length)
        return res