class Solution:
    def singleNonDuplicate(self, nums: List[int]) -> int:
        # if len(nums) == 1: return nums[0]

        l, r = 0, len(nums) - 1
        # while l <= r:
        #     m = (l + r) // 2
        #     if l == r == m:
        #         return nums[m]
        #     if m%2 == 0:
        #         if nums[m] == nums[m + 1]:
        #             l = m
        #         else:
        #             r = m
        #     else:
        #         if nums[m] == nums[m + 1]:
        #             r = m - 1
        #         else:
        #             l = m + 1
        while l < r:
            m = (l + r) // 2
            if m % 2 != 0:
                m = m - 1
            if nums[m] == nums[m + 1]:
                l = m + 2
            else:
                r = m
        return nums[l]
