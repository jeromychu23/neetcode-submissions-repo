class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        L, R = 0, len(matrix) - 1
        while L <= R:
            M = (L + R) // 2
            if target < matrix[M][0]:
                R = M - 1
            elif matrix[M][-1] < target:
                L = M + 1
            else:
                matrix = matrix[M]
                break
        
        if L > R:
            return False
        
        l, r = 0, len(matrix) - 1
        while l <= r:
            m = (l + r) // 2
            if target < matrix[m]:
                r = m - 1
            elif matrix[m] < target:
                l = m + 1
            else:
                return True
        
        return False
