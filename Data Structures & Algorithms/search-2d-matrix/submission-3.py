class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        new_matrix = list()
        L, R = 0, len(matrix) - 1
        while L <= R:
            M = (L + R) // 2
            if matrix[M][0] > target:
                    R = M - 1
            elif matrix[M][-1] < target:
                    L = M + 1
            elif matrix[M][0] <= target <= matrix[M][-1]:
                new_matrix = matrix[M]
                break

        if not (L <= R):
            return False
            
        l, r = 0, len(matrix[0]) - 1
        while l <= r:
            m = (l + r) // 2
            if new_matrix[m] > target:
                r = m - 1
            elif new_matrix[m] < target:
                l = m + 1
            else:
                return True
        
        return False