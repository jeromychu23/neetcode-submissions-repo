# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root1:
            return root2
        if not root2:
            return root1
        
        q = collections.deque()
        q.append((root1, root2))

        while q:
            n1, n2 = q.popleft()

            n1.val += n2.val

            if n1.left and n2.left:
                q.append((n1.left, n2.left))
            elif not n1.left:
                n1.left = n2.left
            
            if n1.right and n2.right:
                q.append((n1.right, n2.right))
            elif not n1.right:
                n1.right = n2.right
        
        return root1