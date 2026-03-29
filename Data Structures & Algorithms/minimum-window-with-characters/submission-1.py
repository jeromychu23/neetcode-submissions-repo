class Solution:
    def minWindow(self, s: str, t: str) -> str:
        s_len, t_len = len(s), len(t)
        
        t_count, window_count = {}, {}
        for i in t:
            t_count[i] = 1 + t_count.get(i, 0)
        
        need, have = len(t_count), 0

        len_res = float('inf')
        start = 0
        end = 0
        
        l = 0
        for r, c in enumerate(s):
            window_count[c] = 1 + window_count.get(c, 0)

            if c in t_count and window_count[c] == t_count[c]:
                have += 1
            
            while need == have:
                if r - l + 1 < len_res:
                    len_res = r - l + 1
                    start, end = l, r
                
                window_count[s[l]] -= 1
                if s[l] in t_count and window_count[s[l]] < t_count[s[l]]:
                    have -= 1
                
                l += 1

        return "" if len_res == float('inf') else s[start: end + 1]