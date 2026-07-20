class Solution:
    def numDecodings(self, s: str) -> int:
        if not s or s[0] == "0":
            return 0
        
        dp_minus_1 = 1
        dp_minus_2 = 1

        for i in range(2, len(s) + 1):
            dp_i = 0

            if s[i - 1] != "0":
                dp_i += dp_minus_1
            
            if 10 <= int(s[i-2:i]) <= 26:
                dp_i += dp_minus_2
            
            dp_minus_2 = dp_minus_1
            dp_minus_1 = dp_i
        
        return dp_minus_1
