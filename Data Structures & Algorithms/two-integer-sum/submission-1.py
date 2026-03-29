class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:

        # val : idx
        hashMap = {}

        # 用target減去每一個idx的value，將結果比對hash map裡面是否有符合的值
        for i, v in enumerate(nums):
            diff = target - v
            if diff in hashMap:
                return [hashMap[diff], i]
            hashMap[v] = i
