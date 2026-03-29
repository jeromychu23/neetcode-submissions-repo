class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        if len(s1) > len(s2):
            return False
        
        s1_count = [0] * 26
        for c in s1:
            s1_count[ord(c) - ord('a')] += 1
        
        w = len(s1)
        for i in range(len(s2) - w + 1):
            w_count = [0] * 26
            for c in s2[i:i+w]:
                w_count[ord(c) - ord('a')] += 1
            if w_count == s1_count:
                return True

        return False                
