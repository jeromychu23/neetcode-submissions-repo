class Solution:
    def trap(self, height: List[int]) -> int:
        highest_wall = 0
        highest_wall_idx = 0
        for i, w in enumerate(height):
            if w > highest_wall:
                highest_wall = w
                highest_wall_idx = i

        l = 0
        r = 0
        res = 0
        for leftWall in range(highest_wall_idx):
            if l - height[leftWall] <= 0:
                res += 0
            else:
                res += l - height[leftWall]
            l = max(l, height[leftWall])

        for rightWall in range(len(height) - 1, highest_wall_idx, -1):
            if r - height[rightWall] <= 0:
                res += 0
            else:
                res += r - height[rightWall]
            r = max(r, height[rightWall])
        return res