class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        mp = defaultdict(list)
        visited = set()
        for n1, n2 in edges:
            mp[n1].append(n2)
            mp[n2].append(n1)

        def dfs(node, visited):
            for nei in mp[node]:
                if nei not in visited:
                    visited.add(nei)
                    dfs(nei, visited)

        res = 0
        for n in range(n):
            if n not in visited:
                visited.add(n)
                dfs(n, visited)
                res += 1
        
        return res