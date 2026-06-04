class Solution:
    def letterCombinations(self, digits: str) -> List[str]:
        res = []
        hashmap = {
            "2": "abc",
            "3": "def",
            "4": "ghi",
            "5": "jkl",
            "6": "mno",
            "7": "pqrs",
            "8": "tuv",
            "9": "wxyz"
        }

        def dfs(cur_str, i):
            if len(cur_str) == len(digits):
                res.append(cur_str)
                return
            
            for c in hashmap[digits[i]]:
                dfs(cur_str + c, i + 1)
        
        if digits:
            dfs("", 0)
        
        return res