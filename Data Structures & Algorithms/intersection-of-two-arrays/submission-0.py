class Solution:
    def intersection(self, nums1: List[int], nums2: List[int]) -> List[int]:
        res = set()
        nums1 = set(nums1)
        nums2 = set(nums2)

        if len(nums1) > len(nums2):
            for n in nums2:
                if n in nums1:
                    res.add(n)
            return list(res)
        else:
            for n in nums1:
                if n in nums2:
                    res.add(n)
            return list(res)