class MinStack:

    def __init__(self):
        self.stack = []
        self.minStack = []

    def push(self, val: int) -> None:
        if self.minStack and val <= self.minStack[0]:
            self.minStack.insert(0, val)
        else:
            self.minStack.append(val)
        self.stack.append(val)

    def pop(self) -> None:
        val = self.stack.pop()
        self.minStack.remove(val)

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.minStack[0]
