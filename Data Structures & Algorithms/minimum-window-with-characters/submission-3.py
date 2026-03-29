class Solution:
    def minWindow(self, s: str, t: str) -> str:
        t_count, win_count = {}, {}
        for c in t:
            t_count[c] = 1 + t_count.get(c, 0)
            
        have, need = 0, len(t_count)
        len_res = float('inf')
        start = 0
        end = 0
        l = 0
        for r, c in enumerate(s):
            win_count[c] = win_count.get(c, 0) + 1

            if c in t_count and win_count[c] == t_count[c]:
                have += 1

            while have == need:
                if r - l + 1 < len_res:
                    len_res = r - l + 1
                    start, end = l, r

                win_count[s[l]] -= 1
                if s[l] in t_count and win_count[s[l]] < t_count[s[l]]:
                    have -= 1
                l += 1

        return s[start: end + 1] if len_res != float('inf') else ""
            