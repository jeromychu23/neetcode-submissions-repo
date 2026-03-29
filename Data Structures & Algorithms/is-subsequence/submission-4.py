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
            if s[sp] == t[tp]:
                res += t[tp]
                if res == s:
                    return True
                else:
                    tp += 1
                    sp += 1
            else:
                tp += 1
        return False
