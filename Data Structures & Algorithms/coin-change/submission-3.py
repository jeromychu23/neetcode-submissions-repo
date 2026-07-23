class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:

        q = collections.deque([(amount, 0)])
        seen = set()
        seen.add(amount)
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

