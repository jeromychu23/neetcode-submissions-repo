class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        window_count = {} # A: 1
        longest = 0 # 1
        l = 0
        for r in range(len(s)):
            window_count[s[r]] = 1 + window_count.get(s[r], 0)
            while (r - l + 1) - max(window_count.values()) > k:
                window_count[s[l]] -= 1
                l += 1
            longest = max(longest, r - l + 1)
        return longest