class Solution:
    def isValid(self, s: str) -> bool:
        res = []
        mp = {
            ")": "(",
            "}": "{",
            "]": "["
        }
        for i in s:
            if i in mp:
                if res and res[-1] == mp[i]:
                    res.pop()
                else:
                    return False
            else:
                res.append(i)
        
        return True if not res else False