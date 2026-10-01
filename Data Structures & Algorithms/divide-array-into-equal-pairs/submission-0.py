class Solution:
    def divideArray(self, nums: List[int]) -> bool:
        count = {}

        for n in nums:
            if n not in count:
                count[n] = 0
            count[n] += 1

        for c in count.values():
            if c % 2 == 1:
                return False

        return True   