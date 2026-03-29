class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if len(s) == 0:
            return 0
        longest = 1
        l, r = 0, 1
        while r < len(s):
            if s[r] not in set(s[l:r]):
                current_len = len(s[l:r+1])
                r += 1
            else:
                current_len = len(s[l:r])
                l += 1
            longest = max(longest, current_len)
        return longest