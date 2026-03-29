class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [0] * len(nums)
        hmap = {}
        zero_count = 0
        p = 1

        for i, n in enumerate(nums):
            hmap[n] = i
            while n == 0:
                n = 1
                zero_count += 1
            p *= n

        if zero_count >= 2:
            return res
        if zero_count == 1:
            res[hmap[0]] = p
            return res
        else:  
            for i in range(len(nums)):
                res[i] = int(p / nums[i])
            return res    