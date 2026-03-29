class Solution:
    def scoreOfString(self, s: str) -> int:
        res = 0
        l = 0
        r = 1
        while l < r < len(s):
            res += abs(ord(s[l]) - ord(s[r]))
            l += 1
            r += 1

        return res