# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        count = 0

        def dfs(node, maxVal = float('-inf')):
            if not node:
                return 0
            nonlocal count
            if node.val >= maxVal:
                count += 1
                maxVal = node.val
            dfs(node.left, maxVal)
            dfs(node.right, maxVal)
        
        dfs(root)
        return count