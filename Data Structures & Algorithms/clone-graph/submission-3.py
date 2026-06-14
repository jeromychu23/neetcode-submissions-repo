"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if not node:
            return None

        visited = {}
        visited[node] = Node(node.val)
        q = deque([node])

        while q:
            cur_node = q.popleft()
            for cnn in cur_node.neighbors:
                if cnn not in visited:
                    visited[cnn] = Node(cnn.val)
                    q.append(cnn)
                visited[cur_node].neighbors.append(visited[cnn])
        
        return visited[node]