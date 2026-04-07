class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        minBuy = prices[0]
        for p in prices:
            minBuy = min(minBuy, p)
            profit = max((p - minBuy), profit)
        return profit