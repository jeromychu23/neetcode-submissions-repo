class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        res = []
        candidates.sort()

        def dfs(cur, total, start):
            if total == target:
                res.append(cur[:])
                return
            
            for i in range(start, len(candidates)):
                if i > start and candidates[i] == candidates[i - 1]:
                    continue
                
                if total + candidates[i] > target:
                    break
                
                cur.append(candidates[i])
                dfs(cur, total + candidates[i], i + 1)
                cur.pop()
        
        dfs([], 0, 0)
        return res