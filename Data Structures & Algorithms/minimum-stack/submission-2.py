class MinStack:

    def __init__(self):
        self.stack = []
        self.dq = collections.deque()

    def push(self, val: int) -> None:
        if self.dq and val <= self.dq[0]:
            self.dq.insert(0, val)
        else:
            self.dq.append(val)
        self.stack.append(val)

    def pop(self) -> None:
        val = self.stack.pop()
        self.dq.remove(val)

    def top(self) -> int:
        return self.stack[-1]

    def getMin(self) -> int:
        return self.dq[0]

