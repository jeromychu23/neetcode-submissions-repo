class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        if amount == 0: return 0

        q = collections.deque()
        for coin in coins:
            cur_amount = amount - coin
            if cur_amount >= 0:
                q.append((cur_amount, 1))

        seen = set()
        while q:
            amount, count = q.popleft()
            if amount == 0:
                return count
            for c in coins:
                amount_left = amount - c
                if amount_left >= 0 and amount_left not in seen:
                    seen.add(amount_left)
                    q.append((amount_left, count + 1))
            
        return -1

