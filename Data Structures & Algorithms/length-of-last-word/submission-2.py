class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        r, length = len(s) - 1, 0
        while s[r] == ' ':
            r -= 1
        
        while r >= 0 and s[r] != ' ':
            length += 1
            r -= 1
        return length
