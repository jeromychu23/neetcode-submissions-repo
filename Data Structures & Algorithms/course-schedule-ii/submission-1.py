class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        hashmap = defaultdict(list)
        count = [0] * numCourses
        for target, indegree in prerequisites:
            count[target] += 1
            hashmap[indegree].append(target)
        
        q = deque()
        for c in range(numCourses):
            if count[c] == 0:
                q.append(c)
        
        res = []
        while q:
            cur = q.popleft()
            res.append(cur)
            for course in hashmap[cur]:
                count[course] -= 1
                if count[course] == 0:
                    q.append(course)
        
        if all(i == 0 for i in count):
            return res
        else:
            return []