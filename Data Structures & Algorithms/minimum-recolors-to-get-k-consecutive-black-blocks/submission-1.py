class Solution:
    def minimumRecolors(self, blocks: str, k: int) -> int:
        res = float('inf')
        l = 0
        count = defaultdict(int)
        for r, c in enumerate(blocks):
            count[c] = 1 + count.get(c, 0)
            if r - l + 1 == k:
                res = min(res, count["W"])
                count[blocks[l]] -= 1
                l += 1
        return res