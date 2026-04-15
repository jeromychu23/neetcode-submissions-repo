class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        while len(stones) > 1:
            heapq.heapify_max(stones)
            s1 = heapq.heappop_max(stones)
            s2 = heapq.heappop_max(stones)
            new_s = abs(s1 - s2) if s1 != s2 else 0
            if new_s != 0:
                heapq.heappush(stones, new_s)
        
        return 0 if len(stones) == 0 else stones[0]