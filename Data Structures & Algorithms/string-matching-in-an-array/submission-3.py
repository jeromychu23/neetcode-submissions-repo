class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        res = []
        n = len(words)
        for i in range(n):
            w = words[0]
            for j in range(1, n):
                if w in words[j]:
                    res.append(w)
            words.remove(w)
            words.append(w)
        return list(set(res))