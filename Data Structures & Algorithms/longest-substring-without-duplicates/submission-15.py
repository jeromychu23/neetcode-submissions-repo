class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        mp = {}
        l = 0
        res = 0
        for r in range(len(s)):
            if s[r] in mp:
                # 若這輪的str存在於mp中，就要把左指針移動到原字串的下一個位置
                # 並且要跟當前的l比大小，取較大的
                l = max(mp[s[r]] + 1, l)
            # 將此輪的字串跟位置記錄到mp中
            mp[s[r]] = r
            # 最後計算長度且比大小
            res = max(res, r - l + 1)

        return res