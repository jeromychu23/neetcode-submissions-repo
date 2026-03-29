class Solution:
    def replaceElements(self, arr: List[int]) -> List[int]:
        n = len(arr)
        nums = [0] * n
        for i in range(n):
            if i == n - 1:
                nums[i] = -1
            else:
                nums[i] = max(arr[i+1:n])
        
        return nums
