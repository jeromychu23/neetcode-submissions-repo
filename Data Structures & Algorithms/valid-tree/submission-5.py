class Solution:
    def validTree(self, n: int, edges: List[List[int]]) -> bool:
        if len(edges) != n - 1:
            return False

        hashmap = defaultdict(list)
        for start_node, end_node in edges:
            hashmap[start_node].append(end_node)
            hashmap[end_node].append(start_node)
        
        visited = set()
        def dfs(cur, pre):
            if cur in visited:
                return False
            
            visited.add(cur)
            for nei in hashmap[cur]:
                if nei == pre:
                    continue
                
                if not dfs(nei, cur):
                    return False
            
            return True
        
        return dfs(0, -1) and len(visited) == n