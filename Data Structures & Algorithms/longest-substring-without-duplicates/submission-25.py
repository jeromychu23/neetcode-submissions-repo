class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        seen = {}
        length = 0
        l = 0
        for r in range(len(s)):
            if s[r] in seen:
                l = max(seen[s[r]] + 1, l)
            seen[s[r]] = r
            length = max(r - l + 1, length)
        return length
                 
