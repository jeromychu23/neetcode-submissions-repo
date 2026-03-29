class Solution:
    def isPalindrome(self, s: str) -> bool:
        cleaned = re.sub(r'[^A-Za-z0-9]', '', s).lower()
        return cleaned == cleaned[::-1]
