class Solution:
    def minOperations(self, logs: List[str]) -> int:
        res = 0
        for l in logs:
            if l == './':
                continue
            if l == '../':
                if res > 0:
                    res -= 1
            else:
                res += 1
        return res