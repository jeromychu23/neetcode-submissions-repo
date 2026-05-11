class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        res = [[]]

        for num in nums:
            cur_sub = []
            for r in res:
                item = r + [num]
                cur_sub.append(item)
            res += cur_sub
        
        return res