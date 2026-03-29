class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:

        dq = collections.deque()
        output = []
        left_point = right_point = 0

        for i, v in enumerate(nums):
            while dq and v > nums[dq[-1]]:
                dq.pop()
            dq.append(i)

            if dq[0] < left_point:
                dq.popleft()
            
            if i + 1 >= k:
                output.append(nums[dq[0]])
                left_point += 1
            right_point += 1
        
        return output
