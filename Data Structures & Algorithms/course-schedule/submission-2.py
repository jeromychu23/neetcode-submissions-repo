class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        hashmap = defaultdict(list)
        count = [0] * numCourses
        for target, indegree in prerequisites:
            count[target] += 1
            hashmap[indegree].append(target)
        
        q = deque()
        for c in range(numCourses):
            if count[c] == 0:
                q.append(c)
        
        while q:
            cur = q.popleft()
            for i in hashmap[cur]:
                count[i] -= 1
                if count[i] == 0:
                    q.append(i)
        
        return all(i == 0 for i in count)