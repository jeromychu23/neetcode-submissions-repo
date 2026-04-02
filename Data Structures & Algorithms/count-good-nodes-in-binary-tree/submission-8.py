# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        stack = [(root, float('-inf'))]
        res = 0

        while stack:
            node, maxVal = stack.pop()
            if node.val >= maxVal:
                res += 1
            newMax = max(maxVal, node.val)
            if node.left:
                stack.append((node.left, newMax))
            if node.right:
                stack.append((node.right, newMax))
        
        return res