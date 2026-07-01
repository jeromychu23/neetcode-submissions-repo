class Solution:
    def countComponents(self, n: int, edges: List[List[int]]) -> int:
        mp = defaultdict(list)
        for n1, n2 in edges:
            mp[n1].append(n2)
            mp[n2].append(n1)
        
        visit = set()
        def dfs(node):

            for nei in mp[node]:
                if nei not in visit:
                    visit.add(nei)
                    dfs(nei)
        
        res = 0
        for node in range(n):
            if node not in visit:
                dfs(node)
                res += 1
        
        return res