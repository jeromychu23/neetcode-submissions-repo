# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def mergeTrees(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> Optional[TreeNode]:
        if root1 is None:
            return root2
        if root2 is None:
            return root1
        
        q = collections.deque()
        q.append((root1, root2))
        
        while q:
            n1, n2 = q.popleft()
            
            n1.val += n2.val
            
            # 左子節點
            if n1.left and n2.left:
                q.append((n1.left, n2.left))
            elif not n1.left:
                n1.left = n2.left
            
            # 右子節點
            if n1.right and n2.right:
                q.append((n1.right, n2.right))
            elif not n1.right:
                n1.right = n2.right
        
        return root1