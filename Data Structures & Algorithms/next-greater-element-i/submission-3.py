class Solution:
    def nextGreaterElement(self, nums1: List[int], nums2: List[int]) -> List[int]:
        res = [-1] * len(nums1)
        mp = {num: i for i, num in enumerate(nums1)}
        
        stack = []
        for i in range(len(nums2)):
            current_val = nums2[i]
            while stack and current_val > stack[-1]:
                val = stack.pop()
                idx = mp[val]
                res[idx] = current_val
            if current_val in mp:
                stack.append(current_val)
        return res
        