class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        mp = {}
        for i, p in enumerate(position):
            step = (target - p) / speed[i]
            mp[p] = step

        position.sort(reverse=True)
        stack = []
        for p in position:
            step = mp[p]
            stack.append(step)
            if len(stack) >= 2 and stack[-1] <= stack[-2]:
                stack.pop()
        return len(stack)