class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        mp = [-1] * 128
        res = 0
        l = 0
        for r in range(len(s)):
            idx = ord(s[r])
            if mp[idx] >= 0:
                l = max(mp[idx] + 1, l)
            mp[idx] = r
            res = max(res, r - l + 1)
        return res

        # mp = {}

        # res = 0
        # l = 0
        # for r in range(len(s)):
        #     if s[r] in mp:
        #         l = max(mp[s[r]] + 1, l)
        #     mp[s[r]] = r
        #     res = max(res, r - l + 1)
        # return res