class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        
        # maxP = 0
        # minBuy = prices[0]
        # for sell_price in prices:
        #     minBuy = min(sell_price, minBuy)
        #     maxP = max(sell_price - minBuy, maxP)
        # return maxP

        maxP = 0
        l = 0
        r = 1
        while r < len(prices):
            if prices[l] < prices[r]:
                current_profit = prices[r] - prices[l]
                maxP = max(maxP, current_profit)
            else:
                l = r
            r += 1
        return maxP
