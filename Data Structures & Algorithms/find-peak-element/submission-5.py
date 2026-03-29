class Solution:
    def findPeakElement(self, nums: List[int]) -> int:
        l, r = 0, len(nums) - 1
        if len(nums) == 1: return 0
        if nums[l] > nums[l + 1]: return l
        if nums[r] > nums[r - 1]: return r
        while l <= r:
            m = (l + r) // 2
            if nums[m - 1] < nums[m] and nums[m] > nums[m + 1]:
                return m
            if nums[m - 1] > nums[m]:
                r = m
            elif nums[m + 1] > nums[m]:
                l = m
        # return m