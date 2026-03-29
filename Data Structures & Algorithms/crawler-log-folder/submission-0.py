class Solution:
    def minOperations(self, logs: List[str]) -> int:
        res = 0
        for l in logs:
            if l == '../':
                if res > 0:
                    res -= 1
            elif l == './':
                res += 0
            else:
                res += 1
        return res