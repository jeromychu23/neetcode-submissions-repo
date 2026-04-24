class Solution:
    def leastInterval(self, tasks: List[str], n: int) -> int:
        count = Counter(tasks)
        freq = list(count.values())
        maxf = max(freq)
        maxf_count = freq.count(maxf)

        ans = max(
            len(tasks),
            (maxf - 1) * (n + 1) + maxf_count
        )

        return ans


                
