class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        dp = [amount + 1] * (amount + 1)
        dp[0] = 0

        # dp array的index代表使用需要多少個目前的coins最小可以達到的數量
        # 例如：coins = [1, 3, 4, 5]
        # dp[0] = 0是因為沒有組合可以組合出amount = 0
        # dp[1] = 1因為可以用coin 1來達到最小組合
        # dp[2] = 2因為可以用兩個coin 1來達到最小組合 
        for a in range(1, amount + 1):
            for coin in coins:
                if a - coin >= 0:
                    dp[a] = min(dp[a], 1 + dp[a - coin])
        
        return dp[amount] if dp[amount] != amount + 1 else -1