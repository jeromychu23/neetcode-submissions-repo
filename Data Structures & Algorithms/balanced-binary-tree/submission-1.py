# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        res = []

        def dfs(root):
            nonlocal res

            if not root:
                return 0
            
            left = dfs(root.left)
            right = dfs(root.right)
            res.append(abs(left - right))
            return 1 + max(left, right)
        dfs(root)
        print(res)
        return all(x <= 1 for x in res)

