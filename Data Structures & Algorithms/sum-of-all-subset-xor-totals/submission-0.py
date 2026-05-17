class Solution:
    def subsetXORSum(self, nums: List[int]) -> int:
        res = [[]]

        for num in nums:
            res += [subset + [num] for subset in res]

        total = 0

        for subset in res:
            xor_total = 0
            for n in subset:
                xor_total ^= n
            total += xor_total

        return total