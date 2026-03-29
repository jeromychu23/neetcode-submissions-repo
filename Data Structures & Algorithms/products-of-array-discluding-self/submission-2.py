class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [1] * len(nums)
        # res = [1, 1, 1, 1]
        prefix = 1 # 1 > 2 > 8 > 48
        for i in range(len(nums)):
            res[i] = prefix
            prefix *= nums[i]
        # pre_res = [1, 1, 2, 8]
        postfix = 1 # 6 > 24 > 48 > 48
        for i in reversed(range(len(nums))):
            res[i] *= postfix
            postfix *= nums[i]
        # res = [48, 24, 12, 8]
        return res