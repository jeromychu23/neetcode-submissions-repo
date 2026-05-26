class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        res = []

        def dfs(cur_list, nums):
            if len(cur_list) == len(nums):
                res.append(cur_list[:])
                return
            
            for num in nums:
                if num in cur_list:
                    continue
                
                cur_list.append(num)
                dfs(cur_list, nums)
                cur_list.pop()
        
        dfs([], nums)
        return res