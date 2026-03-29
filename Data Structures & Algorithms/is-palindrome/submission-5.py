class Solution:
    def isPalindrome(self, s: str) -> bool:
        clean = re.sub(r'[^A-Za-z0-9]', '', s).lower()
        return clean == clean[::-1]