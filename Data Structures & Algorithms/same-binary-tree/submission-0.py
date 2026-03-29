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
                pn = pq.popleft()
                qn = qq.popleft()
                
                if pn is None and qn is None:
                    continue

                if pn is None or qn is None or pn.val != qn.val:
                    return False

                pq.append(pn.left)
                qq.append(qn.left)
                pq.append(pn.right)
                qq.append(qn.right)
        return True