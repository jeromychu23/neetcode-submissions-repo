class Solution:
    def isAnagram(self, s: str, t: str) -> bool:

        # if len(s) != len(t):
        #     return False
        
        # count = [0] * 26
        # for i in range(len(s)):
        #         count[ord(s[i]) - ord('a')] += 1
        #         count[ord(t[i]) - ord('a')] -= 1
        
        # return all(x == 0 for x in count)

        s_list = sorted([s1 for s1 in s])
        t_list = sorted([t1 for t1 in t])
        
        return (s_list == t_list)
