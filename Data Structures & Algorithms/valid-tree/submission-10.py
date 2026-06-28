class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False

        hashmap = defaultdict(list)
        for start_n, end_n in edges:
            hashmap[start_n].append(end_n)
            hashmap[end_n].append(start_n)
        
        visited = set()
        q = deque([(0, -1)])
        visited.add(0)
        while q:
            cur, pre = q.popleft()
            for nei in hashmap[cur]:
                if pre == nei:
                    continue
                if nei in visited:
                    return False
                visited.add(nei)
                q.append((nei, cur))
        
        return len(visited) == n