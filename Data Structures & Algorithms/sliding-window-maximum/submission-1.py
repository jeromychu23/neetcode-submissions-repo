from collections import deque

class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:

        dq = deque()
        res = []

        for idx, value in enumerate(nums):
            # 判斷dq是否有值以及從dq最右邊開始比大小，若當下的value比較大，就pop
            while dq and value >= nums[dq[-1]]:
                dq.pop()
            # append比完大小的idx (dq只存idx不存value)
            dq.append(idx)
            # 判斷最大值的idx是否超出window，超出的話就要popleft
            if dq[0] <= idx - k:
                dq.popleft()
            # 判斷窗口是否成立，若成立，就append最大值dq[0]
            if idx >= k -1:
                res.append(nums[dq[0]])
        
        return res