class Solution:

    def encode(self, strs: List[str]) -> str:
        encode_str = ""
        for s in strs:
            encode_str += str(len(s)) + "#" + s
        return encode_str

    def decode(self, s: str) -> List[str]:
        res = []
        # i表示數字位置
        i = 0
        while i < len(s):
            # j表示"#"位置
            j = i
            while s[j] != "#":
                j += 1
            length = int(s[i:j])
            # j + 1是字的開頭
            res.append(s[j + 1: j + 1 + length])
            i = j + length + 1
        return res