class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        res = []
        for t in tokens:
            if t == "+":
                res.append(res.pop() + res.pop())
            elif t == "-":
                val2, val1 = res.pop(), res.pop()
                res.append(val1 - val2)
            elif t == "*":
                res.append(res.pop() * res.pop())
            elif t == "/":
                val2, val1 = res.pop(), res.pop()
                res.append(int(val1 / val2))
            else:
                res.append(int(t))
        return res[-1]