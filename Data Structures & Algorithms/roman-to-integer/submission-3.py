class Solution:
    def romanToInt(self, s: str) -> int:
        mp = {
            "M": 1000,
            "D": 500,
            "C": 100,
            "L": 50,
            "X": 10,
            "V": 5,
            "I": 1
        }
        res = 0
        for i, c in enumerate(s):
            if i < len(s) - 1 and mp[s[i]] < mp[s[i + 1]]:
                res -= mp[c]
            else:
                res += mp[c]
        return res