class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:

        array_len = len(nums)
        distinct_len = len(set(nums))

        if array_len == distinct_len:
            return False
        else:
            return True
        