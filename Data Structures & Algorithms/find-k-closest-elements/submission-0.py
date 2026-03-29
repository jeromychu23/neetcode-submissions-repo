class Solution:
    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:
        mp = defaultdict(list)
        for i in range(len(arr)):
            distance = abs(arr[i] - x)
            mp[distance].append(arr[i])
        mp = sorted(mp.items())
        res = []
        for i, val in mp:
            for j in val:
                res.append(j)
                if len(res) == k:
                    return sorted(res)