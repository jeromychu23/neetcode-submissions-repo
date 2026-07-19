class Solution:
    def longestPalindrome(self, s: str) -> str:
        res = ''
        def expand(l, r):
            while l >= 0 and r < len(s) and s[l] == s[r]:
                l -= 1
                r += 1
            
            return l + 1, r - 1
        
        for i in range(len(s)):
            l, r = expand(i, i)
            if r - l + 1 > len(res):
                res = s[l:r + 1]
            
            l, r = expand(i, i + 1)
            if r - l + 1 > len(res):
                res = s[l:r + 1]
        
        return res

