class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        directions = [[1, 0], [-1, 0], [0, 1], [0, -1]]
        rows, cols = len(grid), len(grid[0])
        max_area = 0

        def bfs(r, c):
            q = collections.deque()
            grid[r][c] = 0
            q.append((r, c))
            res = 1

            while q:
                r, c = q.popleft()
                for dr, dc in directions:
                    nr, nc = r + dr, c + dc
                    if (nr < 0 or nc < 0 or nr >= rows or nc >= cols or grid[nr][nc] == 0):
                        continue
                    
                    res += 1
                    q.append((nr, nc))
                    grid[nr][nc] = 0
            
            return res

        for r in range(rows):
            for c in range(cols):
                if grid[r][c] == 1:
                    max_area = max(bfs(r, c), max_area)
        
        return max_area