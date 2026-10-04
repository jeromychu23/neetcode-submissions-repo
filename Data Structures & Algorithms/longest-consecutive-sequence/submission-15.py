class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        hash_set = set(nums)
        max_len = 0;
        
        for n in nums:
            if n - 1 not in hash_set:
                cur_len = 1
                while n + cur_len in hash_set:
                    cur_len += 1
                
                max_len = max(max_len, cur_len)
        
        return max_len