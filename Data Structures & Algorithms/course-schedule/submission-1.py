class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        mp = defaultdict(list)
        count = [0] * numCourses
        for src, indegree in prerequisites:
            count[src] += 1
            mp[indegree].append(src)
        
        q = deque()
        for c in range(numCourses):
            if count[c] == 0:
                q.append(c)
        
        while q:
            cur = q.popleft()
            for i in mp[cur]:
                count[i] -= 1
                if count[i] == 0:
                    q.append(i)
        
        return all(i == 0 for i in count)