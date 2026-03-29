class Solution:
    def characterReplacement(self, s: str, k: int) -> int:
        window_count = {}
        longest = 0
        l = 0
        maxf = 0
        for r in range(len(s)):
            window_count[s[r]] = 1 + window_count.get(s[r], 0)
            maxf = max(maxf, window_count[s[r]])
            while r - l + 1 - maxf > k:
                window_count[s[l]] -= 1
                l += 1
            longest = max(longest, r - l + 1)
        return longest