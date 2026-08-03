class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        stack = []
        max_area = 0
        heights.append(0)

        for i, cur_height in enumerate(heights):
            cur_start = i

            while stack and cur_height < stack[-1][1]:
                start, prev_height = stack.pop()
                width = i - start
                max_area = max(max_area, prev_height * width)
                cur_start = start 

            stack.append((cur_start, cur_height))
        
        return max_area