class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:

        # create hash map
        res = {}
        # 將target減去每個value，並將減去的值跟對應的idx新增到hash map
        for i, v in enumerate(nums):
            diff = target - v
            if diff in res:
                return [res[diff], i]
            res[v] = i