class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = [[]]

        for num in nums:
            subset = []
            for r in res:
                item = r + [num]
                subset.append(item)
            res += subset
        
        return res