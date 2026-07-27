class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        dp = [False] * (len(s) + 1)
        dp[0] = True

        for end in range(1, len(s) + 1):
            dp[end] = any(
                len(word) <= end
                and dp[end - len(word)]
                and s[end - len(word):end] == word
                for word in wordDict
            )

        return dp[-1]