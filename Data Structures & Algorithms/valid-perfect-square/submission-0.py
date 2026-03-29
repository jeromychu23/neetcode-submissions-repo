class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        l, r = 0, num
        while l <= r:
            m = (l + r) // 2
            res = m * m
            if res == num:
                return True
            if res > num:
                r = m - 1
            else:
                l = m + 1
        return False