class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        hashmap = {
            "2": ["a", "b", "c"],
            "3": ["d", "e", "f"],
            "4": ["g", "h", "i"],
            "5": ["j", "k", "l"],
            "6": ["m", "n", "o"],
            "7": ["p", "q", "r", "s"],
            "8": ["t", "u", "v"],
            "9": ["w", "x", "y", "z"]
        }
        if len(digits) == 0: return []

        res = []

        def dfs(cur, i):
            if len(cur) == len(digits):
                res.append("".join(cur[:]))
                return

            for j in range(i, len(digits)):
                j_list = hashmap[digits[j]]
                for a in j_list:
                    cur.append(a)
                    dfs(cur, j + 1)
                    cur.pop()

        dfs([], 0)
        return res

