class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()
        res = []

        for i, n in enumerate(nums):
            # 表示後面都是正，所以相加不可能等於0，只接挑出迴圈
            if n > 0:
                break
            
            # i不是第一個，且當下的n等於上一個n，就直接下一個i，因為sort的關係，只要第一個n一樣，後面能組合的一定也一樣了
            if i > 0 and n == nums[i - 1]:
                continue
            
            l, r = i + 1, len(nums) - 1
            while l < r:
                threeSum = n + nums[l] + nums[r]
                if threeSum < 0:
                    l += 1
                elif threeSum > 0:
                    r -= 1
                else:
                    res.append([n, nums[l], nums[r]])
                    l += 1
                    while l < r and nums[l] == nums[l - 1]:
                        l += 1
        return res