class Solution:

    def encode(self, strs: List[str]) -> str:
        res = ""
        for s in strs:
            res += str(len(s)) + "#" + s
        return res

    def decode(self, s: str) -> List[str]:
        res = []
        l = 0
        r = 0
        while l < len(s):
            while s[r] != "#":
                r += 1
            length = int(s[l:r])
            res.append(s[r + 1: r + 1 + length])
            l = r + length + 1
            r = l
        return res
             