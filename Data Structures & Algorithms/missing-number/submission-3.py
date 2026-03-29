class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        orig = sum(nums)
        expect = 0
        for i in range(n+1):
            expect += i
        return expect - orig