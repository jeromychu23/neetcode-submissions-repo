# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        pq = deque([p])
        qq = deque([q])
        while pq and qq:
            for _ in range(len(pq)):
                p_node = pq.popleft()
                q_node = qq.popleft()
                if p_node is None and q_node is None:
                    continue
                if p_node is None or q_node is None or p_node.val != q_node.val:
                    return False
                pq.append(p_node.left)
                qq.append(q_node.left)
                pq.append(p_node.right)
                qq.append(q_node.right)
        return True
