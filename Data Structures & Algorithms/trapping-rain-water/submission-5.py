class Solution:
    def trap(self, height: List[int]) -> int:
        peak = max(range(len(height)), key=height.__getitem__)

        res = 0

        leftM = 0
        
        for l in range(peak):
            leftM = max(height[l], leftM)
            res += leftM - height[l]
        
        rightM = 0
        for r in range(len(height) - 1, peak, -1):
            rightM = max(height[r], rightM)
            res += rightM - height[r]
            
        return res