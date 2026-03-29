class Solution:
    def trap(self, height: List[int]) -> int:
        peak_idx = max(range(len(height)), key=lambda i: height[i])

        res = 0

        leftMax = 0
        for i in range(peak_idx):
            leftMax = max(leftMax, height[i])
            res += leftMax - height[i]
        
        rightMax = 0
        for i in range(len(height) - 1, peak_idx, -1):
            rightMax = max(rightMax, height[i])
            res += rightMax - height[i]
        
        return res