class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        for t in tokens:
            if t == "+":
                val1, val2 = stack.pop(-2), stack.pop(-1)
                stack.append(val1 + val2)
            elif t == "*":
                val1, val2 = stack.pop(-2), stack.pop(-1)
                stack.append(val1 * val2)
            elif t == "-":
                val1, val2 = stack.pop(-2), stack.pop(-1)
                stack.append(val1 - val2)
            elif t == "/":
                val1, val2 = stack.pop(-2), stack.pop(-1)
                val = int(val1 / val2)
                stack.append(math.floor(val))
            else:
                stack.append(int(t))
        return stack[-1]