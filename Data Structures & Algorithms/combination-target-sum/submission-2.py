class Solution:
    def combinationSum(self, nums: List[int], target: int) -> List[List[int]]:
        res = []
        nums.sort()

        def dfs(cur, total, start):
            if total == target:
                res.append(cur[:]) # copy.()的語法糖
                return
            
            for i in range(start, len(nums)):
                if i >= len(nums) or total > target:
                    return
                
                cur.append(nums[i])
                dfs(cur, total + nums[i], i)
                cur.pop()
            
        dfs([], 0, 0)
        return res