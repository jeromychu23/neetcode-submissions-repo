class Solution:
    def trap(self, height: List[int]) -> int:
        highest_wall_idx = max(range(len(height)), key=height.__getitem__)
       
        res = 0

        leftMax = 0
        for i in range(highest_wall_idx):
            leftMax = max(leftMax, height[i])
            res += leftMax - height[i]

        rightMax = 0
        for i in range(len(height) - 1, highest_wall_idx, -1):
            rightMax = max(rightMax, height[i])
            res += rightMax - height[i]

        return res