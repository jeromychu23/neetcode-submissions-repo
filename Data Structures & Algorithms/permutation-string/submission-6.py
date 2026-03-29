class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        n1, n2 = len(s1), len(s2)
        if n1 > n2:
            return False
        
        s1_count = [0] * 26
        s2_count = [0] * 26
        for i in range(n1):
            s1_count[ord(s1[i]) - ord('a')] += 1
            s2_count[ord(s2[i]) - ord('a')] += 1
        if s1_count == s2_count:
            return True
        
        for i in range(1, n2 - n1  + 1):
            count = [0] * 26
            for j in range(i, i + n1):
                count[ord(s2[j]) - ord('a')] += 1
            if s1_count == count:
                return True
        return False
        