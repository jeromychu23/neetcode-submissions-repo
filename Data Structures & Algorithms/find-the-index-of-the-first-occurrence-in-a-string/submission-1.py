class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        n1, n2 = len(haystack), len(needle)
        if n1 < n2: return -1

        for c in range(n1):
            if haystack[c] == needle[0] and haystack[c:c+n2] == needle:
                return c
        return -1
