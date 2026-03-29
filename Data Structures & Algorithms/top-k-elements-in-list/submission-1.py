class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        group = defaultdict(int)

        for i in nums:
            group[i] += 1

        count = [[] for _ in range(len(nums) + 1)]

        for num, freq in group.items():
            count[freq].append(num)

        res = []

        for n in reversed(range(1, len(count))):
            for l in count[n]:
                res.append(l)
                if len(res) == k:
                    return res