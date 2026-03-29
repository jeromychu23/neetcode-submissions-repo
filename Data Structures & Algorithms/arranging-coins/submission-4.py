class Solution:
    def arrangeCoins(self, n: int) -> int:
        l, r = 0, n
        while l <= r:
            mid = (l + r) // 2
            coins = (mid / 2) * (mid + 1)
            if coins == n:
                return mid
            if coins > n:
                r = mid - 1
            else:
                l = mid + 1
        return r
            