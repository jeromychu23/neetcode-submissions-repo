# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if not root:
            return []

        res = collections.deque([])
        q = [root]

        while q:
            qLen = len(q)
            for _ in range(qLen):
                node = q.pop()
                if node:
                    res.appendleft(node.val)
                if node.left:
                    q.append(node.left)
                if node.right:
                    q.append(node.right)
        
        return list(res)
        
