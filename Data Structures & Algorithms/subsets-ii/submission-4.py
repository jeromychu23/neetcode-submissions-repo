class Solution:
    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()

        def dfs(cur, start):
            res.append(cur[:])

            for i in range(start, len(nums)):
                if i > start and nums[i] == nums[i - 1]:
                    continue
                
                cur.append(nums[i])
                dfs(cur, i + 1)
                cur.pop()
        
        dfs([], 0)
        return res