class Solution:
    def maxArea(self, heights: List[int]) -> int:
        max_area = 0
        l = 0
        r = len(heights) - 1
        while l < r:
            
            if heights[l] < heights[r] and l < r:
                current_area = (r - l) * heights[l]
                l += 1
            elif heights[l] > heights[r] and l < r:
                current_area = (r - l) * heights[r]
                r -= 1
            else:
                current_area = (r - l) * heights[l]
                l += 1
            max_area = max(max_area, current_area)
        return max_area