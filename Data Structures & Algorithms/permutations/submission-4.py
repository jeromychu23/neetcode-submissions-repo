class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        res = []
        used = [False] * len(nums)

        def dfs(cur_list):
            if len(cur_list) == len(nums):
                res.append(cur_list[:])
                return
            
            for i in range(len(nums)):
                if used[i]:
                    continue
                
                used[i] = True
                cur_list.append(nums[i])

                dfs(cur_list)
                
                cur_list.pop()
                used[i] = False
        
        dfs([])
        return res