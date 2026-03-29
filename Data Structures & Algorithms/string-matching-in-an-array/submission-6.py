class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        # 用分隔符號拼成一個大字串
        combined = "#".join(words)
        res = []
        for w in words:
            # 如果 w 在大字串中出現的次數 > 1，表示它是某人的子字串
            # (因為它自己本身就在大字串裡佔了一次)
            if combined.count(w) > 1:
                res.append(w)
        return res