class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        res = []
        # sub = []
        nums.sort()

        def dfs(sub, start):
            res.append(sub[::])
            
            for i in range(start, len(nums)):
                if i > start and nums[i] == nums[i - 1]:
                    continue
                
                sub.append(nums[i])
                dfs(sub, i + 1)
                sub.pop()
        
        dfs([], 0)
        return res

