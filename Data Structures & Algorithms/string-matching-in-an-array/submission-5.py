class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        res =set()
        for word in words:
            for i in range(0,len(words)):
                if words[i] in word and words[i] != word:
                    res.add(words[i])

        return list(res)