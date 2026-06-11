class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        rows, cols = len(grid), len(grid[0])
        max_area = 0
        cur_area = 0

        def dfs(r, c):

            nonlocal cur_area
            if (r < 0 or c < 0 or r >= rows or c >= cols or grid[r][c] == 0):
                return
            
            grid[r][c] = 0
            cur_area += 1

            dfs(r + 1, c)
            dfs(r - 1, c)
            dfs(r, c + 1)
            dfs(r, c - 1)

        for r in range(rows):
            for c in range(cols):
                if grid[r][c] == 1:
                    cur_area = 0
                    dfs(r, c)
                    max_area = max(cur_area, max_area)
        
        return max_area