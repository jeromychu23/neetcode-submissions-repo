class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = {}
        for _, n in enumerate(nums):
            count[n] = 1 + count.get(n, 0)
        
        bucket = [[] for _ in range(len(nums) + 1)]
        for n, freq in count.items():
            bucket[freq].append(n)
        
        res = []
        for i in range(len(bucket) - 1, 0, -1):
            for j in bucket[i]:
                res.append(j)
                if len(res) == k:
                    return res