class Solution:
    def findMin(self, nums: List[int]) -> int:
        l, r = 0, len(nums) - 1
        
        if nums[l] < nums[r]:
            return nums[l]

        while l <= r:
            m = (l + r) // 2
            if nums[m] > nums[r]:
                if nums[m] < nums[m + 1]:
                    l = m + 1
                else:
                    return nums[m + 1]
            elif nums[m] < nums[l]:
                if nums[m] > nums[m - 1]:
                    r = m - 1
                else:
                    return nums[m]
            elif l == r:
                return nums[m]
            