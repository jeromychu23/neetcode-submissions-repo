class Solution:
    def dailyTemperatures(self, temperatures: List[int]) -> List[int]:
        mp = {}
        for i, t in enumerate(temperatures):
            mp[t] = i

        res = [0] * len(temperatures)
        stack = []
        for i, t in enumerate(temperatures):
            l = i
            while stack and l < len(temperatures):
                future_temp = temperatures[l]
                if future_temp > stack[-1]:
                    idx = i - 1
                    days = l - idx
                    res[idx] = days
                    # stack.pop()
                    break
                l += 1
            stack.append(t)
        return res
