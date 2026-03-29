class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        # no future day to sell
        if len(prices) <= 1:
            return 0
        
        profit = 0
        l, r = 0, 1
        while r < len(prices):
            if prices[l] < prices[r]: #Sell
                current_profit = prices[r] - prices[l]
                profit = max(profit, current_profit)
            else:
                l = r
            r += 1
        return profit
        