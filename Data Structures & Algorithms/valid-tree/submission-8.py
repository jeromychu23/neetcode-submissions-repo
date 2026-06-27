class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False

        hashmap = defaultdict(list)
        for start_node, end_node in edges:
            hashmap[start_node].append(end_node)
            hashmap[end_node].append(start_node)
        
        visited = set()
        q = deque([(0, -1)])
        visited.add(0)
        while q:
            cur, pre = q.popleft()
            for nei in hashmap[cur]:
                if nei == pre:
                    continue
                if nei in visited:
                    return False
                visited.add(nei)
                q.append((nei, cur))

        return len(visited) == n