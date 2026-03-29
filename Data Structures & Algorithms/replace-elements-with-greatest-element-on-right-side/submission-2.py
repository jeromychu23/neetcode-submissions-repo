class Solution:
    def replaceElements(self, arr: List[int]) -> List[int]:
        n = len(arr)
        nums = [0] * n
        rightMax = -1
        for i in reversed(range(n)):
            nums[i] = rightMax
            rightMax = max(rightMax, arr[i])
        return nums
