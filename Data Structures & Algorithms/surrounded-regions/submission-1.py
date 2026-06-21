class Solution:
    def solve(self, board: List[List[str]]) -> None:
        rows, cols = len(board), len(board[0])
        directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]

        def dfs(r, c, visited):
            # 走出邊界，代表這條路沒有被包住
            if r < 0 or c < 0 or r >= rows or c >= cols:
                return 0

            # 遇到 X，代表這條路被擋住
            if board[r][c] == "X":
                return 1

            # 避免 O 之間來回走
            if (r, c) in visited:
                return 1

            visited.add((r, c))

            # 這裡不是固定直線走，而是讓它可以轉彎找路
            up = dfs(r - 1, c, visited)
            down = dfs(r + 1, c, visited)
            left = dfs(r, c - 1, visited)
            right = dfs(r, c + 1, visited)

            # 四個方向都被擋住，才代表這條路被包住
            if up + down + left + right == 4:
                return 1
            
            return 0

        to_flip = []

        for r in range(rows):
            for c in range(cols):
                if board[r][c] == "O":
                    count = 0

                    for dr, dc in directions:
                        visited = set()
                        visited.add((r, c))

                        count += dfs(r + dr, c + dc, visited)

                    if count == 4:
                        to_flip.append((r, c))

        for r, c in to_flip:
            board[r][c] = "X"