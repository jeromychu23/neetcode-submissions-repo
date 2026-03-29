class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        
        maxP = 0
        minBuy = prices[0]
        for sell_price in prices:
            minBuy = min(sell_price, minBuy)
            maxP = max(sell_price - minBuy, maxP)
        return maxP