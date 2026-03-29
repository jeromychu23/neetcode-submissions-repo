class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        combined = "#".join(words)
        res = []
        for w in words:
            if combined.count(w) > 1:
                res.append(w)
        return res