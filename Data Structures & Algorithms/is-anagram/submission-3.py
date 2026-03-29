class Solution:
    def isAnagram(self, s: str, t: str) -> bool:

        # if len(s) != len(t):
        #     return False
        
        # count = [0] * 26
        # for i in range(len(s)):
        #         count[ord(s[i]) - ord('a')] += 1
        #         count[ord(t[i]) - ord('a')] -= 1
        
        # return all(x == 0 for x in count)

        if len(s) != len(t):
            return False
        
        s_list = sorted([x for x in s])
        t_list = sorted([x for x in t])

        if s_list == t_list:
            return True
        else:
            return False
