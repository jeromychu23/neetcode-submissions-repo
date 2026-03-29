class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        res= 0
        for i, n in enumerate(nums):
            if n != val:
                nums[res] = nums[i]
                res += 1
        return res