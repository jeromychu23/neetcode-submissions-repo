class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        mp = {}
        res = float('inf')
        for i, n in enumerate(nums):
            if n in mp:
                res = min(abs(mp[n] - i), res)
            mp[n] = i
        return res <= k