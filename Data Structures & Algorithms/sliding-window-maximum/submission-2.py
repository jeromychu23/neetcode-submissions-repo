from collections import deque

class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:

        dq = deque()
        res = []
        # 設定左跟右指針
        l = r = 0

        for idx, value in enumerate(nums):
            # 判斷dq是否有值以及從dq最右邊開始比大小，若當下的value比較大，就pop
            while dq and value > nums[dq[-1]]:
                dq.pop()
            # append比完大小的idx (dq只存idx不存value)
            dq.append(idx)
            # 判斷最大值的idx是否超出window(左指針)，超出的話就要popleft
            if dq[0] < l:
                dq.popleft()
            # 判斷窗口是否成立，若成立，就append最大值dq[0]
            if (r + 1) >= k:
                res.append(nums[dq[0]])
                l += 1
            r += 1
        
        return res