class Solution:
    def longestPalindrome(self, s: str) -> str:
        res = s[0]
        for i in range(len(s)):
            r = len(s) - 1
            while i < r:
                sub_s = s[i:r+1]
                if self.isPalindrome(sub_s):
                    if len(sub_s) > len(res):
                        res = sub_s
                r -= 1
        return res    
    
    def isPalindrome(self, word):
        return word == word[::-1]
