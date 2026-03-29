class Solution:
    def isAnagram(self, s: str, t: str) -> bool:

        s_list = sorted([s1 for s1 in s])
        t_list = sorted([t1 for t1 in t])

        return (s_list == t_list)

        