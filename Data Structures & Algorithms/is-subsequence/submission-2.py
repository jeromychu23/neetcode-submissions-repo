class Solution:
    def isSubsequence(self, s: str, t: str) -> bool:
        if len(s) > len(t):
            return False
        
        if len(s) == 0:
            return True

        sp = 0
        tp = 0
        res = ""
        while tp < len(t):
            if s[sp] != t[tp]:
                tp += 1
            else:
                res += t[tp]
                tp += 1
                if sp + 1 < len(s):
                    sp += 1 
                else:
                    break
        return res == s
