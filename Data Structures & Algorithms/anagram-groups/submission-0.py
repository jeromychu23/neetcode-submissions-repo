class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        # 用default比面dict裡面沒有key會導致error
        from collections import defaultdict
        # 創建value是list的dict
        res = defaultdict(list)

        for s in strs:
            # create hash map
            count = [0] * 26
            for c in s:
                count[ord(c) - ord('a')] += 1
            # 把count當作key，把s append到value，因為value在最一開始已經指定是list，所以可以用append
            # 因為list不能當作key，所以要轉為tuple
            res[tuple(count)].append(s)
        # 把所有value都放到list裡面
        return list(res.values())



        