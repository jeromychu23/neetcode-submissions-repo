class Solution:
    def numSubarrayProductLessThanK(self, nums: List[int], k: int) -> int:
        l, product, count = 0, 1, 0
        for r in range(len(nums)):
            product *= nums[r]
            while l <= r and product >= k:
                product /= nums[l]
                l += 1
            count += r - l + 1
        return count