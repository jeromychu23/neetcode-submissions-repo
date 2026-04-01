# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        maxNode = float('-inf')
        res = 0

        def dfs(node, maxNode):
            nonlocal res
            if not node:
                return None
            if node:
                if node.val >= maxNode:
                    res += 1
            newMax = max(maxNode, node.val)
            dfs(node.left, newMax)
            dfs(node.right, newMax)
        
        dfs(root, maxNode)
        return res
