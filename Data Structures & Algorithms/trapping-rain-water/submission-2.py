class Solution:
    def trap(self, height: List[int]) -> int:
        # 因為這樣根本不可能蓄水
        if len(height) < 3:
            return 0

        highest_wall_idx = max(range(len(height)), key=lambda i: height[i])
       
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